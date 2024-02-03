use crate::modules::bin::getlinks::get_links;
use tokio;
mod modules {
    pub mod bin {
        pub mod getlinks;
    }
}
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    url: String,

    ///Eliminate from the list urls that are relative (contian no ://, *.*.* or any other kind of
    //patterns
    #[arg(long, default_value_t = false)]
    ignore_relative: bool,

    ///Eliminate from the list urls that are http
    #[arg(long, default_value_t = false)]
    ignore_http: bool,

    ///Eliminate from the list urls that are https
    #[arg(long, default_value_t = false)]
    ignore_https: bool,

    ///Filter for substrings in url, !substr1 will only display urls without substr1 in them, and
    ///*substr will only display urls with substr in them, there can be multiple filters such as
    ///*substr1 *substr2 *substr3, combining whitelist and blacklist filters is possible but might
    ///yield bad results
    #[arg(long)]
    substr_filter: Option<Vec<String>>,

    #[arg(
        short,
        long,
        conflicts_with = "filter_domain_names",
        conflicts_with = "filter_tld",
        default_value_t = true
    )]
    keep_domain: bool,

    #[arg(
        short,
        long,
        conflicts_with = "ignore_relative",
        default_value_t = false
    )]
    convert_relative: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let url = args.url.clone();
    let res = get_links(&url, args).await.join("\n");
    print!("{}", res);
}
