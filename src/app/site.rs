pub struct Site {
    pub msg: String
}

impl Site {
    pub fn log(&self) {
        println!("{}", self.msg);
    }
}