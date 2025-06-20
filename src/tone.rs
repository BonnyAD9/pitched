use std::{fmt::Display, str::FromStr};

use pareg::{ArgError, FromArgStr, FromRead, parsef, reader::AutoSetFromRead};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    pub const _A4: Self = Self::new(note::A, 4);

    pub const C3: Self = Self::new(note::C, 3);
    pub const C4: Self = Self::new(note::C, 4);

    pub const fn new(tone: u8, octave: u8) -> Self {
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

    pub fn tone(&self) -> u8 {
        self.0 % Self::TONE_CNT
    }

    pub fn octave(&self) -> i8 {
        (self.0 / Self::TONE_CNT) as i8 - Self::OCTAVE_SHIFT as i8
    }
}

impl Display for Tone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.tone() {
            0 => write!(f, "c")?,
            1 => write!(f, "cis")?,
            2 => write!(f, "d")?,
            3 => write!(f, "dis")?,
            4 => write!(f, "e")?,
            5 => write!(f, "f")?,
            6 => write!(f, "fis")?,
            7 => write!(f, "g")?,
            8 => write!(f, "gis")?,
            9 => write!(f, "a")?,
            10 => write!(f, "ais")?,
            11 => write!(f, "h")?,
            _ => unreachable!(),
        }

        write!(f, "{}", self.octave())
    }
}

impl FromRead for Tone {
    fn from_read<'a>(
        r: &mut pareg::Reader,
        _fmt: &'a pareg::ReadFmt<'a>,
    ) -> pareg::Result<(Self, Option<ArgError>)> {
        let pos = r.pos();

        let Some(tone) = r.next()? else {
            return r.err_parse("Missing tone name.").err();
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
            _ => return r.err_parse("Invalid tone name `{tone}`").err(),
        };

        let modifier = match r.peek()? {
            Some('s' | 'b') => {
                r.next()?;
                -1
            }
            Some('#') => {
                r.next()?;
                1
            }
            Some(c @ ('e' | 'i')) => {
                r.next()?;
                if !matches!(r.peek()?, Some('s')) {
                    r.unnext(c);
                    0
                } else if c == 'e' {
                    r.next()?;
                    -1
                } else {
                    r.next()?;
                    1
                }
            }
            _ => 0,
        };

        let octave = match r.peek()? {
            Some(c) if c.is_ascii_digit() => i8::from_read(r, &"".into())?.0,
            Some('-') => {
                r.next()?;
                if matches!(r.peek()?, Some(c) if c.is_ascii_digit()) {
                    r.unnext('-');
                    i8::from_read(r, &"".into())?.0
                } else {
                    r.unnext('-');
                    4
                }
            }
            _ => 4,
        };

        let err_range = || {
            r.err_parse("The tone is out of supported range.")
                .span_start(pos)
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

        Ok((Self(tone), None))
    }
}

impl AutoSetFromRead for Tone {}
impl FromArgStr for Tone {}

impl FromStr for Tone {
    type Err = ArgError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tone = Tone(0);
        parsef!(&mut s.into(), "{tone}")?;
        Ok(tone)
    }
}
