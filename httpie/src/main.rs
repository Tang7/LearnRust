use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(version = "1.0")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: Subcommand,
}

// Only support Get / Post for now
#[derive(Clap, Debug)]
enum Subcommand {
    Get(Get),
    Post(Post),
}

#[derive(Clap, Debug)]
struct Get {
    url: String, // http request url
}

#[derive(Clap, Debug)]
struct Post {
    url: String, // http request url
    body: Vec<String>, // http request data, json Key-Value pair format
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
