use chrono::NaiveDate;
use csv::Reader;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct BSCalendar {
    Baisakh: u8,
    Jestha: u8,
    Ashar: u8,
    Shrawan: u8,
    Bhadra: u8,
    Asoj: u8,
    Kartik: u8,
    Mangsir: u8,
    Poush: u8,
    Magh: u8,
    Falgun: u8,
    Chaitra: u8,
    Days: u16,
}

struct BSDate {
    year: u16,
    month: u8,
    day: u8,
}

static SHORT_MONTH_NAMES: [&str; 12] = [
    "Bai", "Jes", "Ash", "Shr", "Bha", "Aso", "Kar", "Man", "Pou", "Mag", "Fal", "Chai",
];
static FULL_MONTH_NAMES: [&str; 12] = [
    "Baisakh", "Jestha", "Ashar", "Shrawan", "Bhadra", "Asoj", "Kartik", "Mangsir", "Poush",
    "Magh", "Falgun", "Chaitra",
];
static NEPALI_MONTH_NAMES: [&str; 12] = [
    "बैशाख",
    "जेष्ठ",
    "आषाढ",
    "श्रावण",
    "भाद्र",
    "आसोज",
    "कार्तिक",
    "मंसिर",
    "पौष",
    "माघ",
    "फाल्गुन",
    "चैत्र",
];
static NEPALI_DAY_NAMES: [&str; 7] = [
    "आइतबार",
    "सोमबार",
    "मंगलबार",
    "बुधबार",
    "बिहिबार",
    "शुक्रबार",
    "शनिबार",
];
static NEPALI_DAY_NAMES_SHORT: [&str; 7] = ["आइत", "सोम", "मंगल", "बुध", "बिहि", "शुक्र", "शनि"];
static NEPALI_DIGITS: [&str; 10] = ["०", "१", "२", "३", "४", "५", "६", "७", "८", "९"];

static MIN_NEPALI_DATE: (u16, u8, u8) = (1975, 1, 1);
static MAX_NEPALI_DATE: (u16, u8, u8) = (2100, 12, 30);

static START_ENGLISH_DATE: NaiveDate = NaiveDate::from_ymd_opt(1918, 4, 13).unwrap();
static CALENDAR_PATH: &str = "calendar_bs.csv";

fn get_bs_calendar_rows(path: &str) -> Result<Vec<BSCalendar>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;

    let rows: Vec<BSCalendar> = rdr
        .deserialize()
        .skip(0) // Skip the header
        .collect::<Result<Vec<BSCalendar>, csv::Error>>() // Collect the rows
        .unwrap(); // Unwrap the result

    Ok(rows)
}

fn get_digit_in_nepali(digit: u16) -> String {
    let mut nepali_digit = String::new();
    for d in digit.to_string().chars() {
        nepali_digit.push_str(NEPALI_DIGITS[d.to_digit(10).unwrap() as usize]);
    }
    nepali_digit
}

fn main() -> Result<(), Box<dyn Error>> {
    let rows = get_bs_calendar_rows(CALENDAR_PATH)?;
    let ad_date = NaiveDate::from_ymd_opt(2025, 3, 1).unwrap();

    // Algorithm to convert AD to BS
    // 1. Check if the date is within the range
    // 2. Check if the date is greater than 1918-04-13
    // 3. Calculate the difference between the date and 1918-04-13
    // 4. Calculate the number of days in between
    // 5. Add that number of days to the start of the BS calendar
    // 6. Iterate over the rows and find the year

    if ad_date < START_ENGLISH_DATE {
        println!("Date should be greater than 1918-04-13");
        return Ok(()); // Return early
    }
    let mut days_diff = ad_date.signed_duration_since(START_ENGLISH_DATE).num_days();
    println!("Days diff: {}", days_diff);

    let mut bs_date = BSDate {
        year: 1975,
        month: 1,
        day: 1,
    };

    for row in rows.iter() {
        if days_diff > 366 {
            bs_date.year += 1;
            days_diff -= row.Days as i64;
        } else {
            let month_tuple: [u8; 12] = [
                row.Baisakh,
                row.Jestha,
                row.Ashar,
                row.Shrawan,
                row.Bhadra,
                row.Asoj,
                row.Kartik,
                row.Mangsir,
                row.Poush,
                row.Magh,
                row.Falgun,
                row.Chaitra,
            ];

            for (_, days) in month_tuple.iter().enumerate() {
                if days_diff > *days as i64 {
                    days_diff -= *days as i64;
                    bs_date.month += 1;
                } else {
                    bs_date.day = (days_diff + 1) as u8;
                    break;
                }
            }
        }
    }

    println!(
        "BS Date: {} {} {}",
        bs_date.day, bs_date.month, bs_date.year
    );

    // Print the date in Nepali
    println!(
        "BS Date: {} {} {}",
        get_digit_in_nepali(bs_date.day as u16),
        NEPALI_MONTH_NAMES[(bs_date.month - 1) as usize],
        get_digit_in_nepali(bs_date.year as u16)
    );
    Ok(())
}
