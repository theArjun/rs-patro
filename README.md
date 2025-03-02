# rs-patro - Feature Plan

This document outlines the planned features and development roadmap for `rs-patro`, a Rust library designed to provide comprehensive date and time functionalities, potentially including support for regional calendars and time zones.

## Core Features (Initial Release)
* **Validation and Error Handling:**
    * Input validation for date and time values.
    * Error handling for invalid inputs.
* **Gregorian Calendar Support:**
    * Fundamental date and time operations based on the Gregorian calendar.
    * Date and time arithmetic (addition, subtraction).
    * Date and time formatting and parsing (similar to `strftime` and `strptime`).
* **Time Zone Handling:**
    * Basic time zone awareness and conversion.
    * Support for common time zones.
* **Date and Time Object Representation:**
    * Robust `Date`, `Time`, and `DateTime` structs.
    * Clear and consistent API for manipulating these objects.
* **Duration and Period Calculations:**
    * Representation of time durations and periods.
    * Operations for calculating differences between dates and times.
* **Clear Error Handling:**
    * Result based error handling.
    * Informative error messages.
* **Comprehensive Testing:**
    * Unit and integration tests to ensure reliability.
    * Thorough testing of edge cases.

## Planned Enhancements (Future Releases)

* **Regional Calendar Support:**
    * Implementation of regional calendar systems (e.g., Bikram Sambat, Nepali Calendar, etc.).
    * Conversion between Gregorian and regional calendars.
* **Advanced Time Zone Features:**
    * Support for historical time zone data.
    * Daylight Saving Time (DST) calculations.
    * IANA time zone database support.
* **Localization:**
    * Support for localized date and time formatting.
    * Translation of month and day names.
* **Calendar Generation:**
    * Functions for generating calendar views (monthly, yearly).
* **Improved Performance:**
    * Optimization of date and time calculations.
    * Efficient memory usage.
* **Extended Formatting Options:**
    * More robust formatting and parsing options.
* **Enhanced Documentation:**
    * Clear and concise documentation with examples.
    * API reference documentation.
* **Feature Flag Support:**
    * Allow optional features to be enabled or disabled at compile time.
* **Integration with other Rust libraries:**
    * Consider integration with popular libraries such as `serde`, `chrono` and others.

## Usage Examples

1. Convert a Gregorian date to a Nepali date:
```rust
mod constants;
mod conversions;
mod month_end;
mod utils;
use chrono::NaiveDate;
use conversions::BSDate;

fn main() {
    let ad_date = NaiveDate::from_ymd_opt(2025,3,2).unwrap();
    let ad_date = BSDate::from_ad(ad_date).unwrap();
    println!("AD Date: {}", ad_date);
}
```

2. Convert a Nepali date to a Gregorian date:
```rust
mod constants;
mod conversions;
mod month_end;
mod utils;
use conversions::BSDate;

fn main() {
    let bs_date = BSDate::new(2081,11,18);
    let ad_date = bs_date.to_ad().unwrap();
    println!("AD Date: {}", ad_date);
}
```

## Ultimate Plan
- Create a python wrapper for the library and use it as drop in replacement for `nepali_datetime` library.

## Contribution

Contributions are welcome! Please refer to the `CONTRIBUTING.md` file for guidelines on how to contribute.
