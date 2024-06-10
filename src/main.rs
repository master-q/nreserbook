extern crate reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{thread, time};

#[derive(Serialize, Deserialize)]
struct CalilCheck {
    session: String,
    r#continue: i32,
    books: Map<String, Value>,
}

const LIBRARY: &str = "Kanagawa_Yokohama";
const APPKEY: &str = env!("CALIL_APPKEY");

fn main() {
    let mut session: Option<String> = None;
    let mut cont;

    loop {
        // API specification
        // https://calil.jp/doc/api_ref.html
        let url = match session {
            None => "https://api.calil.jp/check?appkey".to_string()
                + APPKEY
                + "&isbn=4405012539,4492553908,4797395230&systemid="
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
            let iter = json.books.iter();
            for val in iter {
                let lib = val.1.as_object().unwrap()[LIBRARY].as_object().unwrap();
                println!("{}: s={}, r={}", val.0, lib["status"], lib["reserveurl"],);
                assert!(lib["status"] == "OK" || lib["status"] == "Cache");
            }
            break;
        }

        println!(".");
        let dur = time::Duration::from_secs(2);
        thread::sleep(dur);
    }
}
