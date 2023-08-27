use super::cpu_time::CpuTime;
use std::io;
use std::ops::Deref;
use std::str::FromStr;

pub struct CpuTimes(pub Vec<CpuTime>);

impl Deref for CpuTimes {
    type Target = Vec<CpuTime>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for CpuTimes {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CpuTimes(
            s.lines()
                .filter(|s| {
                    if s.starts_with("cpu ") {
                        return false;
                    }
                    s.starts_with("cpu")
                })
                .map(|s| s.parse())
                .collect::<io::Result<Vec<CpuTime>>>()?,
        ))
    }
}
