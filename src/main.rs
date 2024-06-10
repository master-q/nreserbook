extern crate reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{thread, time};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct CalilCheck {
    session: String,
    r#continue: i32,
    books: Map<String, Value>,
}

const APPKEY: &str = env!("CALIL_APPKEY");
const MAXREQ: usize = 100;
const LIBRARY: &str = "Kanagawa_Yokohama";

// private
fn isbn_to_reserveurl_once(isbns: Vec<String>) -> HashMap<String, String> {
    assert!(isbns.len() <= MAXREQ);

    let mut session: Option<String> = None;
    let mut cont;

    println!("+");
    loop {
        // API specification
        // https://calil.jp/doc/api_ref.html
        let url = match session {
            None => "https://api.calil.jp/check?appkey".to_string()
                + APPKEY
                + "&isbn="
                + &isbns.join(",")
                + "&systemid="
                + LIBRARY
                + "&format=json&callback=no",
            Some(s) => "https://api.calil.jp/check?appkey".to_string()
                + APPKEY
                + "&session="
                + &s
                + "&format=json&callback=no"
        };
        let res = reqwest::blocking::get(url).unwrap().text().unwrap();
        let json: CalilCheck = serde_json::from_str(&res).unwrap();
        session = Some(json.session);
        cont = json.r#continue;

        if cont == 0 {
            let mut reserveurls = HashMap::new();
            let iter = json.books.iter();
            for val in iter {
                let lib = val.1.as_object().unwrap()[LIBRARY].as_object().unwrap();
                assert!(lib["status"] == "OK" || lib["status"] == "Cache");
                reserveurls.insert(val.0.to_string(), lib["reserveurl"].as_str().unwrap().to_string());
            }
            return reserveurls;
        }

        println!(".");
        let dur = time::Duration::from_secs(2);
        thread::sleep(dur);
    }
}

fn isbn_to_reserveurl(isbns: Vec<String>) -> HashMap<String, String> {
    let mut ret = HashMap::new();
    for c in isbns.chunks(MAXREQ) {
        ret.extend(isbn_to_reserveurl_once(c.to_vec()));
    }
    return ret;
}

fn nwait_reserve(url: &String) -> String {
    let html_content = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&html_content);
    let selector = scraper::Selector::parse("em").unwrap();
    let mut ems = document.select(&selector);
    let _ = ems.next().unwrap();
    let em = ems.next().unwrap();
    return em.inner_html();
}

fn main() {
    let mut isbns = Vec::new();
    isbns.push("4152100702".to_string());
    isbns.push("4334102905".to_string());
    isbns.push("4478109680".to_string());

    let ret = isbn_to_reserveurl(isbns);
    for r in ret {
        let w = nwait_reserve(&r.1);
        println!("{}: ({}) {}", r.0, w, r.1);
    }
}
