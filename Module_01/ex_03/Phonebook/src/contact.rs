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
        println!("");
        println!("--- Contact Info ---");
        println!("NAME    : {}", self.name);
        println!("PHONE   : {}", self.phone);
        println!("NICKNAME: {}", self.nickname);
        if self.is_bookmarked {
            println!("{} is Bookmarked.", self.name);
        } else {
            println!("{} is not Bookmarked.", self.name);
        }
        println!("--------------------");
        println!("");
    }
}
