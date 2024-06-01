<div align="center">
<img src="https://images.waer.ltd/notes/20240601113455.png" width="140px" />

[![Crates.io](https://img.shields.io/crates/d/quick_pswd.svg)](https://crates.io/crates/xpwd)
[![License](https://img.shields.io/github/license/08820048/xpwd)](https://github.com/08820048/xpwd/blob/master/LICENSE)
[![rustc 1.77.0](https://img.shields.io/badge/rust-1.77.0-orange.svg)](https://img.shields.io/badge/rust-1.77.0-orange.svg)
[![Documentation](https://docs.rs/console/badge.svg)](https://docs.rs/xpwd)
[![GitHub stars](https://img.shields.io/github/stars/08820048/xpwd)](https://github.com/08820048/xpwd/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/08820048/xpwd)](https://github.com/08820048/xpwd/network/members)
[![GitHub issues](https://img.shields.io/github/issues/08820048/xpwd)](https://github.com/08820048/xpwd/issues)
[![Contributors](https://img.shields.io/github/contributors/08820048/xpwd?style=flat-square)](https://github.com/08820048/xpwd/graphs/contributors)

(**In order to provide a more efficient and convenient user experience, the original project quick_pswd (https://crates.io/crates/quick_pswd) has officially been renamed to xpwd.**)
A command-line password generator built with Rust, offering a swift and effortless solution for creating passwords of varying strengths. Cure your password creation woes with a single command!

</div>

## Installation

> More installation methods are coming soon...

### **Install using Cargo**

Please ensure that your operating system has Rust and the Cargo development environment properly configured.

```powershell
cargo install xpwd --version=1.0.0 // When installing, please select the version of your choice.
```

------------------

## CLI  Usage

```powershell
$ xpwd -h
               _______           ______
|\     /|     (  ____ )|\     /|(  __  \
( \   / )     | (    )|| )   ( || (  \  )
 \ (_) /_____ | (____)|| | _ | || |   ) |
  ) _ ((_____)|  _____)| |( )| || |   | |
 / ( ) \      | (      | || || || |   ) |
( /   \ )     | )      | () () || (__/  )
|/     \|     |/       (_______)(______/

Command line arguments structure

Options:
  -l, --len <LEN>            Length of password [default: 8]
  -c, --complex <COMPLEX>    Complexity of the password [default: m]
  -p, --password <PASSWORD>  Check strength of your password
  -h, --help                 Print help
  -V, --version              Print version


$ xpwd -l 8 -c s
 j1618a59

$ xpwd -l 16 -c m
Cio6PYtg30mLYtHC

$ xpwd -l 18 -c c
yd;mW)[1sW5q3p)6.Y

```



> **The generated password will be automatically copied to the clipboard by default, allowing you to directly paste and use it.**

---

## Futures

> Here are the features and improvements we plan to add to the tool in the future. If you have any suggestions or ideas, feel free to share!

1. **Quick Password Generation**
   - ✅ Rapidly generates random passwords of specified lengths and strengths.
2. **Password Strength Check**
   - ✅ Assesses the strength of user-inputted passwords, providing visual feedback.
3. **Customizable Password Policy**
   - ⌛ Enables users to customize password generation rules, such as mandating the inclusion of uppercase letters, lowercase letters, numbers, special characters, and their minimum occurrences.
4. **Password History Management**
   - ⌛ Offers a secure means for users to store and manage their previously generated passwords (with emphasis on encrypted storage), including features to flag frequently used passwords and search for specific ones.
5. **Password Expiration Reminder**
   - ⌛ Implements a notification system that alerts users, based on their settings (e.g., every 90 days), when to change passwords for specific websites or applications.
6. **Password Synchronization & Backup**
   - ⌛ Provides cloud synchronization functionality, allowing users to sync their password database across multiple devices, ensuring secure access while facilitating cross-platform use.
7. **Security Assessment Report**
   - ⌛ Generates detailed reports on password security, analyzing the distribution of password strengths and reuse within the user's database, offering improvement suggestions.
8. **Password Leak Check**
   - ⌛ Integrates with an `API` (like `Have I Been Pwned`) to check if user-supplied passwords have been exposed in known data breaches, enhancing user awareness of password security.
9. **Random Passphrase Generation**
   - ⌛ In addition to traditional random strings, includes a dictionary-based passphrase generator that produces longer but more memorable passwords, such as “`CorrectHorseBatteryStaple`”.
10. **Multilingual Password Support**
    - ⌛ Expands character set support to allow for passwords containing non-English characters, catering to international users with specific needs.
11. **Graphical User Interface**
    - ⌛ Considers developing a graphical user interface (`GUI`) to enhance the user experience, particularly for those unfamiliar with command-line operations.
12. **Plug-in Based Development**
    - ⌛ Develops related **plugins** to enrich usage scenarios, including browser plugins, `IDE` integrations, and apps for multiple platforms.
13. **Educational Module**
    - ⌛Incorporates educational content on password security, covering common password cracking methods and guidelines for creating and managing strong passwords, to boost user awareness.

----

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/08820048/xpwd/blob/master/LICENSE) file for details.
