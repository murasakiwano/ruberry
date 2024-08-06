use std::io::stdin;

use color_eyre::eyre::bail;
use diesel::RunQueryDsl;
use ruberry_lib::{db::establish_db_connection, models::NewTransaction, schema::transactions};

fn main() -> color_eyre::Result<()> {
    let connection = &mut establish_db_connection().unwrap();

    println!("please insert the filename containing the csv journal");
    let mut filename = String::new();
    stdin().read_line(&mut filename)?;
    let file = std::fs::read_to_string(&filename[..(filename.len() - 1)])?;
    let mut lines = file.lines().map(|l| l.trim());
    match lines.next() {
        Some("date,category,title,amount") => (),
        Some(h) => {
            bail!(
                "need csv to have a header \"{}\", received \"{}\"",
                "date,category,title,amount",
                h
            );
        }
        None => {
            bail!("received an empty csv");
        }
    }

    let mut txs = Vec::new();
    for l in lines {
        let tx = NewTransaction::parse(l)?;
        txs.push(tx);
    }

    let row_count = diesel::insert_into(transactions::table)
        .values(&txs)
        .execute(connection)?;

    println!("Created {} transactions", row_count);

    Ok(())
}
