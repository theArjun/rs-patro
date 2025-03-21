use crate::constants::START_ENGLISH_DATE;
use crate::month_end::BS_CALENDAR_MONTH_ENDS;
use crate::utils::get_digit_in_devanagari;
use chrono::NaiveDate;
use std::error::Error;

pub struct BSDate {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl BSDate {
    pub fn new(year: u16, month: u8, day: u8) -> BSDate {
        BSDate { year, month, day }
    }

    pub fn to_ad(&self) -> Result<NaiveDate, Box<dyn Error>> {
        // Algorithm to convert BS to AD
        // 1. Check if the BS year is within the range
        // 2. Calculate the number of days from the start of the BS calendar to the given BS date
        // 3. Add that number of days to the start of the AD calendar

        if self.year < 1975 || self.year > 2100 {
            return Err("Year should be between 1975 and 2100".into());
        }

        let mut days_diff = 0;
        for year in 1975..self.year {
            days_diff += BS_CALENDAR_MONTH_ENDS[(year - 1975) as usize][13] as i64;
        }

        let month_tuple: &[u16] = &BS_CALENDAR_MONTH_ENDS[(self.year - 1975) as usize][1..13];
        for month in 0..(self.month - 1) as usize {
            days_diff += month_tuple[month] as i64;
        }

        days_diff += (self.day - 1) as i64;
        let ad_date = START_ENGLISH_DATE + chrono::Duration::days(days_diff);
        Ok(ad_date)
    }

    pub fn from_ad(ad_date: NaiveDate) -> Result<BSDate, Box<dyn Error>> {
        // Algorithm to convert AD to BS
        // 1. Check if the date is within the range
        // 2. Check if the date is greater than 1918-04-13
        // 3. Calculate the difference between the date and 1918-04-13
        // 4. Calculate the number of days in between
        // 5. Add that number of days to the start of the BS calendar
        // 6. Iterate over the rows and find the year

        if ad_date < START_ENGLISH_DATE {
            return Err("Date should be greater than 1918-04-13".into());
        }

        let mut bs_date = BSDate {
            year: 1975,
            month: 1,
            day: 1,
        };

        let mut days_diff = ad_date.signed_duration_since(START_ENGLISH_DATE).num_days();

        for row in BS_CALENDAR_MONTH_ENDS.iter() {
            if days_diff > 366 {
                bs_date.year += 1;
                days_diff -= row[row.len() - 1] as i64;
            } else {
                let month_tuple: &[u16] = &row[1..13];
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
        Ok(bs_date)
    }
}

impl std::fmt::Display for BSDate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}-{}-{}",
            get_digit_in_devanagari(self.year),
            get_digit_in_devanagari(self.month as u16),
            get_digit_in_devanagari(self.day as u16)
        )
    }
}
