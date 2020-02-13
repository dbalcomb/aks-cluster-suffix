use std::io::{self, Read, Write};

use aks_cluster_suffix::suffix;
use serde::{Deserialize, Serialize};

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
