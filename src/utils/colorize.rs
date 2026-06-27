pub trait Colorize {
    fn red(&self)    -> String;
    fn green(&self)  -> String;
    fn yellow(&self) -> String;
    fn cyan(&self)   -> String;
    fn bold(&self)   -> String;
    fn dimmed(&self) -> String;
}

#[cfg(not(target_os = "windows"))]
impl<T: AsRef<str>> Colorize for T {
    fn red(&self)    -> String { format!("\x1b[31m{}\x1b[0m", self.as_ref()) }
    fn green(&self)  -> String { format!("\x1b[32m{}\x1b[0m", self.as_ref()) }
    fn yellow(&self) -> String { format!("\x1b[33m{}\x1b[0m", self.as_ref()) }
    fn cyan(&self)   -> String { format!("\x1b[36m{}\x1b[0m", self.as_ref()) }
    fn bold(&self)   -> String { format!("\x1b[1m{}\x1b[0m", self.as_ref())  }
    fn dimmed(&self) -> String { format!("\x1b[2m{}\x1b[0m", self.as_ref()) }
}

#[cfg(target_os = "windows")]
impl<T: AsRef<str>> Colorize for T {
    fn red(&self)    -> String { self.as_ref().to_string() }
    fn green(&self)  -> String { self.as_ref().to_string() }
    fn yellow(&self) -> String { self.as_ref().to_string() }
    fn cyan(&self)   -> String { self.as_ref().to_string() }
    fn bold(&self)   -> String { self.as_ref().to_string() }
    fn dimmed(&self) -> String { self.as_ref().to_string() }
}