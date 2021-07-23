use std::io::{stdin, stdout, Error, Write};
pub fn input(question: Option<&str>) -> Result<String, Error> {
    let mut result = String::new();
    if let Some(question) = question {
        print!("{}", question);
        stdout().flush()?;
    }
    stdin().read_line(&mut result)?;
    return Ok(result.replace("\n", "").replace("\r", ""));
}
pub fn eval(s: &str) -> Result<f64, fasteval::Error> {
    let mut ns = fasteval::EmptyNamespace;
    fasteval::ez_eval(s, &mut ns)
}
