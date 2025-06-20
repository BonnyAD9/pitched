use std::{
    borrow::Cow,
    io::{IsTerminal, stdout},
};

use termal::{gradient, printmcln};

pub fn help() {
    let color = stdout().is_terminal();
    let sign: Cow<str> = if color {
        gradient("BonnyAD9", (250, 50, 170), (180, 50, 240)).into()
    } else {
        "BonnyAD9".into()
    };
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");

    printmcln!(
        color,
        "Welcome in help for {'g i}pitched{'_} by {sign}{'_}.
version: {version}

{'g}Usage:
  {'c}pitched {'gr}[{'dy}flags{'gr}]{'_}
    Start pitched.
    
{'g}Flags:
  {'y}-h  -?  --help{'_}
    Show this help.
    
  {'y}-p  --port {'w}<midi-port>{'_}
    Select the midi port to use. By default chooses the last midi port.
    
  {'y}-r  --range {'w}<start-note>..<end-note>{'_}
    Specify the range of notes that will be generated. By default the range is
    C3..C4. Note that the start of the range is inclusive and the end is
    exclusive. If the range is less than octave, than when guessing, the octave
    is automatically infered to be the correct one instead of 4.
    
  “ {'i}One who despises the word will do badly, But
    one who fears the commandment will be rewarded.{'_} ”
                                     {'bold w}✝ Proverbs 13:13{'_}
"
    )
}

pub fn help_inside() {
    let color = stdout().is_terminal();
    let sign: Cow<str> = if color {
        gradient("BonnyAD9", (250, 50, 170), (180, 50, 240)).into()
    } else {
        "BonnyAD9".into()
    };
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");

    printmcln!(
        color,
        "Welcome in help for {'g i}pitched{'_} by {sign}{'_}.
version: {version}

{'g}Usage:
  {'c}?  help{'_}
    Show this help.
    
  {'c}q  quit{'_}
    Exit the app.
    
  {'c}<note>{'_}
    Guess the given note.

  {'c}<enter/empty>{'_}
    Play the note again.
    
Notes satisfy the pattern /{'i}[CDEFGAHBcdefgahb](#|is|s|es|b)?[0-9]*{'_}/. `H`
and `B` represent the same note. The octave is implicitly 4, but when guessing
over single octave, the octave is ignored.

  “ {'i}One who despises the word will do badly, But
    one who fears the commandment will be rewarded.{'_} ”
                                     {'bold w}✝ Proverbs 13:13{'_}
    "
    )
}
