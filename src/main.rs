use clap::Parser;
use webpage::{Webpage, WebpageOptions};

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(value_parser)]
    url: String,
}

fn main() {
    let args = Args::parse();
    let info =
        Webpage::from_url(&args.url, WebpageOptions::default()).expect("Could not read from URL");
    println!("{}", serde_json::to_string(&info).unwrap());
}
