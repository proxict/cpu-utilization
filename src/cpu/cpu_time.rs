use std::io;
use std::str::FromStr;

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
    type Err = io::Error;

    fn from_str(cpu_stat: &str) -> Result<Self, Self::Err> {
        let (_cpu_name, cpu_stat) = get_next_token(cpu_stat).ok_or(CpuTimeParseError::CpuName)?;
        let (user, cpu_stat) = get_next_token(cpu_stat).ok_or(CpuTimeParseError::User)?;
        let (nice, cpu_stat) = get_next_token(cpu_stat).ok_or(CpuTimeParseError::Nice)?;
        let (system, cpu_stat) = get_next_token(cpu_stat).ok_or(CpuTimeParseError::System)?;
        let (idle, cpu_stat) = get_next_token(cpu_stat).ok_or(CpuTimeParseError::Idle)?;
        let (iowait, cpu_stat) = get_next_token(cpu_stat).ok_or(CpuTimeParseError::IoWait)?;
        let (irq, cpu_stat) = get_next_token(cpu_stat).ok_or(CpuTimeParseError::Irq)?;
        let (softirq, cpu_stat) = get_next_token(cpu_stat).ok_or(CpuTimeParseError::SoftIrq)?;
        let (steal, cpu_stat) = get_next_token(cpu_stat).ok_or(CpuTimeParseError::Steal)?;
        let (guest, cpu_stat) = get_next_token(cpu_stat).ok_or(CpuTimeParseError::Guest)?;
        let (guest_nice, _) = get_next_token(cpu_stat).ok_or(CpuTimeParseError::GuestNice)?;

        let parse_token = |token: &str| {
            token
                .parse::<u64>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        };

        Ok(CpuTime {
            user: parse_token(user)?,
            nice: parse_token(nice)?,
            system: parse_token(system)?,
            idle: parse_token(idle)?,
            iowait: parse_token(iowait)?,
            irq: parse_token(irq)?,
            softirq: parse_token(softirq)?,
            steal: parse_token(steal)?,
            guest: parse_token(guest)?,
            guest_nice: parse_token(guest_nice)?,
        })
    }
}

enum CpuTimeParseError {
    CpuName,
    User,
    Nice,
    System,
    Idle,
    IoWait,
    Irq,
    SoftIrq,
    Steal,
    Guest,
    GuestNice,
}

impl From<CpuTimeParseError> for &'static str {
    fn from(parse_error: CpuTimeParseError) -> Self {
        match parse_error {
            CpuTimeParseError::CpuName => "cpu name",
            CpuTimeParseError::User => "user",
            CpuTimeParseError::Nice => "nice",
            CpuTimeParseError::System => "system",
            CpuTimeParseError::Idle => "idle",
            CpuTimeParseError::IoWait => "IO wait",
            CpuTimeParseError::Irq => "IRQ",
            CpuTimeParseError::SoftIrq => "soft IRQ",
            CpuTimeParseError::Steal => "steal",
            CpuTimeParseError::Guest => "guest",
            CpuTimeParseError::GuestNice => "guest nice",
        }
    }
}

impl From<CpuTimeParseError> for io::Error {
    fn from(parse_error: CpuTimeParseError) -> Self {
        let msg: &str = parse_error.into();
        io::Error::new(io::ErrorKind::InvalidInput, msg)
    }
}
