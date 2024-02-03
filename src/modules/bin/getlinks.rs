use crate::Args;
use reqwest::Client;
use scraper::{Html, Selector};
use url::Url;
//use std::collections::VecDeque;
pub fn filter_url(link: &str, args: &Args) -> bool {
    let mut ok = true;
    if args.ignore_relative && !link.contains("http") {
        ok = false;
    }
    if args.ignore_http && link.contains("http:") {
        ok = false;
    }
    if args.ignore_https && link.contains("https:") {
        ok = false;
    }
    let str_filters = args.substr_filter.clone().take();
    match str_filters {
        Some(str_filters) => {
            for str_filter in str_filters.iter() {
                match str_filter.chars().nth(0).unwrap() {
                    '*' => {
                        println!("{}", &str_filter[1..]);
                        ok = link.contains(&str_filter[1..]);
                    }
                    '!' => {
                        ok = !link.contains(&str_filter[1..]);
                    }
                    _ => {
                        panic!("bad wildcard {}", str_filter);
                    }
                }
            }
        }
        None => {}
    };

    ok
}

pub async fn get_links(url: &str, args: Args) -> Vec<String> {
    let client: Client = Client::new();
    let response = client.get(url).send().await.expect("error");

    let body = response.text().await.expect("Hello");

    let document = Html::parse_document(&body);
    let selector = Selector::parse("a[href]").expect("Failed to parse CSS selector");

    let mut url_vec: Vec<String> = document
        .select(&selector)
        .filter_map(|link| link.value().attr("href"))
        .map(|href| href.to_string())
        .collect::<Vec<String>>()
        .into_iter()
        .collect();

    if args.convert_relative {
        url_vec = url_vec
            .into_iter()
            .map(|link: String| {
                if link.chars().nth(0).unwrap_or('0') == '/' {
                    let parsed_url = Url::parse(url)
                        .and_then(|base| base.join(&link))
                        .expect("couldnt join URLS");
                    parsed_url.into()
                } else {
                    link
                }
            })
            .collect();
    }

    url_vec
}
