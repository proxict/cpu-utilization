use super::cpu_time::CpuTime;
use super::parse_error::ParseError;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::ops::Deref;
use std::str::FromStr;

#[derive(PartialEq)]
pub struct CpuTimes(pub Vec<CpuTime>);

impl Deref for CpuTimes {
    type Target = Vec<CpuTime>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for CpuTimes {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cpu_times = s
            .lines()
            .filter(|s| !s.starts_with("cpu ") && s.starts_with("cpu"))
            .map(|s| s.parse())
            .collect::<Result<Vec<CpuTime>, ParseError>>()?;
        match cpu_times.is_empty() {
            false => Ok(CpuTimes(cpu_times)),
            true => Err(ParseError::NoCpus),
        }
    }
}

impl Debug for CpuTimes {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.is_empty() {
            false => {
                for (idx, cpu_time) in self.iter().enumerate() {
                    writeln!(f, "cpu{idx}[{:?}]", cpu_time)?;
                }
                Ok(())
            }
            true => {
                write!(f, "No CPUs")
            }
        }
    }
}
