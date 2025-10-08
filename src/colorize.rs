pub trait Colorize {
    fn error(&self) -> String;
    fn success(&self) -> String;
    fn warning(&self) -> String;
    fn info(&self) -> String;

    fn bold(&self) -> String;
}

impl<T: AsRef<str>> Colorize for T {
    fn error(&self) -> String {
        format!("\x1b[31m{}\x1b[0m", self.as_ref())
    }
    fn success(&self) -> String {
        format!("\x1b[32m{}\x1b[0m", self.as_ref())
    }
    fn warning(&self) -> String {
        format!("\x1b[33m{}\x1b[0m", self.as_ref())
    }
    fn info(&self) -> String {
        format!("\x1b[36m{}\x1b[0m", self.as_ref())
    }
    
    fn bold(&self) -> String {
        format!("\x1b[1m{}\x1b[0m", self.as_ref())
    }
}