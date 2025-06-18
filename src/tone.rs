use std::str::FromStr;

use pareg::{ArgError, FromArgStr};

#[derive(Clone, Copy, Debug)]
pub struct Tone(pub u8);

mod note {
    pub const C: u8 = 0;
    pub const D: u8 = 2;
    pub const E: u8 = 4;
    pub const F: u8 = 5;
    pub const G: u8 = 7;
    pub const A: u8 = 9;
    pub const B: u8 = 11;
}

impl Tone {
    pub const OCTAVE_SHIFT: u8 = 2;
    pub const TONE_CNT: u8 = 12;

    pub const _A4: Self = Self::_new(note::A, 4);

    pub const fn _new(tone: u8, octave: u8) -> Self {
        Self(tone + (octave + Self::OCTAVE_SHIFT) * Self::TONE_CNT)
    }

    pub fn press(&self, channel: u8, velocity: u8) -> [u8; 3] {
        [0x90 | channel, self.0 & 0x7f, velocity & 0x7f]
    }

    pub fn release(&self, channel: u8, velocity: u8) -> [u8; 3] {
        [0x80 | channel, self.0 & 0x7f, velocity & 0x7f]
    }

    pub fn _instrument(&self, channel: u8, instrument: u8) -> [u8; 2] {
        [0xd0 | channel, instrument & 0x7f]
    }
}

impl FromArgStr for Tone {}

impl FromStr for Tone {
    type Err = pareg::ArgError;

    fn from_str(arg: &str) -> pareg::Result<Self> {
        let Some(tone) = arg.chars().next() else {
            return ArgError::parse_msg("Missing tone name.", arg.to_string())
                .err();
        };

        let tone = match tone.to_ascii_lowercase() {
            'c' => note::C,
            'd' => note::D,
            'e' => note::E,
            'f' => note::F,
            'g' => note::G,
            'a' => note::A,
            'b' => note::B,
            'h' => note::B,
            _ => {
                return ArgError::parse_msg(
                    format!("Invalid tone name `{tone}`."),
                    arg.to_string(),
                )
                .err();
            }
        };

        let modifier = arg[1..].trim_end_matches(|c: char| c.is_ascii_digit());
        let octave = &arg[modifier.len() + 1..];

        let modifier = match modifier {
            "s" | "es" | "b" => -1,
            "is" | "#" => 1,
            "" => 0,
            _ => {
                return ArgError::parse_msg(
                    format!("Invalid modifer `{modifier}`."),
                    arg.to_string(),
                )
                .err();
            }
        };

        let octave = if octave.is_empty() {
            4
        } else {
            octave.parse::<i8>().map_err(|e| {
                ArgError::parse_msg(
                    format!("Invalid octave: {e}."),
                    arg.to_string(),
                )
            })?
        };

        let err_range = || {
            ArgError::parse_msg(
                "The tone is out of supported range.",
                arg.to_string(),
            )
            .err()
        };

        let Some(octave) = Self::OCTAVE_SHIFT.checked_add_signed(octave)
        else {
            return err_range();
        };

        let Some(octave) = Self::TONE_CNT.checked_mul(octave) else {
            return err_range();
        };

        let Some(tone) = octave.checked_add(tone) else {
            return err_range();
        };

        let Some(tone) = tone.checked_add_signed(modifier) else {
            return err_range();
        };

        if tone > 0x7f {
            return err_range();
        }

        Ok(Self(tone))
    }
}
