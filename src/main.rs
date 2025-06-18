use std::{thread, time::Duration};

use anyhow::{Result, bail};
use midir::MidiOutput;

use crate::tone::Tone;

mod tone;

fn main() -> Result<()> {
    let port = "128:0";

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

    let tone: Tone = "A4".parse()?;

    conn.send(&tone.press(0, 127))?;

    thread::sleep(Duration::from_secs(1));

    conn.send(&tone.release(0, 127))?;

    Ok(())
}
