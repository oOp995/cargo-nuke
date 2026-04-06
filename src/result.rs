use std::{error::Error, fmt::Display};


///`CliError` represents CLI error possible in `cargo-nuke`

#[derive(Debug)]
pub enum CliError {
    _ArgsConflict(String, String),
    OptConflict(String, String),
}

impl Error for CliError {}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            CliError::_ArgsConflict(arg1, arg2) => {
                write!(
                    f,
                    "cli-Error:\nargs conflict : arg --{} can't be used with arg --{}",
                    arg1, arg2
                )
            }
            CliError::OptConflict(opt1, opt2) => {
                write!(
                    f,
                    "cli-Error:\noptions conflict: option --{} can't be used with option --{}",
                    opt1, opt2
                )
            }
        }
    }
}

#[derive(Debug)]
pub struct Conflicts {
    is: bool,
    error: Option<CliError>,
}
impl Conflicts {
    pub fn new(is: bool, error: Option<CliError>) -> Self {
        Self { is, error }
    }

    pub fn is_conflict(&self) -> bool {
        self.is
    }

    pub fn reason(&self) -> Option<CliError> {
        match &self.error.as_ref().unwrap() {
            CliError::_ArgsConflict(arg1, arg2) => {
                Some(CliError::_ArgsConflict(arg1.into(), arg2.into()))
            }
            CliError::OptConflict(arg1, arg2) => {
                Some(CliError::OptConflict(arg1.into(), arg2.into()))
            }
        }
    }
}
