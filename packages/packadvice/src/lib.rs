pub enum ExitCode {
    Success = 0,

    /// User input
    /// - Illegal command line argument (CLI)
    InputError,

    /// File System - Directory not found
    /// - Non-existent pack directory
    NotFoundDirectory,
}
