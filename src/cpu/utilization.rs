use super::cpu_times::CpuTimes;
use super::parse_error::ParseError;
use std::fs;

pub struct Utilization {
    cputimes: CpuTimes,
    prev_cputimes: CpuTimes,
}

impl Utilization {
    pub fn new() -> Result<Self, ParseError> {
        Ok(Utilization {
            cputimes: Self::read_cpu_times()?,
            prev_cputimes: Self::read_cpu_times()?,
        })
    }

    pub fn update(&mut self) -> Result<(), ParseError> {
        self.prev_cputimes = std::mem::replace(&mut self.cputimes, Self::read_cpu_times()?);
        Ok(())
    }

    pub fn get_average_load(&self) -> Result<f32, ParseError> {
        let cpu_count = self.cputimes.len();
        let mut total = 0.0;
        for i in 0..cpu_count {
            total += self.get_core_load(i)?;
        }
        Ok(total / cpu_count as f32)
    }

    fn get_core_load(&self, core: usize) -> Result<f32, ParseError> {
        let last = &self.cputimes.get(core).ok_or(IndexError)?;
        let prev = &self.prev_cputimes.get(core).ok_or(IndexError)?;

        let idle_diff = last.idle.abs_diff(prev.idle);
        let diff = last.get_total().abs_diff(prev.get_total());

        if diff == 0 {
            return Ok(0.0);
        }

        let percentage = (100 * (diff - idle_diff)) as f32 / diff as f32;
        Ok(percentage.clamp(0.0, 100.0))
    }

    fn read_cpu_times() -> Result<CpuTimes, ParseError> {
        fs::read_to_string("/proc/stat")
            .map_err(|e| ParseError::Generic(e.to_string()))?
            .parse()
    }

    pub fn iter(&self) -> UtilizationIterator {
        UtilizationIterator::new(self)
    }
}

pub struct UtilizationIterator<'a> {
    utilization: &'a Utilization,
    current_index: usize,
}

impl<'a> UtilizationIterator<'a> {
    pub fn new(utilization: &'a Utilization) -> Self {
        UtilizationIterator {
            utilization,
            current_index: 0,
        }
    }
}

impl<'a> Iterator for UtilizationIterator<'a> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.utilization.cputimes.len() {
            let load = self.utilization.get_core_load(self.current_index).unwrap();
            self.current_index += 1;
            Some(load)
        } else {
            None
        }
    }
}

pub struct IndexError;

impl From<IndexError> for ParseError {
    fn from(_: IndexError) -> Self {
        ParseError::Generic("Out of bounds error".to_string())
    }
}
