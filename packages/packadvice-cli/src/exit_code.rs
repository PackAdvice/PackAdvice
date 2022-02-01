pub enum ExitCode {
    Success = 0,

    /// User input
    /// - Illegal command line argument
    InputError,

    /// File System - Directory not found
    /// - Non-existent pack directory
    NotFoundDirectory,
}
