use std::process::exit;

use base64::Engine;
use clap::Parser;
use clap_stdin::FileOrStdin;
use log::error;

use crate::BASE64;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(index = 1, name = "input", help = "input file or stdin")]
    input: FileOrStdin,
    #[arg(long, short, help = "treat lines in input as base64 encoded data")]
    base64: bool,
}

impl Args {
    #[must_use]
    pub const fn is_base64(&self) -> bool {
        self.base64
    }

    #[must_use]
    pub fn content(self) -> String {
        self.input.contents().unwrap_or_else(|error| {
            error!("{error}");
            exit(1)
        })
    }

    pub fn b64content(self) -> impl Iterator<Item = (String, Vec<u8>)> {
        self.content()
            .lines()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .into_iter()
            .filter_map(|line| {
                BASE64
                    .decode(&line)
                    .map_err(|error| error!("{error}"))
                    .map(|bytes| (line, bytes))
                    .ok()
            })
    }
}
