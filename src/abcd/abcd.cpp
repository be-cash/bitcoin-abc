#include <abcd/abcd.h>
#include <util/system.h>
#include <string>
#include <primitives/block.h>

std::unique_ptr<AbcdServer> g_abcdserver;

AbcdServer::AbcdServer() : abcdrust(make_abcd_rust()) {}

void AbcdServer::UpdatedBlockTip(const CBlockIndex *pindexNew,
                                  const CBlockIndex *pindexFork,
                                  bool fInitialDownload) {
    if (pindexFork) {
        abcdrust->updated_block_tip_fork(*pindexNew, *pindexFork, fInitialDownload);
    } else {
        abcdrust->updated_block_tip_nofork(*pindexNew, fInitialDownload);
    }
}

void AbcdServer::TransactionAddedToMempool(const CTransactionRef &ptxn) {
    abcdrust->transaction_added_to_mempool(ptxn);
}

void AbcdServer::TransactionRemovedFromMempool(const CTransactionRef &ptx) {
    abcdrust->transaction_removed_from_mempool(ptx);
}

void AbcdServer::BlockConnected(const std::shared_ptr<const CBlock> &block,
                                 const CBlockIndex *pindex,
                                 const std::vector<CTransactionRef> &txnConflicted) {
    abcdrust->block_connected(*block, *pindex, txnConflicted);
}

void AbcdServer::BlockDisconnected(const std::shared_ptr<const CBlock> &block,
                                    const CBlockIndex *pindex) {
    abcdrust->block_disconnected(*block, *pindex);
}

void AbcdServer::NewPoWValidBlock(const CBlockIndex *pindex,
                                   const std::shared_ptr<const CBlock> &block) {
    abcdrust->new_pow_valid_block(*pindex, *block);
}

void AbcdServer::Start() {
    RegisterValidationInterface(this);
    abcdrust->start_grpc();
}

std::unique_ptr<AbcdServer> g_abcideindex;

