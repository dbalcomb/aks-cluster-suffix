use std::io::{self, Read, Write};

use serde::{Deserialize, Serialize};

use self::suffix::suffix;

mod lookup;
mod suffix;

#[derive(Deserialize)]
struct Input {
    input: String,
}

#[derive(Serialize)]
struct Output {
    output: String,
}

#[cfg_attr(tarpaulin, skip)]
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;

    let input: Input = serde_json::from_str(&buffer)?;
    let output = Output {
        output: suffix(input.input.as_bytes()),
    };

    io::stdout().write_all(serde_json::to_string(&output)?.as_bytes())?;

    Ok(())
}
