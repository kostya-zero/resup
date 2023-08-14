pub struct Term;
impl Term {
    pub fn message(msg: &str) {
        println!("{}", msg);
    }

    pub fn error(msg: &str) {
        println!("ERROR: {}", msg);
    }

    pub fn warn(msg: &str) {
        println!("WARN: {}", msg);
    }
}
