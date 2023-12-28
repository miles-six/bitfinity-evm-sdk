
use std::collections::HashMap;
use ethers_core::types::{Block, Transaction};
use super::BlockChainDB;


#[derive(Clone)]
pub struct HashMapBlockchain {
    pub blocks: HashMap<u64, Block<Transaction>>,
}

impl HashMapBlockchain {
    pub fn new() -> Self {
        HashMapBlockchain {
            blocks: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl BlockChainDB for HashMapBlockchain {

    async fn get_blocks_in_range(&self, start: u64, end: u64) -> anyhow::Result<Vec<u64>> {
        let mut blocks_in_range = Vec::new();
        for block_number in start..=end {
            if self.blocks.contains_key(&block_number) {
                blocks_in_range.push(block_number);
            }
        }
        Ok(blocks_in_range)
    }

    async fn get_block_by_number(&self, block: u64) -> anyhow::Result<Block<Transaction>>{
        match self.blocks.get(&block) {
            Some(block) => Ok(block.clone()),
            None => Err(anyhow::anyhow!("Block not found")),
        }
    }
    async fn insert_block(&mut self, block: Block<Transaction>) -> anyhow::Result<()>{
        self.blocks.insert(block.number.unwrap().as_u64(), block);
        Ok(())
    }

}