use clap::Parser;
use clap_stdin::MaybeStdin;
use mjl::JsonLexer;
use mjp::parse;
use std::error::Error;

use crate::emitting::emit;

mod emitting;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// JSON input. If substituted by `-`, it will be read from stdin
    pub input: MaybeStdin<String>,
    #[arg(short = 'i', long, default_value_t = 2)]
    pub indentation: u8,
    /// Output reformatted JSON
    #[arg(short = 'p', long, default_value_t = true)]
    pub pretty: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let parsed = parse(JsonLexer {
        input: &args.input,
        byte_offset: 0,
    })?;
    if args.pretty {
        println!("{}", emit(parsed, args.indentation));
        Ok(())
    } else {
        println!("{}", args.input);
        Ok(())
    }
}
