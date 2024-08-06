use self::commands::{get_expenses, get_transactions, greet};

mod commands;
pub mod date;
pub mod db;
pub mod error;
pub mod models;
pub mod schema;

pub use db::{insert_transaction, read_expenses, read_transactions};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| Ok(db::init()?))
        .invoke_handler(tauri::generate_handler![
            greet,
            get_transactions,
            get_expenses,
            commands::create_transaction,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
