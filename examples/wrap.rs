// use fu::{Result, Wrap};

// fn example() -> Result<()> {
//     let res = std::fs::File::open("abc").wrap("wrapped")?;
//     // let _ = res.inspect_err(|e| println!("{}", e.full_chain()));
//     Ok(())
// }

fn main() {
    let r: std::result::Result<(), Box<dyn std::error::Error>> = Err("hello".into());
    let e = r.unwrap_err();
    dbg!(e.source());
}
