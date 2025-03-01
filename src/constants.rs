use chrono::NaiveDate;

pub static START_ENGLISH_DATE: NaiveDate = NaiveDate::from_ymd_opt(1918, 4, 13).unwrap();
pub static NEPALI_DIGITS: [&str; 10] = ["०", "१", "२", "३", "४", "५", "६", "७", "८", "९"];
pub static NEPALI_MONTH_NAMES: [&str; 12] = [
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
// pub static SHORT_MONTH_NAMES: [&str; 12] = [
//     "Bai", "Jes", "Ash", "Shr", "Bha", "Aso", "Kar", "Man", "Pou", "Mag", "Fal", "Chai",
// ];
// pub static FULL_MONTH_NAMES: [&str; 12] = [
//     "Baisakh", "Jestha", "Ashar", "Shrawan", "Bhadra", "Asoj", "Kartik", "Mangsir", "Poush",
//     "Magh", "Falgun", "Chaitra",
// ];
// pub static NEPALI_DAY_NAMES: [&str; 7] = [
//     "आइतबार",
//     "सोमबार",
//     "मंगलबार",
//     "बुधबार",
//     "बिहिबार",
//     "शुक्रबार",
//     "शनिबार",
// ];
// pub static NEPALI_DAY_NAMES_SHORT: [&str; 7] = ["आइत", "सोम", "मंगल", "बुध", "बिहि", "शुक्र", "शनि"];
//
// pub static MIN_NEPALI_DATE: (u16, u8, u8) = (1975, 1, 1);
// pub static MAX_NEPALI_DATE: (u16, u8, u8) = (2100, 12, 30);
//
