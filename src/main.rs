mod constants;
mod conversions;
mod month_end;
mod utils;
use conversions::{BSDate, bs_to_ad};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let bs_date = BSDate::new(2081, 11, 18);
    println!("BS Date: {}", bs_date);

    // Convert back from BS to AD and print the result
    let converted_ad_date = bs_to_ad(bs_date)?;
    println!("Converted AD Date: {}", converted_ad_date);

    Ok(())
}
