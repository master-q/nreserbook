extern crate reqwest;
use std::io;

const APPKEY: &str = env!("CALIL_APPKEY");

fn main() {
    let mut res = reqwest::blocking::get(
        "https://api.calil.jp/check?appkey".to_string()
            + APPKEY
            + "&isbn=4405012539,4492553908&systemid=Kanagawa_Yokohama&format=json&callback=no"
    ).unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}
