// -- spot_error.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotError {
    file: &'static str,
    line: u32,
    desc: String,
}
impl SpotError {
    pub fn new(file: &'static str, line: u32, desc: &str) -> Self {
        Self {
            file,
            line,
            desc: desc.to_string(),
        }
    }
}

impl std::error::Error for SpotError {}

impl std::fmt::Display for SpotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SpotError({}: {}, {:?})",
            self.file, self.line, self.desc
        )
    }
}

pub type SpotResult<T> = Result<T, SpotError>;
