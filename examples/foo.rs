use fu::{bail, ensure, Result};

const MAX: i32 = 10;

fn example(value: i32) -> Result<()> {
    ensure!(value >= 0, "value must be non-negative");

    if value > MAX {
        bail!("value is larger than {}", MAX);
    }

    Ok(())
}

fn main() -> Result<()> {
    example(-1)
}

// Error: value must be non-negative    examples/foo.rs:[4:5]
