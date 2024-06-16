extern crate reqwest;
extern crate scraper;

use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let res = client.get("https://www.google.com").send()?.text()?;
    let document = Html::parse_document(&res);
    let selector = Selector::parse("a").unwrap();
    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href") {
            println!("{}", link);
        }
    }

    Ok(())
}

