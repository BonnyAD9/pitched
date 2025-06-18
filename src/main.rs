use std::{env, thread, time::Duration};

use anyhow::{Result, bail};
use midir::MidiOutput;
use rand::{rng, Rng};
use termal::{codes, printcln, raw::readers::{prompt_to, read_line}};

use crate::tone::Tone;

mod tone;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("Invalid number of arguments!");
    }
    
    let port: &str = &args[1];

    let out = MidiOutput::new("pitched-midi-out")?;
    let mut conn = None;

    for p in out.ports() {
        if p.id() == port {
            conn = Some(
                out.connect(&p, "pitched-midi-play")
                    .map_err(|e| anyhow::Error::msg(e.to_string()))?,
            );
            break;
        }
    }

    let Some(mut conn) = conn else {
        bail!("No midi port.");
    };
    
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
