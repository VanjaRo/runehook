use chainhook_sdk::utils::Context;
use model::DbRune;
use ordinals::RuneId;
use postgres::{Client, Error, NoTls, Transaction};
use refinery::embed_migrations;

pub mod index_cache;
pub mod model;

embed_migrations!("migrations");

pub fn init_db(ctx: &Context) -> Result<Client, Error> {
    let mut client = Client::connect("host=localhost user=postgres", NoTls)?;
    migrations::runner().run(&mut client).unwrap();
    // Insert default UNCOMMON•GOODS rune
    // let rune = SpacedRune::from_str("UNCOMMON•GOODS").unwrap();
    // let _ = insert_etching(
    //     &Etching {
    //         divisibility: Some(0),
    //         premine: Some(0),
    //         rune: Some(rune.rune),
    //         spacers: Some(rune.spacers),
    //         symbol: Some('⧉'),
    //         terms: Some(Terms {
    //             amount: Some(1),
    //             cap: Some(u128::max_value()),
    //             height: (Some(840000), Some(1050000)),
    //             offset: (None, None),
    //         }),
    //         turbo: false,
    //     },
    //     1,
    //     0,
    //     &"".to_string(),
    //     &mut client,
    //     ctx,
    // );
    Ok(client)
}

pub fn insert_rune_rows(
    rows: &Vec<DbRune>,
    db_tx: &mut Transaction,
    _ctx: &Context,
) -> Result<bool, Error> {
    let stmt = db_tx.prepare(
        "INSERT INTO runes
        (name, block_height, tx_index, tx_id, divisibility, premine, symbol, terms_amount, terms_cap, terms_height_start,
         terms_height_end, terms_offset_start, terms_offset_end, turbo)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        ON CONFLICT (name) DO NOTHING"
    )?;
    for row in rows.iter() {
        let _ = db_tx.execute(
            &stmt,
            &[
                &row.name,
                &row.block_height,
                &row.tx_index,
                &row.tx_id,
                &row.divisibility,
                &row.premine,
                &row.symbol,
                &row.terms_amount,
                &row.terms_cap,
                &row.terms_height_start,
                &row.terms_height_end,
                &row.terms_offset_start,
                &row.terms_offset_end,
                &row.turbo,
            ],
        );
    }
    Ok(true)
}

// pub fn insert_edict(
//     edict: &Edict,
//     block_height: u64,
//     tx_index: u32,
//     tx_id: &String,
//     client: &mut Client,
//     ctx: &Context,
// ) -> Result<bool, Error> {
//     //
// }

pub fn get_rune_by_rune_id(rune_id: RuneId, db_tx: &mut Transaction, ctx: &Context) -> Option<DbRune> {
    let rows = match db_tx.query(
        "SELECT * FROM runes WHERE block_height = $1 AND tx_index = $2",
        &[&rune_id.block.to_string(), &rune_id.tx.to_string()],
    ) {
        Ok(rows) => rows,
        Err(e) => {
            error!(
                ctx.expect_logger(),
                "error retrieving rune: {}",
                e.to_string()
            );
            panic!();
        }
    };
    let Some(row) = rows.get(0) else {
        return None;
    };
    Some(DbRune::from_pg_row(row))
}
