use std::{env, path::PathBuf};

use crate::Conflicts;
use crate::result::CliError;
use clap::Parser;

///cargo nuke is the cargo clean bomb , clean all cargo builds in directory save millons of bytes.
#[derive(Parser, Debug)]
#[command(name = "cargo-nuke")]
#[command(about = "purges all cargo build files in directory,reclaim disk space")]
#[command(version)]
pub struct NukeCli {
    /// Positional argument to accept "nuke" from `cargo nuke` invocation
    /// This will be ignored - it's only here to consume the subcommand name
    #[arg(hide = true)]  // Hide from --help output
    pub cmd: Option<String>,

    ///Optional path, default is current dir
    #[arg(long, short)]
    pub path: Option<PathBuf>,

    ///flag to bypass nuke confirmation (read docs carefully)
    #[arg(long, short)]
    pub sure: bool,

    ///flag to dry-run
    #[arg(long("dry-run"))]
    dry_run: bool,

    ///argument to specify days - default is 30 days
    #[arg(long, default_value_t = 30)]
    pub older_than: u64,
}

impl NukeCli {
    pub  fn parse() -> ArgsList {

        let conflict: Conflicts;
        let args = <NukeCli as Parser>::parse();
                // args.cmd will be Some("nuke") when called as `cargo nuke`
                // and None when called as `cargo-nuke`
                // We just ignore it either way
        let path = if let Some(p) = args.path {
            p
        } else {
            match env::current_dir() {
                Ok(p) => p,
                Err(e) => {
                    println!("access-previlege needed to use cargo nuke");
                    let e = e.to_string();
                    panic!("{e}");
                }
            }
        };

        let sure = args.sure;

        let days = args.older_than;
        let dry_run = args.dry_run;
        if dry_run && sure {
            conflict = Conflicts::new(
                true,
                Some(CliError::OptConflict("dry-run".into(), "sure".into())),
            )
        } else {
            conflict = Conflicts::new(false, None);
        }

        ArgsList {
            path,
            sure,
            days,
            dry_run,
            conflict,
        }
    }
}

///`ArgsList` is the final args parsed from command line
#[derive(Debug)]
pub struct ArgsList {

    ///path passed in command line through `--path`
    /// or `env::current_dir()` of no path is passed
    path: PathBuf,

    /// `true` if `--sure` is passed, `false` otherwise.
    sure: bool,

    /// `true` if `--dry-run` is passed, `false` otherwise.
    dry_run: bool,

    /// days passed in `--older-than`, 30 default otherwise
    days: u64,

    /// conflicts as such as , `--sure` with `--dry-run` is conflict.
    pub conflict: Conflicts,
}

impl ArgsList {
    pub fn is_sure(&self) -> bool {
        self.sure
    }

    pub fn is_dry(&self) -> bool {
        self.dry_run
    }

    pub fn days(&self) -> u64 {
        self.days
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
