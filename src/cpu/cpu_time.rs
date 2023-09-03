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

#[cfg(test)]
mod tests {
    #[test]
    fn get_next_token() {
        assert_eq!(super::get_next_token(""), None);

        let test_str = "abc def ghi";
        let tok = super::get_next_token(test_str);
        assert_eq!(tok, Some(("abc", "def ghi")));

        let tok = super::get_next_token(tok.unwrap().1);
        assert_eq!(tok, Some(("def", "ghi")));

        let tok = super::get_next_token(tok.unwrap().1);
        assert_eq!(tok, Some(("ghi", "")));

        let tok = super::get_next_token(tok.unwrap().1);
        assert_eq!(tok, None);
    }

    #[test]
    fn cpu_time_from_str() {
        let cpu_stat = "cpu0 1 2 3 4 5 6 7 8 9 10";
        assert_eq!(
            cpu_stat.parse(),
            Ok(super::CpuTime {
                user: 1,
                nice: 2,
                system: 3,
                idle: 4,
                iowait: 5,
                irq: 6,
                softirq: 7,
                steal: 8,
                guest: 9,
                guest_nice: 10,
            })
        );
    }
}
