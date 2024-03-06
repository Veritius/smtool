use crate::{ExitCode, EXIT_CODE_SUCCESS};

static DNS_NAME: &str = "https://icanhazip.com/";

pub(super) fn run() -> ExitCode {
    match reqwest::blocking::get(DNS_NAME) {
        Ok(response) => {
            match response.text() {
                Ok(text) => {
                    print!("{text}");
                    return EXIT_CODE_SUCCESS
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