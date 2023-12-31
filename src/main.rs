use ifunny_auth::basic::BasicToken;
use ifunny_auth::client::login;
use ifunny_auth::error::Error;
use std::{thread, time::Duration};
use text_io::read;


#[tokio::main]
async fn main() {
    let basic = BasicToken::generate();
    println!("\nPlease enter your username");
    print!("\\> ");
    let username: String = read!();
    println!("\nPlease enter your password");
    let password = rpassword::prompt_password("\\> ").unwrap();
    println!("\nPriming basic, this takes 10 seconds...");
    let _ = login(&username, &password, &basic).await;
    thread::sleep(Duration::from_secs(10));

    /// Checks the error, prints it out for the user.
    /// Returns a bool indicating whether it is a captcha Error or not.
    pub fn handle_error(err: Error, primed: bool) -> bool {
        match err {
            ifunny_auth::error::Error::BoxError(e) => println!("{e}"),
            ifunny_auth::error::Error::ReqwestError(e) => println!("{e}"),
            ifunny_auth::error::Error::JsonError(e) => println!("{e}"),
            ifunny_auth::error::Error::CaptchaRequired { captcha_url } => {
                if !primed {
                    open::that(captcha_url).unwrap();
                    println!("\nWhen the Captcha is done, please type anything and press enter to continue.");
                    print!("\\>");
                    let _: String = read!();
                }
                return true;
            }
            ifunny_auth::error::Error::UnknownError => {
                println!("\nUnknown error occurred because the program reached its bounds.")
            }
        };
        false
    }

    match login(&username, &password, &basic).await {
        Ok(bearer) => {
            println!("\nPrimed Basic Token:\n{basic}");
            println!("\nBearer Token: \n{bearer}")
        }
        Err(err) => {
            let is_captcha_error = handle_error(err, false);
            if is_captcha_error {
                match login(&username, &password, &basic).await {
                    Ok(bearer) => {
                        println!("\nPrimed Basic Token:\n{basic}");
                        println!("\nBearer Token: \n{bearer}")
                    }
                    Err(err) => {
                        handle_error(err, true);
                    }
                }
            }
        }
    }
}
