#include <abcdgen.h>
#include <abcdgen_bridge.h>

#include <rust/cxx.h>
#include <primitives/block.h>
#include <primitives/transaction.h>
#include <validationinterface.h>

#ifndef BITCOIN_INDEX_ABCD_H
#define BITCOIN_INDEX_ABCD_H


/**
 *
 */
class AbcdServer final : public CValidationInterface {
public:
    AbcdServer();
    void Start();
protected:
    virtual void
    UpdatedBlockTip(const CBlockIndex *pindexNew,
                    const CBlockIndex *pindexFork,
                    bool fInitialDownload);

    virtual void
    TransactionAddedToMempool(const CTransactionRef &ptxn);

    virtual void
    TransactionRemovedFromMempool(const CTransactionRef &ptx);

    virtual void
    BlockConnected(const std::shared_ptr<const CBlock> &block,
                   const CBlockIndex *pindex,
                   const std::vector<CTransactionRef> &txnConflicted);

    virtual void
    BlockDisconnected(const std::shared_ptr<const CBlock> &block,
                      const CBlockIndex *pindex);

    virtual void
    NewPoWValidBlock(const CBlockIndex *pindex,
                     const std::shared_ptr<const CBlock> &block);
private:
    rust::Box<AbcdRust> abcdrust;
};

extern std::unique_ptr<AbcdServer> g_abcdserver;

#endif // BITCOIN_INDEX_ABCD_H

