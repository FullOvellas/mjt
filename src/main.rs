use clap::Parser;
use clap_stdin::MaybeStdin;
use mjl::JsonLexer;
use mjp::{Json, parse};
use std::error::Error;

use crate::emitting::emit;

mod emitting;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Apply formatting to the input
    Format {
        /// JSON input. If substituted by `-`, it will be read from stdin
        input: MaybeStdin<String>,
        /// Amount of spaces used for indentation
        #[arg(short = 'i', long, default_value_t = 2)]
        indentation: u8,
    },
    /// Aarse the input and fail if it's invalid
    Validate {
        /// JSON input. If substituted by `-`, it will be read from stdin
        input: MaybeStdin<String>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match args.subcommand {
        Subcommand::Format { input, indentation } => {
            println!("{}", emit(do_parse(&input)?, indentation));
            Ok(())
        }
        Subcommand::Validate { input } => do_parse(&input).map(|_| ()),
    }
}

fn do_parse(input: &MaybeStdin<String>) -> Result<Json, Box<dyn Error>> {
    parse(JsonLexer {
        input,
        byte_offset: 0,
    })
}
