use std::fs;
use std::path::Path;

use diesel::{
    sqlite::SqliteConnection, Connection as _, ExpressionMethods, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness as _};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

use crate::{
    error::Error,
    models::{NewTransaction, Transaction},
};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn init() -> Result<(), Error> {
    let data_dir = dirs::data_dir().unwrap_or(dirs::home_dir().unwrap());
    let db_path = data_dir.to_str().unwrap().to_string() + "/ruberry/database.sqlite";
    let db_dir = Path::new(&db_path).parent().unwrap();

    if !db_dir.exists() {
        fs::create_dir_all(db_dir)?;
    }

    if !Path::new(&db_path).exists() {
        fs::File::create(&db_path)?;
    }

    let database_url = format!("sqlite://{db_path}");
    let mut connection = SqliteConnection::establish(&database_url)?;
    connection.run_pending_migrations(MIGRATIONS).unwrap();

    Ok(())
}

pub fn establish_db_connection() -> Result<SqliteConnection, Error> {
    let data_dir = dirs::data_dir().unwrap_or(dirs::home_dir().unwrap());
    let db_path = data_dir.to_str().unwrap().to_string() + "/ruberry/database.sqlite";

    let database_url = format!("sqlite://{db_path}");
    Ok(SqliteConnection::establish(&database_url)?)
}

pub fn insert_transaction(
    conn: &mut SqliteConnection,
    category: &str,
    title: &str,
    amount: Decimal,
    date: &str,
) -> Result<Transaction, Error> {
    use crate::schema::transactions;

    let amount = amount.to_f32().unwrap();

    let new_transaction = NewTransaction {
        category: category.to_string(),
        title: title.to_string(),
        amount,
        date: date.to_string(),
    };

    Ok(diesel::insert_into(transactions::table)
        .values(&new_transaction)
        .returning(Transaction::as_returning())
        .get_result(conn)?)
}

pub fn read_transactions(conn: &mut SqliteConnection) -> Result<Vec<Transaction>, Error> {
    use crate::schema::transactions::dsl::*;

    Ok(transactions.select(Transaction::as_select()).load(conn)?)
}

/// An expense is categorized as a transaction whose value is *positive*
pub fn read_expenses(conn: &mut SqliteConnection) -> Result<Vec<Transaction>, Error> {
    use crate::schema::transactions::dsl::*;

    Ok(transactions
        .filter(amount.ge(0.0))
        .select(Transaction::as_select())
        .load(conn)?)
}
