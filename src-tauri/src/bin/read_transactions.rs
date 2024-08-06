use color_eyre::eyre::Result;
use ruberry_lib::{db::establish_db_connection, read_transactions};

fn main() -> Result<()> {
    let conn = &mut establish_db_connection()?;

    let txs = read_transactions(conn)?;

    for tx in txs {
        println!("{}", tx);
    }

    Ok(())
}
