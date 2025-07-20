use colored::*;

pub struct Contact {
    pub name: String,
    pub phone: String,
    pub nickname: String,
    pub is_bookmarked: bool,
}

impl Contact {
    pub fn new(name: String, phone: String, nickname: String) -> Self {
        Self {
            name,
            phone,
            nickname,
            is_bookmarked: false,
        }
    }

    pub fn display(&self) {
        println!();
        println!("{}", "--- Contact Info ---".bold().white());

        println!(
            "{} {}",
            "👤 NAME    :".bold().cyan(),
            self.name.bold().cyan()
        );

        println!(
            "{} {}",
            "📞 PHONE   :".bold().green(),
            self.phone.bold().green()
        );

        println!(
            "{} {}",
            "🏷️  NICKNAME :".bold().magenta(),
            self.nickname.bold().magenta()
        );

        if self.is_bookmarked {
            println!(
                "{}",
                format!("⭐ {} is Bookmarked.", self.name).bold().yellow()
            );
        } else {
            println!(
                "{}",
                format!("🔖 {} is not Bookmarked.", self.name)
                    .bold()
                    .white()
            );
        }

        println!("{}", "--------------------".bold().white());
        println!();
    }
}
