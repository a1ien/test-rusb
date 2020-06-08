#[derive(Copy, Clone)]
pub enum Error {
    NotFound,
    Busy,
    Timeout,
    Overflow,
}

impl Error {
    pub fn strerror(self) -> &'static str {
        match self {
            Error::NotFound => "NotFound",
            Error::Busy => "Busy",
            Error::Timeout => "Timeout",
            Error::Overflow => "Overflow",
        }
    }
}
