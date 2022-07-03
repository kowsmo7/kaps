use crate::image::Region;
use anyhow::anyhow;
use std::convert;
use std::error;
use std::fmt::{self, Display};
use std::io::{self, ErrorKind};
use std::process::{Command, Output};

#[derive(Debug)]
pub enum SlopError {
    Cancelled,
    BadOutput(anyhow::Error),
    NotFound,
    SpawnError(io::Error),
}

impl Display for SlopError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use SlopError::*;
        write!(
            f,
            "{}",
            match self {
                Cancelled => "Command was cancelled by user.".into(),
                BadOutput(e) => format!("Couldn't parse slop output: {e}"),
                NotFound => "Command `slop` not found.".into(),
                SpawnError(e) => format!("Failed to spawn command `slop`: {e}"),
            }
        )
    }
}

impl error::Error for SlopError {}

const HAS_FORMAT_ARG: &str = "Extra slop arguments cannot contain the -f/--format arguments.";

fn parse_output(output: Output) -> Result<Region, SlopError> {
    use SlopError::*;

    // this path seems to never hit, but whatever
    if !output.status.success() {
        return Err(Cancelled);
    }

    let stdout = std::str::from_utf8(&output.stdout).map_err(|e| BadOutput(e.into()))?;

    if stdout.contains("cancelled") {
        return Err(Cancelled);
    }

    // a little bit of rusty magic
    let slice: Vec<u32> = stdout
        .split(' ')
        .map(|n| n.parse())
        .collect::<Result<Vec<u32>, _>>()
        .map_err(|e| BadOutput(e.into()))?;

    let [x, y, w, h]: [u32; 4] = match slice.len() {
        4 => slice.try_into().unwrap(),
        _ => unreachable!(),
    };

    Ok(Region::new(x as _, y as _, w as _, h as _))
}

pub fn ensure_arguments(extra_args: &[String]) -> anyhow::Result<()> {
    match extra_args
        .iter()
        .any(|arg| arg == "-f" || arg.starts_with("--format"))
    {
        false => Ok(()),
        true => Err(anyhow!(HAS_FORMAT_ARG)),
    }
}

pub fn get_slop_selection(mut extra_args: Vec<String>) -> Result<Region, SlopError> {
    // ASSUMES NO -f / --format ARGUMENT
    extra_args.push("-f".into());
    extra_args.push("%x %y %w %h".into());

    Command::new("slop")
        .args(&extra_args)
        .output()
        .map_err(|e| match e.kind() {
            ErrorKind::NotFound => SlopError::NotFound,
            _ => SlopError::SpawnError(e),
        })
        .map(parse_output)
        .and_then(convert::identity) // essentially Result::flatten (but stable)
}
