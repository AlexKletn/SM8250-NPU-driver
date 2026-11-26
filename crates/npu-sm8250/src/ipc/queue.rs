#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Queue {
    HighPriority,
    AppsCommand,
    DspCommand,
    AppsResponse,
    DspResponse,
    FirmwareLog,
}

impl Queue {
    pub const ALL: [Self; 6] = [
        Self::HighPriority,
        Self::AppsCommand,
        Self::DspCommand,
        Self::AppsResponse,
        Self::DspResponse,
        Self::FirmwareLog,
    ];

    pub const fn capacity_bytes(self) -> usize {
        match self {
            Self::HighPriority | Self::FirmwareLog => 1024,
            _ => 4096,
        }
    }
}
