use std::{fmt::Display, ops::Deref};

use chrono::{DateTime, Datelike, Local, MappedLocalTime, NaiveDate};
use color_eyre::{
    eyre::{bail, eyre},
    Result,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Serialize, Deserialize)]
pub struct RuberryDate(DateTime<Local>);

impl Deref for RuberryDate {
    type Target = DateTime<Local>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for RuberryDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let year = self.0.year();
        let month = self.0.month();
        let day = self.0.day();

        write!(f, "{:0>4}-{:0>2}-{:0>2}", year, month, day)
    }
}

impl From<DateTime<Local>> for RuberryDate {
    fn from(value: DateTime<Local>) -> Self {
        RuberryDate(value)
    }
}

impl RuberryDate {
    /// Parse a date from its string format
    ///
    /// The date is expected to be in the "%Y-%m-%d" format, e.g., "1998-22-12". Internally, we use
    /// a [`chrono::DateTime<Local>`], because its API is more convenient to use for our use case,
    /// but the representation we want as a string is akin to [`chrono::NaiveDate`]
    pub fn parse(input: &str) -> Result<Self> {
        let naive_date = match NaiveDate::parse_from_str(input, "%Y-%m-%d") {
            Ok(d) => d,
            Err(e) => {
                bail!("failed to parse date: {}", e);
            }
        };
        let naive_datetime = naive_date
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| eyre!("failed to add hms to date: {}", naive_date))?;
        let MappedLocalTime::Single(local_datetime) = naive_datetime.and_local_timezone(Local)
        else {
            bail!(
                "could not get a single, unambiguous local datetime from the input value: {}",
                input
            )
        };

        Ok(RuberryDate(local_datetime))
    }
}
