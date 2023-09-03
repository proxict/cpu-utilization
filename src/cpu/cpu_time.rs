use super::parse_error::ParseError;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct CpuTime {
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
    pub steal: u64,
    pub guest: u64,
    pub guest_nice: u64,
}

impl CpuTime {
    pub fn get_total(&self) -> u64 {
        self.user + self.nice + self.system + self.idle + self.steal
    }
}

fn get_next_token(string: &str) -> Option<(&str, &str)> {
    for (i, c) in string.chars().enumerate() {
        if c == ' ' {
            return Some((&string[..i], &string[i + 1..]));
        }
    }
    match string.is_empty() {
        true => None,
        false => Some((string, &string[string.len()..])),
    }
}

impl FromStr for CpuTime {
    type Err = ParseError;

    fn from_str(cpu_stat: &str) -> Result<Self, Self::Err> {
        let (_cpu_name, cpu_stat) = get_next_token(cpu_stat).ok_or(ParseError::CpuName)?;
        let (user, cpu_stat) = get_next_token(cpu_stat).ok_or(ParseError::User)?;
        let (nice, cpu_stat) = get_next_token(cpu_stat).ok_or(ParseError::Nice)?;
        let (system, cpu_stat) = get_next_token(cpu_stat).ok_or(ParseError::System)?;
        let (idle, cpu_stat) = get_next_token(cpu_stat).ok_or(ParseError::Idle)?;
        let (iowait, cpu_stat) = get_next_token(cpu_stat).ok_or(ParseError::IoWait)?;
        let (irq, cpu_stat) = get_next_token(cpu_stat).ok_or(ParseError::Irq)?;
        let (softirq, cpu_stat) = get_next_token(cpu_stat).ok_or(ParseError::SoftIrq)?;
        let (steal, cpu_stat) = get_next_token(cpu_stat).ok_or(ParseError::Steal)?;
        let (guest, cpu_stat) = get_next_token(cpu_stat).ok_or(ParseError::Guest)?;
        let (guest_nice, _) = get_next_token(cpu_stat).ok_or(ParseError::GuestNice)?;

        Ok(CpuTime {
            user: user.parse::<u64>().map_err(|_| ParseError::User)?,
            nice: nice.parse::<u64>().map_err(|_| ParseError::Nice)?,
            system: system.parse::<u64>().map_err(|_| ParseError::System)?,
            idle: idle.parse::<u64>().map_err(|_| ParseError::Idle)?,
            iowait: iowait.parse::<u64>().map_err(|_| ParseError::IoWait)?,
            irq: irq.parse::<u64>().map_err(|_| ParseError::Irq)?,
            softirq: softirq.parse::<u64>().map_err(|_| ParseError::SoftIrq)?,
            steal: steal.parse::<u64>().map_err(|_| ParseError::Steal)?,
            guest: guest.parse::<u64>().map_err(|_| ParseError::Guest)?,
            guest_nice: guest_nice
                .parse::<u64>()
                .map_err(|_| ParseError::GuestNice)?,
        })
    }
}
