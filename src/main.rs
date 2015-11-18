extern crate hyper;
extern crate rss;

use std::io::{BufWriter, Read, Write};
use std::fs::File;
// use std::sync::Arc;
use std::thread;

// hyper to make http requests
use hyper::Client;
use hyper::header::Connection;

// rss to parse rss feed
use rss::Rss;


fn main() {
    let client = Client::new();
    let mut res = client.get("http://feeds.twit.tv/twit_video_small.xml")
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
            let client1 = Client::new();
            let link = item.link.unwrap();
            println!("fetching {}", link);
            let v: Vec<&str> = link.split('/').collect();
            let fname = v.last().unwrap();
            println!("fname: {}", fname);
            let mut res = match client1.get(&link).send() {
                Ok(res) => res,
                Err(_) => {
                    panic!("fetch failed");
                }
            };
            println!("we fetched");
            let f = match File::create(fname) {
                Ok(f) => f,
                Err(_) => panic!("Cannot create file!"),
            };
            println!("we created");
            let mut writer = BufWriter::new(f);
            let mut buffer = String::new();
            res.read_to_string(&mut buffer).unwrap();
            let by = buffer.as_bytes();
            writer.write(by).unwrap();
            println!("Fetched !");
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
