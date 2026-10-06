use std::fmt::Display;

/// Prints progress messages for one deploy entry.
pub struct Reporter<'a> {
    name: &'a str,
    verb: &'static str,
    verbose: u8,
}

impl<'a> Reporter<'a> {
    pub fn new(name: &'a str, verb: &'static str, verbose: u8) -> Self {
        Reporter {
            name,
            verb,
            verbose,
        }
    }

    pub fn start(&self, location: &str) {
        bunt::println!(
            "{$blue}{}ing {} to {}...{/$}",
            self.verb,
            self.name,
            location
        );
    }

    pub fn progress(&self, message: impl Display) {
        bunt::println!("{$blue+bold}{}:{/$} {}", self.name, message);
    }

    pub fn info(&self, message: impl Display) {
        if self.verbose > 0 {
            bunt::println!("{$blue}{}:{/$} {}", self.name, message);
        }
    }

    pub fn done(&self) {
        bunt::println!("{$green}Finished {}ing {}!{/$}", self.verb, self.name);
    }

    pub fn error(&self, message: impl Display) {
        bunt::eprintln!(
            "{$red+bold}Error in {}ing {}:{/$} {[red]}",
            self.verb,
            self.name,
            message
        );
    }
}
