mod constants;
mod conversions;
mod month_end;
mod utils;
use chrono::NaiveDate;
use conversions::BSDate;

fn main() {
    let ad_date = NaiveDate::from_ymd_opt(2025,3,2).unwrap();
    let bs_date = BSDate::from_ad(ad_date).unwrap();
    println!("AD Date: {}", bs_date);
}
