#[derive(PartialEq, Debug)]
pub enum ParseError {
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
    NoCpus,
    Generic(String),
}

impl From<ParseError> for String {
    fn from(parse_error: ParseError) -> Self {
        match parse_error {
            ParseError::CpuName => "cpu name".to_string(),
            ParseError::User => "user".to_string(),
            ParseError::Nice => "nice".to_string(),
            ParseError::System => "system".to_string(),
            ParseError::Idle => "idle".to_string(),
            ParseError::IoWait => "IO wait".to_string(),
            ParseError::Irq => "IRQ".to_string(),
            ParseError::SoftIrq => "soft IRQ".to_string(),
            ParseError::Steal => "steal".to_string(),
            ParseError::Guest => "guest".to_string(),
            ParseError::GuestNice => "guest nice".to_string(),
            ParseError::NoCpus => "no CPUs".to_string(),
            ParseError::Generic(message) => message,
        }
    }
}
