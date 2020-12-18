mod tx;
mod var_int;

pub use crate::tx::*;
pub use crate::var_int::*;
pub use abcd_bridge::*;

#[allow(unused_variables)]
pub trait AbcEventHandler {
    fn updated_block_tip(
        &self,
        block_index_new: &BlockIndex,
        block_index_fork: Option<&BlockIndex>,
        initial_download: bool,
    ) {
    }
    fn transaction_added_to_mempool(&self, tx: &Tx) {}
    fn transaction_removed_from_mempool(&self, tx: &Tx) {}
    fn block_connected(
        &self,
        block: &Block,
        block_index: &BlockIndex,
        tx_conflicted: &cxx::CxxVector<Tx>,
    ) {
    }
    fn block_disconnected(&self, block: &Block, block_index: &BlockIndex) {}
    fn new_pow_valid_block(&self, block_index: &BlockIndex, block: &Block) {}
}
