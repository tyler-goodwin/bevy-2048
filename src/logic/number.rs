#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    ELEVEN,
    TWELVE,
    THIRTEEN,
    FOURTEEN,
    FIFTEEN,
    SIXTEEN,
}

impl Number {
    pub fn value(&self) -> i32 {
        match self {
            Self::ZERO => 2,
            Self::ONE => 4,
            Self::TWO => 8,
            Self::THREE => 16,
            Self::FOUR => 32,
            Self::FIVE => 64,
            Self::SIX => 128,
            Self::SEVEN => 256,
            Self::EIGHT => 512,
            Self::NINE => 1024,
            Self::TEN => 2048,
            Self::ELEVEN => 4096,
            Self::TWELVE => 8192,
            Self::THIRTEEN => 16384,
            Self::FOURTEEN => 32768,
            Self::FIFTEEN => 65536,
            Self::SIXTEEN => 131072,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::ZERO => Self::ONE,
            Self::ONE => Self::TWO,
            Self::TWO => Self::THREE,
            Self::THREE => Self::FOUR,
            Self::FOUR => Self::FIVE,
            Self::FIVE => Self::SIX,
            Self::SIX => Self::SEVEN,
            Self::SEVEN => Self::EIGHT,
            Self::EIGHT => Self::NINE,
            Self::NINE => Self::TEN,
            Self::TEN => Self::ELEVEN,
            Self::ELEVEN => Self::TWELVE,
            Self::TWELVE => Self::THIRTEEN,
            Self::THIRTEEN => Self::FOURTEEN,
            Self::FOURTEEN => Self::FIFTEEN,
            Self::FIFTEEN => Self::SIXTEEN,
            Self::SIXTEEN => Self::SIXTEEN,
        }
    }
}
