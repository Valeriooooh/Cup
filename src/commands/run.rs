use std::io::Read;

pub fn run_project() -> Result<(), std::io::Error> {
    let mut file = std::fs::File::open("Cup.toml")?;
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf)?;
    println!("{}", buf);
    println!("running project");
    Ok(())
}
