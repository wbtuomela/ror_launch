#[derive(Debug, PartialEq)]
pub enum EOpCode {
    ClCheck = 1,
    LcrCheck,
    ClStart,
    LcrStart,
    ClCreate,
    LcrCreate,
    ClInfo,
    LcrInfo,
    // should always be last
    Invalid,
}

impl Into<u8> for EOpCode {
    fn into(self) -> u8 {
        match self {
            EOpCode::Invalid => 0u8,
            EOpCode::ClCheck => 1u8,
            EOpCode::LcrCheck => 2u8,
            EOpCode::ClStart => 3u8,
            EOpCode::LcrStart => 4u8,
            EOpCode::ClCreate => 5u8,
            EOpCode::LcrCreate => 6u8,
            EOpCode::ClInfo => 7u8,
            EOpCode::LcrInfo => 8u8,
        }
    }
}

impl From<u8> for EOpCode {
    fn from(src: u8) -> Self {
        match src {
            1u8 => EOpCode::ClCheck,
            2u8 => EOpCode::LcrCheck,
            3u8 => EOpCode::ClStart,
            4u8 => EOpCode::LcrStart,
            5u8 => EOpCode::ClCreate,
            6u8 => EOpCode::LcrCreate,
            7u8 => EOpCode::ClInfo,
            8u8 => EOpCode::LcrInfo,
            _ => EOpCode::Invalid,
        }
    }
}

pub enum ECheckResult {
    Success,
    Error,
    UpdateRequired,
    // should always be last
    Invalid,
}

impl From<u8> for ECheckResult {
    fn from(src: u8) -> Self {
        match src {
            0u8 => ECheckResult::Success,
            1u8 => ECheckResult::Error,
            2u8 => ECheckResult::UpdateRequired,
            _ => ECheckResult::Invalid,
        }
    }
}
