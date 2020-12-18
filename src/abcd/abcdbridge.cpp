#include <abcd/abcdbridge.h>
#include <blockindex.h>
#include <util/system.h>
#include <cashaddrenc.h>

const std::vector<CTransactionRef> &AbcdBridgeBlockTxs(const CBlock &block) {
    return block.vtx;
}

int32_t AbcdBridgeTxVersion(const CTransactionRef &tx) {
    return tx->nVersion;
}

uint32_t AbcdBridgeTxLockTime(const CTransactionRef &tx) {
    return tx->nLockTime;
}

const std::vector<CTxIn> &AbcdBridgeTxInputs(const CTransactionRef &tx) {
    return tx->vin;
}

const std::vector<CTxOut> &AbcdBridgeTxOutputs(const CTransactionRef &tx) {
    return tx->vout;
}

size_t AbcdBridgeTxSerializeSize(const CTransactionRef &tx) {
    return GetSerializeSize(tx);
}

std::array<std::uint8_t, 32> AbcdBridgeTxId(const CTransactionRef &tx) {
    std::array<std::uint8_t, 32> result;
    TxId txid = tx->GetId();
    std::copy(txid.begin(), txid.end(), result.begin());
    return result;
}

const COutPoint &AbcdBridgeTxInPrevOut(const CTxIn &txin) {
    return txin.prevout;
}

const CScript &AbcdBridgeTxInScriptSig(const CTxIn &txin) {
    return txin.scriptSig;
}

uint32_t AbcdBridgeTxInSequence(const CTxIn &txin) {
    return txin.nSequence;
}

int64_t AbcdBridgeTxOutAmount(const CTxOut &txout) {
    return txout.nValue / Amount::satoshi();
}

const CScript &AbcdBridgeTxOutScriptPubKey(const CTxOut &txout) {
    return txout.scriptPubKey;
}

rust::Slice<const uint8_t> AbcdBridgeTxIdSlice(const TxId &txid) {
    return {txid.begin(), txid.size()};
}

rust::Slice<const uint8_t> AbcdBridgeScriptSlice(const CScript &script) {
    return {script.data(), script.size()};
}

extern "C" CBlockIndex* AbcdBridgeBlockIndexPrev(CBlockIndex *block_index) {
    return block_index->pprev;
}

int32_t AbcdBridgeBlockIndexFile(const CBlockIndex &block_index) {
    return block_index.nFile;
}

uint32_t AbcdBridgeBlockIndexDataPos(const CBlockIndex &block_index) {
    return block_index.nDataPos;
}

rust::String AbcdBridgeDataDir(bool fNetSpecific) {
    return GetDataDir(fNetSpecific).string();
}

DecodedCashAddr AbcdBridgeDecodeCashAddr(rust::Str address, rust::Str expectedPrefix) {
    CashAddrContent content = DecodeCashAddrContent(std::string(address), std::string(expectedPrefix));
    rust::Vec<uint8_t> hash;
    std::copy(content.hash.begin(), content.hash.end(), std::back_inserter(hash));
    return {content.type, hash};
}

std::unique_ptr<FlatFileSeq> AbcdBridgeBlockFileSeq() {
    return std::make_unique<FlatFileSeq>(BlockFileSeq());
}

rust::String AbcdBridgeFlatFileSeqFileName(const FlatFileSeq &flatFileSeq, int32_t fileNum) {
    return flatFileSeq.FileName({fileNum, 0}).string();
}
