use std::ops::Range;

use pareg::{ParegRef, parsef};

use crate::{cli::help, err::Result, tone::Tone};

#[derive(Debug)]
pub struct Args {
    pub midi_port: Option<String>,
    pub helped: bool,
    pub tone_range: Range<Tone>,
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
                "-r" | "--range" => {
                    let arg = args.next_arg::<&str>()?;
                    args.map_res(parsef!(
                        &mut arg.into(),
                        "{}..{}",
                        &mut res.tone_range.start,
                        &mut res.tone_range.end
                    ))?;
                    if res.tone_range.start >= res.tone_range.end {
                        return args
                            .err_invalid()
                            .hint(
                                "First value must be smaller than the second.",
                            )
                            .err()?;
                    }
                }
                _ => args.err_unknown_argument().err()?,
            }
        }

        Ok(res)
    }

    pub fn run(&self) -> bool {
        !self.helped || self.midi_port.is_some()
    }
}

impl Default for Args {
    fn default() -> Self {
        Self {
            midi_port: None,
            helped: false,
            tone_range: Tone::C3..Tone::C4,
        }
    }
}
