pub enum ExitCode {
    Success = 0,

    /// User input
    /// - Illegal command line argument (CLI)
    InputError,
}
