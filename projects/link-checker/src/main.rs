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

    let (dispatcher_in, dispather_out) = mpsc::channel();
    let (extractor_in, extractor_out) = mpsc::channel();

    // wait input
    thread::spawn(|| {
        for received in extractor_out {
            // todo: send to dispather_in
            match received {
                Ok(links) => println!("Links: {links:#?}"),
                Err(err) => println!("Could not extract links: {err:#}"),
            }
        }
    });


    // initialize
    dispatcher_in.send("https://www.google.org").unwrap();

    // extracct
    let mut handles = vec![];
    for url in dispather_out {
        // // todo: break condition
        // if true {
        //     break
        // }

        let client = Client::new();
        let start_url = Url::parse(url).unwrap();
        let crawl_command = CrawlCommand{ url: start_url, extract_links: true };

        // link extractor
        let tx = mpsc::Sender::clone(&extractor_in);
        let handle = thread::spawn(move || {
            let result = visit_page(&client, &crawl_command);
            tx.send(result).unwrap();
        });
        handles.push(handle);

        // todo: break condition
        if true {
            break
        }
    };

    for handle in handles {
        handle.join().unwrap();
    }

    println!("finish.")
}
