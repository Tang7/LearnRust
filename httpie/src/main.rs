use anyhow::{anyhow, Result};
use clap::Parser;
use reqwest::Url;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[clap(version = "1.0")]
struct Opts {
    #[clap(subcommand)]
    subcmd: Subcommand,
}

// Only support Get / Post for now
#[derive(Parser, Debug)]
enum Subcommand {
    Get(Get),
    Post(Post),
}

#[derive(Parser, Debug)]
struct Get {
    // http request URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

#[derive(Parser, Debug)]
struct Post {
    // http request URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    // http requet body
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;

    Ok(s.into())
}

#[derive(Debug)]
struct KvPair {
    k: String,
    v: String,
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split_iter = s.split("=");
        // define a closure returns error if the format is incorrect.
        let err = || anyhow!(format!("Failed to parse KvPair {}", s));
        Ok(Self {
            k: (split_iter.next().ok_or_else(err)?).to_string(),
            v: (split_iter.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
