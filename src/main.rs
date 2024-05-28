mod reader;
mod requester;

use clap::Parser;
use reader::FileContext;

#[derive(Parser)]
struct Cli {
    format: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let file_ctx = FileContext {
        format: args.format,
        path: args.path,
    };
    let yaml = reader::read(file_ctx);
    match requester::init_requests(yaml) {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }
}
