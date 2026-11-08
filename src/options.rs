//! Command-line options for accelerate.

/// Operating mode
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    /// Default mode
    Default,
    /// List devices mode
    ListDevices,
    /// List contexts mode
    ListContexts,
    /// Stats mode
    Stats,
    /// Create context mode
    CreateContext(String),
    /// Destroy context mode
    DestroyContext(String),
}

/// Command-line options
#[derive(Debug, Clone)]
pub struct Options {
    /// Operating mode
    pub mode: Mode,
    /// Config file path
    pub config_path: String,
    /// Verbose output
    pub verbose: bool,
    /// Quiet mode
    pub quiet: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            mode: Mode::Default,
            config_path: String::new(),
            verbose: false,
            quiet: false,
        }
    }
}
