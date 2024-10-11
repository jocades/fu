use fu::Result;

fn example() -> Result<()> {
    let _ = std::fs::File::open("abc")?;
    Ok(())
}

fn main() -> Result<()> {
    example()
}
