use crate::core::BlockIndex;
use crate::core::Transaction;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Copy, Clone, Eq, PartialEq, Debug)]
pub struct Id {
    id: Uuid,
}

impl Id {
    pub fn new() -> Id {
        Id { id: Uuid::new_v4() }
    }
}

#[derive(Deserialize, Serialize, Copy, Clone, Eq, PartialEq, Debug)]
pub struct NewTransactionResult {
    pub block_index: BlockIndex,
}

#[derive(Deserialize, Serialize, Clone, Eq, PartialEq, Debug)]
pub struct BlockResult {
    pub height: i128,
    pub txs: Vec<Transaction>,
    pub size: u8,
}
