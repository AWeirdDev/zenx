use colored::Colorize;

#[allow(dead_code)]
pub enum Logging {
    Info,
    Debug,
    Trace,
}

impl Logging {
    #[allow(dead_code)]
    pub fn print(&self, c: String) {
        let zenx = "zenx".on_black().white().bold();

        match self {
            Logging::Info => println!("{} {} {}", zenx, "info".cyan().bold(), c),
            Logging::Debug => println!("{} {} {}", zenx, "debug".dimmed(), c),
            Logging::Trace => println!("{} {} {}", zenx, "trace".dimmed(), c),
        }
    }
}
