pub struct Contact {
    pub name: String,
    pub phone: String,
    pub nickname: String,
    pub bookmarked: bool,
}

impl Contact {
    pub fn new(name: String, phone: String, nickname: String) -> Self {
        Self {
            name,
            phone,
            nickname,
            bookmarked: false,
        }
    }

    pub fn display(&self) {
        println!("--- Contact Info ---");
        println!("NAME    : {}", self.name);
        println!("PHONE   : {}", self.phone);
        println!("NICKNAME: {}", self.nickname);
        if self.bookmarked {
            println!("{} IS BOOKMARKED.", self.name);
        } else {
            println!("{} IS NOT BOOKMARKED.", self.name);
        }
        println!("-------");
    }
}
