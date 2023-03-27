pub struct Term;
impl Term {
    pub fn info(msg: &str) {
        println!("\x1b[1m\x1b[92m=>\x1b[0m \x1b[1m{}\x1b[0m", msg);
    }
}
