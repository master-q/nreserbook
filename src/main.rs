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

const LIBRARY: &str = "Kanagawa_Yokohama";
const APPKEY: &str = env!("CALIL_APPKEY");

fn isbn_to_reserveurl_once(isbns: Vec<String>) -> HashMap<String, String> {
    assert!(isbns.len() <= 100);

    let mut session: Option<String> = None;
    let mut cont;

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
                reserveurls.insert(val.0.to_string(), lib["reserveurl"].to_string());
            }
            return reserveurls;
        }

        println!(".");
        let dur = time::Duration::from_secs(2);
        thread::sleep(dur);
    }
}

fn main() {
    let mut isbns = Vec::new();
    isbns.push("4405012539".to_string());
    isbns.push("4492553908".to_string());
    isbns.push("4797395230".to_string());

    let ret = isbn_to_reserveurl_once(isbns);
    for r in ret {
        println!("{}: {}", r.0, r.1);
    }
}
