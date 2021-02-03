use bunt::println;

pub(crate) struct Reporter {
    pub name: String,
    pub verb: String,
    pub verbose: i32
}

impl Reporter {

    pub fn done(self) {
        println!("{$green}Finished {}ing {}!{/$}", self.verb, self.name);
    }

    pub fn start(&self, location: &String) {
        println!("{$blue}{}ing {} to {}...{/$}", self.verb, self.name, location);
    }
    pub fn progress(&self, message: &String) {
        println!("{$blue+bold}{}:{/$} {}", self.name, message);
    }
    pub fn info(&self, message: &String) {
        if self.verbose > 0 {
            println!("{$blue}{}:{/$} {}", self.name, message);
        }
    }
    pub fn error(&self, message: &String) {
        println!("{$red+bold}Error in {}ing {}:{/$} {[red]}", self.verb, self.name, message);
    }

}