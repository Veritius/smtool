use crate::ExitCode;

static DNS_NAME: &str = "https://icanhazip.com/";

pub(super) fn run() -> ExitCode {
    match reqwest::blocking::get(DNS_NAME) {
        Ok(response) => {
            match response.text() {
                Ok(text) => {
                    print!("{text}");
                    return 0
                },
                Err(_) => {
                    print!("Invalid HTTP response");
                    return -2
                },
            }
        },
        Err(_) => {
            print!("Couldn't reach the page");
            return -1
        },
    }
}