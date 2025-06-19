use std::{process::ExitCode, thread, time::Duration};

use midir::{MidiOutput, MidiOutputConnection};
use pareg::Pareg;
use rand::{Rng, rng};
use termal::{eprintacln, printcln, raw::readers::prompt_to};

use crate::{
    cli::Args,
    err::{Error, Result},
    tone::Tone,
};

mod cli;
mod err;
mod tone;

fn main() -> ExitCode {
    match start() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintacln!("{'r}error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn start() -> Result<()> {
    let mut args = Args::parse(Pareg::args().get_mut_ref())?;

    if !args.run() {
        return Ok(());
    }

    let out = MidiOutput::new("pitched-midi-out")?;
    let mut conn = midi_connect(out, args.midi_port.take())?;

    let range = 60..72;
    let mut rng = rng();
    let mut buf = String::new();

    loop {
        let t = rng.random_range(range.clone());
        let tone = Tone(t);
        conn.send(&tone.press(0, 127))?;
        thread::sleep(Duration::from_millis(500));
        conn.send(&tone.release(0, 127))?;
        buf.clear();
        prompt_to(&mut buf, "> ")?;
        println!();
        if buf == "q" {
            break;
        }
        let mut t: Tone = match buf.parse() {
            Ok(t) => t,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };
        t.0 -= 12; // Change the default octave 4 to octave 3
        if t == tone {
            printcln!("{'g}Success!{'_}");
        } else {
            printcln!("{'r}Failure!{'_} {tone} (not {t})");
        }
    }

    Ok(())
}

fn midi_connect(
    out: MidiOutput,
    port: Option<String>,
) -> Result<MidiOutputConnection> {
    if let Some(port) = port {
        for p in out.ports() {
            if p.id() == port {
                return Ok(out.connect(&p, "pitched-midi-play")?);
            }
        }
        Err(Error::NoMidiPort(port))
    } else {
        let ports = out.ports();
        let Some(p) = ports.last() else {
            return Err(Error::NoMidiPort(String::new()));
        };
        Ok(out.connect(p, "pitched-midi-play")?)
    }
}
