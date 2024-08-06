use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::str::FromStr as _;

use color_eyre::eyre;
use diesel::prelude::*;
use rust_decimal::Decimal;

use crate::{date::RuberryDate, schema::transactions};

#[derive(Serialize, Deserialize, Queryable, Selectable, Debug)]
#[diesel(table_name = transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Transaction {
    pub id: i32,
    pub category: String,
    pub title: String,
    pub amount: f32,
    pub date: String,
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -- {} -- {} -- R$ {:.2}",
            self.date, self.title, self.category, self.amount
        )
    }
}

#[derive(Deserialize, Insertable, Clone)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub category: String,
    pub title: String,
    pub amount: f32,
    pub date: String,
}

impl NewTransaction {
    /// Parse a Transaction from its String format
    ///
    /// A Transaction is expected to have the format "<date>,<category>,<title>,<amount>". None of
    /// the fields can include a ',', since it will impossibilitate the parsing. The date is
    /// expected to be in the "%Y-%m-%d" format, e.g., "1998-22-12".
    pub fn parse(input: &str) -> color_eyre::Result<Self> {
        // split in ','
        let s_input = input.split(',').collect::<Vec<_>>();
        // check if it was split in four
        if s_input.len() != 4 {
            return Err(eyre::eyre!(
                "Transaction does not have the expected length (4): {}",
                input
            ));
        }

        // iterate through the four parts, parsing each one -- checks order!
        let [date, category, title, amount] = s_input[..] else {
            return Err(eyre::eyre!(
                "this is weird, but the input could not be split in four parts! {:?}",
                s_input
            ));
        };

        let date = RuberryDate::parse(date)?;

        let amount = Decimal::from_str(amount)?;

        Ok(Self {
            category: category.to_string(),
            title: title.to_string(),
            amount: amount.to_f32().unwrap(),
            date: date.to_string(),
        })
    }
}
