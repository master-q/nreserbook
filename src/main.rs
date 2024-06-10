extern crate reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize)]
struct CalilCheck {
    session: String,
    r#continue: i32,
    books: Map<String, Value>,
}

const APPKEY: &str = env!("CALIL_APPKEY");

fn main() {
    let res = reqwest::blocking::get(
        "https://api.calil.jp/check?appkey".to_string()
            + APPKEY
            + "&isbn=4405012539,4492553908&systemid=Kanagawa_Yokohama&format=json&callback=no"
    ).unwrap().text().unwrap();
    let json: CalilCheck = serde_json::from_str(&res).unwrap();
    println!("{}", res);

    println!("session={}, c={}", json.session, json.r#continue);
    let iter = json.books.iter();
    for val in iter {
        let yokohama = val.1.as_object().unwrap()["Kanagawa_Yokohama"].as_object().unwrap();
        println!("{}: s={}, r={}", val.0, yokohama["status"], yokohama["reserveurl"],);
    }
}
