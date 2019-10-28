#[derive(Debug)]
pub struct Error {
    pub message: String
}

pub type Result<T> = std::result::Result<T, Error>;

