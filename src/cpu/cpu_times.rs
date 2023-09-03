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

#[cfg(test)]
mod tests {
    #[test]
    fn cputimes_ok() {
        let proc_stat = r#"cpu  488167 545 209081 9891865 4452 16220 9200 0 0 0
cpu0 61909 15 25333 1235463 645 2152 1663 0 0 0
cpu1 61740 11 26824 1235295 508 2713 967 0 0 0
intr 19685455 14 39586 0 0 0 0 0 0 0 0 0 0 0 0 0 0 136135 309 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1655 273923 0 5900 0 0 0 0 0 0 0 45 482728 2225 139151 294394 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
ctxt 72244753
btime 1693740935
processes 591694
procs_running 1
procs_blocked 0
softirq 14426183 279419 902463 47 605534 136458 0 39890 6692607 1806 5767959
"#;
        let cpu_times: Result<super::CpuTimes, super::ParseError> = proc_stat.parse();
        assert!(cpu_times.is_ok());
        let cpu_times = cpu_times.unwrap();
        assert_eq!(
            cpu_times,
            super::CpuTimes(vec![
                super::CpuTime {
                    user: 61909,
                    nice: 15,
                    system: 25333,
                    idle: 1235463,
                    iowait: 645,
                    irq: 2152,
                    softirq: 1663,
                    steal: 0,
                    guest: 0,
                    guest_nice: 0,
                },
                super::CpuTime {
                    user: 61740,
                    nice: 11,
                    system: 26824,
                    idle: 1235295,
                    iowait: 508,
                    irq: 2713,
                    softirq: 967,
                    steal: 0,
                    guest: 0,
                    guest_nice: 0,
                }
            ])
        );
    }

    #[test]
    fn cputimes_fail() {
        let proc_stat = "";
        let cpu_times: Result<super::CpuTimes, super::ParseError> = proc_stat.parse();
        assert_eq!(cpu_times, Err(super::ParseError::NoCpus));

        // cpu0 misses guest_nice (last column)
        let proc_stat = r#"cpu  488167 545 209081 9891865 4452 16220 9200 0 0 0
cpu0 61909 15 25333 1235463 645 2152 1663 0 0
cpu1 61740 11 26824 1235295 508 2713 967 0 0 0
"#;
        let cpu_times: Result<super::CpuTimes, super::ParseError> = proc_stat.parse();
        assert_eq!(cpu_times, Err(super::ParseError::GuestNice));

        // cpu0 has invalid user time
        let proc_stat = r#"cpu  488167 545 209081 9891865 4452 16220 9200 0 0 0
cpu0 parseme! 15 25333 1235463 645 2152 1663 0 0 0
cpu1 61740 11 26824 1235295 508 2713 967 0 0 0
"#;
        let cpu_times: Result<super::CpuTimes, super::ParseError> = proc_stat.parse();
        assert_eq!(cpu_times, Err(super::ParseError::User));
    }
}
