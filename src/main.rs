//!# DigitShield crate
//! A command-line password generator built with Rust, offering a swift and effortless solution for creating passwords of varying strengths. Cure your password creation woes with a single command!
use ansi_term::Color::Blue;
use clap::Parser;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use colored::Colorize;
use notify_rust::Notification;
use xpwd::*;

/// Command line arguments structure.
#[derive(Debug, Parser)]
#[command(version, author="Code0408", long_about = None)] // Metadata for the CLI tool
struct Args {
    #[arg(
        short = 'l',
        long = "len",
        default_value_t = 8,
        help = "Length of password"
    )]
    len: usize,

    #[arg(
        short = 'c',
        long = "complex",
        default_value_t = String::from("m"),
        help="Complexity of the password"
    )]
    complex: String,

    #[arg(
        short = 'p',
        long = "password",
        help = "Check strength of your password"
    )]
    password: Option<String>,
}

/// Prints program information with styling.
fn print_infos() {
    println!(
        "{}",
        Blue.paint(
            r#" 
               _______           ______  
|\     /|     (  ____ )|\     /|(  __  \ 
( \   / )     | (    )|| )   ( || (  \  )
 \ (_) /_____ | (____)|| | _ | || |   ) |
  ) _ ((_____)|  _____)| |( )| || |   | |
 / ( ) \      | (      | || || || |   ) |
( /   \ )     | )      | () () || (__/  )
|/     \|     |/       (_______)(______/         
        "#
        )
    );
}

fn main() {
    print_infos();
    let args = Args::parse();

    match args.password {
        Some(ref password) => print_password_strength(password),
        None => {
            let pwd = gen_password(args.len, &args.complex);

            println!("Your Password:\t{}", pwd.magenta());

            println!("\n\n");

            // Copy the generated password to the clipboard
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(pwd.to_owned()).unwrap();

            if let Err(e) = Notification::new()
                //.summary("")
                .body("The password has been copied to the clipboard!")
                .show()
            {
                eprintln!("err:{}", e);
            }
        }
    }
}
