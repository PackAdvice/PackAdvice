use packadvice::PackAdviserError;

pub enum ExitCode {
    Success = 0,
    InputError,
    IoError,
}

impl From<PackAdviserError> for ExitCode {
    fn from(err: PackAdviserError) -> Self {
        match err {
            PackAdviserError::IoError(_) => ExitCode::IoError
        }
    }
}
