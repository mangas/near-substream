mod pb;

use pb::{
    receipts::v1::{Receipt, Receipts},
    sf::near::r#type::v1::Block,
};
use substreams_entity_change::pb::entity::EntityChanges;

#[substreams::handlers::map]
fn map_block(blk: Block) -> Result<Receipts, substreams::errors::Error> {
    let mut out = Receipts::default();

    for shard in blk.shards {
        let chunk = match shard.chunk {
            Some(chunk) => chunk,
            None => continue,
        };

        for receipt in chunk.receipts {
            out.chunk_receipts.push(Receipt {
                id: hex::encode(receipt.receipt_id.unwrap().bytes.as_slice()),
            })
        }

        for exec_outcome in shard.receipt_execution_outcomes {
            out.outcome_receipts.push(Receipt {
                id: hex::encode(
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

#[substreams::handlers::map]
fn graph_out(blk: Block) -> Result<EntityChanges, substreams::errors::Error> {
    let mut out = EntityChanges::default();

    let hex = hex::encode(&blk.header.as_ref().unwrap().hash.as_ref().unwrap().bytes);

    out.push_change(
        "Block",
        &hex,
        blk.header.unwrap().height,
        substreams_entity_change::pb::entity::entity_change::Operation::Create,
    );

    Ok(out)
}
