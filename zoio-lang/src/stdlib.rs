pub mod io {
    use std::io::{stdin, stdout, Write};
    pub fn println(content: &str) {
        println!("{}", content);
    }
    pub fn print(content: &str) {
        print!("{}", content);
    }
    pub fn read_line() -> String {
        let mut s = String::new();
        let _ = stdin().read_line(&mut s);
        s.replace("\n", "").replace("\r", "")
    }
    pub fn flush_stdout() {
        let _ = stdout().flush();
    }
}
