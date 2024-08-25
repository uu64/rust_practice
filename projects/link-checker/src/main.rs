use std::{sync::mpsc, thread};

use reqwest::blocking::Client;
use reqwest::Url;
use scraper::{Html, Selector};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("bad http response: {0}")]
    BadResponse(String),
}

#[derive(Debug)]
struct CrawlCommand {
    url: Url,
    extract_links: bool,
}

fn visit_page(client: &Client, command: &CrawlCommand) -> Result<Vec<Url>, Error> {
    println!("Checking {:#}", command.url);
    let response = client.get(command.url.clone()).send()?;
    if !response.status().is_success() {
        return Err(Error::BadResponse(response.status().to_string()));
    }

    let mut link_urls = Vec::new();
    if !command.extract_links {
        return Ok(link_urls);
    }

    let base_url = response.url().to_owned();
    let body_text = response.text()?;
    let document = Html::parse_document(&body_text);

    let selector = Selector::parse("a").unwrap();
    let href_values = document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"));
    for href in href_values {
        match base_url.join(href) {
            Ok(link_url) => {
                link_urls.push(link_url);
            }
            Err(err) => {
                println!("On {base_url:#}: ignored unparsable {href:?}: {err}");
            }
        }
    }
    Ok(link_urls)
}

fn main() {
    let (tx, rx) = mpsc::channel();

    // initialize
    tx.send(String::from("https://www.google.org")).unwrap();

    // extracct
    let mut handles = vec![];
    for (num_processed, url) in rx.into_iter().enumerate() {
        if num_processed > 9 {
            break
        }

        let client = Client::new();
        let start_url = Url::parse(&url).unwrap();
        let crawl_command = CrawlCommand{ url: start_url, extract_links: true };
        let tx = mpsc::Sender::clone(&tx);

        let handle = thread::spawn(move || {
            let result = visit_page(&client, &crawl_command);
            match result {
                Ok(links) => {
                    for l in links {
                        if tx.send(l.to_string()).is_err() {
                            // channel is probably closed
                            break;
                        }
                    }
                },
                Err(err) => println!("Could not extract links: {err:#}"),
            }
        });
        handles.push(handle);
    };

    for handle in handles {
        handle.join().unwrap();
    }

    println!("finish.")
}
