use std::{borrow::Cow, io::{stdout, IsTerminal}};

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
        "Welcome in help for {'g i}pitched{'_} by {sign}.
version: {version}

{'g}Usage:
  {'c}pitched {'gr}[{'dy}flags{'gr}]{'_}
    Start pitched.
    
{'g}Flags:
  {'y}-h  -?  --help{'_}
    Show this help.
    
  {'y}-p  --port {'w}<midi-port>{'_}
    Select the midi port to use. By default chooses the last midi port.
    
  “ {'i}One who despises the word will do badly, But
    one who fears the commandment will be rewarded.{'_} ”
                                     {'bold w}✝ Proverbs 13:13{'_}
"
    )
}