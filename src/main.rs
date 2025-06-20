use std::{process::ExitCode, thread, time::Duration};

use midir::{MidiOutput, MidiOutputConnection};
use pareg::Pareg;
use rand::{Rng, rng};
use termal::{eprintacln, printacln, printcln, raw::readers::prompt_to};

use crate::{
    cli::{Args, help_inside},
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

    let range = args.tone_range.start.0..args.tone_range.end.0;
    let octave_check = range.end - range.start > 12;

    let mut rng = rng();
    let mut buf = String::new();

    let mut tone = Tone(rng.random_range(range.clone()));

    printacln!("Type {'c}help{'_} to show help.");

    loop {
        conn.send(&tone.press(0, 127))?;
        thread::sleep(Duration::from_millis(500));
        conn.send(&tone.release(0, 127))?;
        buf.clear();
        prompt_to(&mut buf, "> ")?;
        println!();

        match buf.as_str() {
            "q" | "quit" => break,
            "?" | "help" => {
                help_inside();
                continue;
            }
            "" => continue,
            _ => {}
        }

        let t: Tone = match buf.parse() {
            Ok(t) => t,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };
        let val = if octave_check {
            t == tone
        } else {
            t.tone() == tone.tone()
        };
        if val {
            printcln!("{'g}Success!{'_}");
        } else {
            printcln!("{'r}Failure!{'_} {tone} (not {t})");
        }
        tone = Tone(rng.random_range(range.clone()));
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
        eprintln!(
            "Choosing port {} ({})",
            p.id(),
            out.port_name(p).unwrap_or_default()
        );
        Ok(out.connect(p, "pitched-midi-play")?)
    }
}
