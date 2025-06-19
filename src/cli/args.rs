use pareg::ParegRef;

use crate::{cli::help, err::Result};

#[derive(Debug, Default)]
pub struct Args {
    pub midi_port: Option<String>,
    pub helped: bool,
}

impl Args {
    pub fn parse(mut args: ParegRef) -> Result<Self> {
        let mut res = Self::default();

        while let Some(arg) = args.next() {
            match arg {
                "-h" | "-?" | "--help" => {
                    help();
                    res.helped = true;
                }
                "-p" | "--port" => res.midi_port = Some(args.next_arg()?),
                _ => args.err_unknown_argument().err()?,
            }
        }

        Ok(res)
    }

    pub fn run(&self) -> bool {
        !self.helped || self.midi_port.is_some()
    }
}
