mod constants;
mod conversions;
mod month_end;
mod utils;
use chrono::NaiveDate;
use constants::NEPALI_MONTH_NAMES;
use conversions::{ad_to_bs, bs_to_ad};
use std::error::Error;
use utils::get_digit_in_devanagari;

fn main() -> Result<(), Box<dyn Error>> {
    let ad_date = NaiveDate::from_ymd_opt(2025, 3, 1).unwrap();
    let bs_date = ad_to_bs(ad_date)?;

    // Print the date in Nepali
    println!(
        "BS Date: {} {} {}",
        get_digit_in_devanagari(bs_date.day as u16),
        NEPALI_MONTH_NAMES[(bs_date.month - 1) as usize],
        get_digit_in_devanagari(bs_date.year as u16)
    );

    // Convert back from BS to AD and print the result
    let converted_ad_date = bs_to_ad(bs_date)?;
    println!("Converted AD Date: {}", converted_ad_date);

    Ok(())
}
