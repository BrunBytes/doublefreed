use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    doublefreed_wrangler::wrangle_process();
    Ok(())
}
