use abcd_address_indexer::AbcdAddressIndexer;
use abcd_base::{get_data_dir, AbcEventHandler, Block, BlockIndex, Tx};

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type AbcdRust;
        fn make_abcd_rust() -> Box<AbcdRust>;
        fn start_grpc(&self);
        fn block_connected(
            &self,
            block: &CBlock,
            block_index: &CBlockIndex,
            tx_conflicted: &CxxVector<CTransactionRef>,
        );
        fn updated_block_tip_nofork(&self, block_index_new: &CBlockIndex, initial_download: bool);
        fn updated_block_tip_fork(
            &self,
            block_index_new: &CBlockIndex,
            block_index_fork: &CBlockIndex,
            initial_download: bool,
        );
        fn transaction_added_to_mempool(&self, tx: &CTransactionRef);
        fn transaction_removed_from_mempool(&self, tx: &CTransactionRef);
        fn block_disconnected(&self, block: &CBlock, block_index: &CBlockIndex);
        fn new_pow_valid_block(&self, block_index: &CBlockIndex, block: &CBlock);
    }

    extern "C++" {
        include!("abcd/abcdbridge.h");
        type CBlock = abcd_base::Block;
        type CTransactionRef = abcd_base::Tx;
        type CBlockIndex = abcd_base::BlockIndex;
    }
}

pub struct AbcdRust {
    address_indexer: AbcdAddressIndexer,
}

const ABCD_FOLDER: &'static str = "abcd";

fn make_abcd_rust() -> Box<AbcdRust> {
    let datadir = get_data_dir(true);
    let datadir = std::path::Path::new(&datadir);
    let abcd_folder = datadir.join(ABCD_FOLDER);
    Box::new(AbcdRust {
        address_indexer: AbcdAddressIndexer::new(&abcd_folder).unwrap(),
    })
}

impl AbcdRust {
    fn start_grpc(&self) {
        let address_server = self.address_indexer.make_server();
        std::thread::spawn(move || {
            let mut rt = tokio::runtime::Runtime::new().unwrap();
            let addr = "0.0.0.0:50051".parse().unwrap();
            println!("start grpc on {}", addr);
            rt.block_on(async {
                tonic::transport::Server::builder()
                    .add_service(address_server.service())
                    .serve(addr)
                    .await
                    .unwrap()
            });
        });
    }

    fn event_handlers(&self) -> [&dyn AbcEventHandler; 1] {
        [&self.address_indexer]
    }

    /// Notifies listeners of a block being connected.
    /// Provides a vector of transactions evicted from the mempool as a result.
    ///
    /// Called on a background thread.
    fn block_connected(
        &self,
        block: &Block,
        block_index: &BlockIndex,
        tx_conflicted: &cxx::CxxVector<Tx>,
    ) {
        for handler in self.event_handlers().iter() {
            handler.block_connected(block, block_index, tx_conflicted);
        }
    }

    /// Notifies listeners when the block chain tip advances.
    ///
    /// When multiple blocks are connected at once, UpdatedBlockTip will be
    /// called on the final tip but may not be called on every intermediate tip.
    /// If the latter behavior is desired, subscribe to BlockConnected() instead.
    ///
    /// Called on a background thread.
    fn updated_block_tip_fork(
        &self,
        block_index_new: &BlockIndex,
        block_index_fork: &BlockIndex,
        initial_download: bool,
    ) {
        for handler in self.event_handlers().iter() {
            handler.updated_block_tip(block_index_new, Some(block_index_fork), initial_download);
        }
    }

    /// Notifies listeners when the block chain tip advances.
    ///
    /// When multiple blocks are connected at once, UpdatedBlockTip will be
    /// called on the final tip but may not be called on every intermediate tip.
    /// If the latter behavior is desired, subscribe to BlockConnected() instead.
    ///
    /// Called on a background thread.
    fn updated_block_tip_nofork(&self, block_index_new: &BlockIndex, initial_download: bool) {
        for handler in self.event_handlers().iter() {
            handler.updated_block_tip(block_index_new, None, initial_download);
        }
    }

    /// Notifies listeners of a transaction having been added to mempool.
    ///
    /// Called on a background thread.
    fn transaction_added_to_mempool(&self, tx: &Tx) {
        for handler in self.event_handlers().iter() {
            handler.transaction_added_to_mempool(tx);
        }
    }

    /// Notifies listeners of a transaction leaving mempool.
    ///
    /// This only fires for transactions which leave mempool because of expiry,
    /// size limiting, reorg (changes in lock times/coinbase maturity), or
    /// replacement. This does not include any transactions which are included
    /// in BlockConnectedDisconnected either in block->vtx or in txnConflicted.
    ///
    /// Called on a background thread.
    fn transaction_removed_from_mempool(&self, tx: &Tx) {
        for handler in self.event_handlers().iter() {
            handler.transaction_removed_from_mempool(tx);
        }
    }

    /// Notifies listeners of a block being disconnected
    ///
    /// Called on a background thread.
    fn block_disconnected(&self, block: &Block, block_index: &BlockIndex) {
        for handler in self.event_handlers().iter() {
            handler.block_disconnected(block, block_index);
        }
    }

    /// Notifies listeners that a block which builds directly on our current tip
    /// has been received and connected to the headers tree, though not validated
    /// yet.
    fn new_pow_valid_block(&self, block_index: &BlockIndex, block: &Block) {
        for handler in self.event_handlers().iter() {
            handler.new_pow_valid_block(block_index, block);
        }
    }
}
