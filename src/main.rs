extern crate hyper;
extern crate rss;
extern crate chrono;

use std::io::{BufWriter, Read, Write};
use std::fs::File;
use std::thread;
use chrono::*;

// hyper to make http requests
use hyper::Client;
use hyper::header::Connection;

// rss to parse rss feed
use rss::Rss;

fn main() {
    let client = Client::new();
    let mut res = client.get("http://feeds.twit.tv/twit.xml")
        .header(Connection::close())
        .send()
        .unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let Rss(channel) = body.parse::<Rss>().unwrap();
    println!("Sucking Channel: {}", channel.title);
    // iterate over the items
    let handles: Vec<_> = channel.items.into_iter().map(|item| {
        thread::spawn(move || {
            let start: DateTime<Local> = Local::now();
            let client1 = Client::new();
            let link = item.link.unwrap();
            println!("fetching {}", link);
            let v: Vec<&str> = link.split('/').collect();
            let fname = v.last().unwrap();
            let mut res = match client1.get(&link).send() {
                Ok(res) => res,
                Err(_) => panic!("fetch failed"),
            };
            let f = match File::create(fname) {
                Ok(f) => f,
                Err(_) => panic!("Cannot create file!"),
            };
            let mut writer = BufWriter::new(f);
            let mut buffer = vec![];
            res.read_to_end(&mut buffer).unwrap();
            writer.write(&buffer).unwrap();
            let end: DateTime<Local> = Local::now();
            println!("Fetched file: {}\nStart: {}\nEnd: {}",fname, start, end);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
