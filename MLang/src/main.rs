use std::env::args;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::io::Write;
use std::process::Command;
const MLANG_SRC: &[u8] = include_str!("./mlang.rs").as_bytes();
fn main() -> Result<()> {
    let mut args = args();
    args.next();
    if let Some(ref filename) = args.next() {
        let mut file = File::open(filename)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let ref out_src = filename.replace(".mlang", ".rs");
        let mut out_file = File::create(out_src)?;
        out_file.write(MLANG_SRC)?;
        out_file.write_all(
            format!(
                r#"
            fn main() {{
                mlang! {{
                    {buf}
                }};
            }}
        "#
            )
            .as_bytes(),
        )?;
        let command = Command::new("rustc").arg(out_src).output()?;
        fs::remove_file(out_src)?;
        let ref build = out_src.replace(".rs", "");
                
        #[cfg(unix)]
        {
          Command::new("strip")
            .arg("-s")
            .arg(build)
            .status()?;
        }
        if !command.status.success() {
            println!("{}", String::from_utf8_lossy(&command.stderr));

           
            return Ok(());
        }
        if let Some(out) = args.next() {
            let _ = fs::rename(build, out);

        }
    } else {
        println!("Usage: mlangc <filename>");
        return Ok(());
    }
    Ok(())
}
