use colored::{ColoredString, Colorize};
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) struct Logger {
    provider_name: String,
}

impl Logger {
    pub fn new(name: &str) -> Self {
        Logger {
            provider_name: name.bold().to_string(),
        }
    }

    pub fn info(&self, message: &str) {
        self.print(" INFO".bright_green(), message);
    }

    pub fn warn(&self, message: &str) {
        self.print(" WARN".truecolor(250, 234, 0), message);
    }

    pub fn debug(&self, message: &str) {
        self.print("DEBUG".cyan(), message);
    }

    pub fn error(&self, message: &str) {
        self.print("ERROR".bright_red(), message);
    }

    fn print(&self, prefix: ColoredString, message: &str) {
        println!(
            "{}{}{}{} - {}",
            self.get_template(),
            "[".bright_black(),
            prefix,
            "]".bright_black(),
            message
        );
    }

    fn get_template(&self) -> String {
        format!(
            "{}{}{}",
            format!(
                "{}{}{}",
                "[".bright_black(),
                self.get_time().bright_white(),
                "]".bright_black()
            ),
            format!(
                "{}{}{}",
                "[".bright_black(),
                self.provider_name.cyan(),
                "]".bright_black()
            ),
            format!(
                "{}{}{}",
                "[".bright_black(),
                format!("{:?}", std::thread::current().id()).truecolor(247, 12, 248),
                "]".bright_black()
            )
        )
    }

    fn get_time(&self) -> String {
        let now = SystemTime::now();
        let timestamp = now.duration_since(UNIX_EPOCH).unwrap().as_secs();

        let hours = ((timestamp + 7200) / 3600) % 24;
        let minutes = (timestamp / 60) % 60;
        let seconds = timestamp % 60;

        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}
