//!# DigitShield crate
//! A command-line password generator built with Rust, offering a swift and effortless solution for creating passwords of varying strengths. Cure your password creation woes with a single command!
use ansi_term::Color::Blue;
use clap::Parser;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use notify_rust::Notification;
use xpwd::*;

#[derive(Parser)]
#[clap(name = "xpwd")]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
#[command(
    version,
    about = "Fast, secure, and universal password generator.",
    author = "Code0408"
)]
enum Command {
    #[clap(about = "Generate a secure password of specified length and strength.")]
    Pwd(PasswordArgs),
    #[clap(about = "Check the strength of the password you entered.")]
    Str(StrengthArgs),
    #[clap(about = "Generate a random short passphrase password based on a dictionary.")]
    Pas(PassphraseArgs),
}

#[derive(Parser)]
struct PasswordArgs {
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
}

#[derive(Parser)]
struct StrengthArgs {
    #[arg(
        short = 'p',
        long = "password",
        help = "Check strength of your password"
    )]
    password: String,
}

#[derive(Parser)]
struct PassphraseArgs {
    #[arg(short = 'w', long = "world", default_value_t = 3)]
    num_words: usize,
    #[arg(
        short = 'd',
        long = "dictionary",
        help = "Specify the path to your phrase dictionary file."
    )]
    dictionary: String,
}

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

    let opts = Opts::parse();

    match opts.command {
        Command::Pwd(PasswordArgs) => {
            let pwd = gen_password(PasswordArgs.len, &PasswordArgs.complex);
            let cmatch = match PasswordArgs.complex.as_str() {
                "s" => "simple".to_string(),
                "m" => "medium".to_string(),
                "c" => "complex".to_string(),
                _ => "unknown".to_string(),
            };

            let datas = vec![(PasswordArgs.len.to_string(), cmatch, pwd.clone())];

            print_data_tables(&datas);
            print_password_strength(pwd.as_str());
            println!("\n\n");

            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(pwd.to_owned()).unwrap();

            if let Err(e) = Notification::new()
                .body("The password has benn copied to the clipboard!")
                .show()
            {
                eprintln!("err:{}", e);
            }
        }
        Command::Str(StrengthArgs) => print_password_strength(&StrengthArgs.password),

        Command::Pas(PassphraseArgs) => {
            let dic = load_dictionary(&PassphraseArgs.dictionary);
            let res = generate_random_passphrase(&dic, PassphraseArgs.num_words);

            print_data_tables(&vec![(
                PassphraseArgs.num_words.to_string(),
                PassphraseArgs.dictionary.to_string(),
                res.to_string(),
            )]);
        }
    }
}
