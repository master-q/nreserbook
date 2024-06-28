extern crate reqwest;
use std::collections::HashMap;
use std::env;
use std::{thread, time};
use std::fs::read_to_string;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize, Deserialize)]
struct CalilCheck {
    session: String,
    r#continue: i32,
    books: Map<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Calil {
    None,
    Error,
    Reserveurl(String),
}

const APPKEY: &str = env!("CALIL_APPKEY");
const MAXREQ: usize = 10; // xxx Do not support 100?
const LIBRARY: &str = "Kanagawa_Yokohama";
const JSONFILE: &str = "bookmap.json";

// private
fn isbn_to_reserveurl_once(isbns: Vec<String>) -> HashMap<String, Calil> {
    assert!(isbns.len() <= MAXREQ);

    let mut session: Option<String> = None;
    let mut cont;

    eprint!("+");
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
            Some(ref s) => "https://api.calil.jp/check?appkey".to_string()
                + APPKEY
                + "&session="
                + &s
                + "&format=json&callback=no"
        };
        let res = reqwest::blocking::get(url).unwrap().text().unwrap();
        let Ok(json) = serde_json::from_str::<CalilCheck>(&res) else {
            eprint!("E");
            continue;
        };
        session = Some(json.session);
        cont = json.r#continue;

        if cont == 0 {
            let mut reserveurls = HashMap::new();
            let iter = json.books.iter();
            for val in iter {
                let lib = val.1.as_object().unwrap()[LIBRARY].as_object().unwrap();
		if lib["status"] == "OK" || lib["status"] == "Cache" {
		    let r = lib["reserveurl"].as_str().unwrap();
		    if r.is_empty() {
			reserveurls.insert(val.0.to_string(), Calil::None);
		    } else {
			reserveurls.insert(val.0.to_string(), Calil::Reserveurl(r.to_string()));
		    }
		} else {
		    reserveurls.insert(val.0.to_string(), Calil::Error);
		}
            }
            return reserveurls;
        }

        eprint!(".");
        let dur = time::Duration::from_secs(2);
        thread::sleep(dur);
    }
}

fn isbn_to_reserveurl(bookmap: &mut HashMap<String, Calil>) {
    let mut isbns: Vec<String> = Vec::new();
    for b in &mut *bookmap {
	if b.1 == &mut Calil::None || b.1 == &mut Calil::Error {
	    isbns.push(b.0.to_string())
	};
    }
    for c in isbns.chunks(MAXREQ) {
        bookmap.extend(isbn_to_reserveurl_once(c.to_vec()));
        save_bookmap(bookmap);
    }
}

fn nwait_reserve(calil: &Calil) -> String {
    match calil {
	Calil::None => "-".to_string(),
	Calil::Error => "E".to_string(),
	Calil::Reserveurl(url) => {
	    let html_content = reqwest::blocking::get(url).unwrap().text().unwrap();
	    let document = scraper::Html::parse_document(&html_content);
	    let selector = scraper::Selector::parse("em").unwrap();
	    let mut ems = document.select(&selector);
	    let _ = ems.next().unwrap();
	    let em = ems.next().unwrap();
	    em.inner_html()
	},
    }
}

fn to_isbn(input: &str) -> Option<String> {
    let re = Regex::new(r"^\* .+https://.+amazon.*/([\dX]+)").unwrap();
    return match re.captures(input) {
        Some(caps) => Some(caps[1].to_string()),
        None => None,
    }
}

fn save_bookmap(bookmap: &HashMap<String, Calil>) {
    let j = serde_json::to_string(&bookmap).unwrap();
    std::fs::write(JSONFILE, j).unwrap();
}

fn load_bookmap() -> HashMap<String, Calil> {
    match read_to_string(JSONFILE) {
	Err(_) => {
	    let b = HashMap::new();
	    save_bookmap(&b);
	    b
	},
	Ok(json) => match serde_json::from_str(&json) {
	    Err(_) => {
		let b = HashMap::new();
		save_bookmap(&b);
		b
	    },
	    Ok(b) => b,
	}
    }
}

fn remove_bookmap() {
    let _ = std::fs::remove_file(JSONFILE);
}

fn main() {
    let mut bookmap = load_bookmap();

    let args: Vec<String> = env::args().collect();

    if args[1] == "clean" {
	remove_bookmap();
	return;
    }

    let lines = read_to_string(&args[2]).expect("File not found.");
    if args[1] == "update" {
	for line in lines.lines() {
	    if let Some(isbn) = to_isbn(line) {
		if !bookmap.contains_key(&isbn) {
		    bookmap.insert(isbn, Calil::None);
		}
	    }
	}

	isbn_to_reserveurl(&mut bookmap);
	save_bookmap(&bookmap);
	eprintln!("");
    } else if args[1] == "show" {
	for line in lines.lines() {
	    let mut w = String::from("?");
	    if let Some(isbn) = to_isbn(line) {
		if bookmap.contains_key(&isbn) {
		    w = nwait_reserve(bookmap.get(&isbn).unwrap());
		};
		println!("[{}] {}", w, line)
	    };
	}
    }

}
