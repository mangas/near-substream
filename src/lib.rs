mod pb;

use pb::{
    receipts::v1::{Receipt, Receipts},
    sf::near::r#type::v1::Block,
};

#[substreams::handlers::map]
fn map_block(blk: Block) -> Result<Receipts, substreams::errors::Error> {
    let mut out = Receipts::default();

    for shard in blk.shards {
        let chunk = shard.chunk.unwrap();

        for receipt in chunk.receipts {
            out.chunk_receipts.push(Receipt {
                id: to_base58(receipt.receipt_id.unwrap().bytes.as_slice()),
            })
        }

        for exec_outcome in shard.receipt_execution_outcomes {
            out.outcome_receipts.push(Receipt {
                id: to_base58(
                    exec_outcome
                        .receipt
                        .unwrap()
                        .receipt_id
                        .unwrap()
                        .bytes
                        .as_slice(),
                ),
            });
        }
    }

    Ok(out)
}

fn to_base58(bytes: &[u8]) -> String {
    bs58::encode(bytes).into_string()
}
