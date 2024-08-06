use rust_decimal::Decimal;

use crate::{
    db::{establish_db_connection, insert_transaction, read_expenses, read_transactions},
    error::Error,
    models::Transaction,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn get_transactions() -> Result<Vec<Transaction>, Error> {
    let conn = &mut establish_db_connection()?;

    read_transactions(conn)
}

#[tauri::command]
pub fn get_expenses() -> Result<Vec<Transaction>, Error> {
    let conn = &mut establish_db_connection()?;

    read_expenses(conn)
}

#[tauri::command]
pub fn create_transaction(
    category: &str,
    title: &str,
    amount: Decimal,
    date: &str,
) -> Result<Transaction, Error> {
    let conn = &mut establish_db_connection()?;

    insert_transaction(conn, category, title, amount, date)
}
