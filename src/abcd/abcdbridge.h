#include <rust/cxx.h>
#include <primitives/block.h>
#include <primitives/transaction.h>
#include <validationinterface.h>
#include <abcdgen_bridge.h>
#include <flatfile.h>
#include <blockdb.h>

struct DecodedCashAddr;

rust::Slice<const uint8_t> AbcdBridgeTxIdSlice(const TxId &txid);

const std::vector<CTransactionRef> &AbcdBridgeBlockTxs(const CBlock &block);
int32_t AbcdBridgeTxVersion(const CTransactionRef &tx);
uint32_t AbcdBridgeTxLockTime(const CTransactionRef &tx);
const std::vector<CTxIn> &AbcdBridgeTxInputs(const CTransactionRef &tx);
const std::vector<CTxOut> &AbcdBridgeTxOutputs(const CTransactionRef &tx);
size_t AbcdBridgeTxSerializeSize(const CTransactionRef &tx);
std::array<std::uint8_t, 32> AbcdBridgeTxId(const CTransactionRef &tx);

const COutPoint &AbcdBridgeTxInPrevOut(const CTxIn &txin);
const CScript &AbcdBridgeTxInScriptSig(const CTxIn &txin);
uint32_t AbcdBridgeTxInSequence(const CTxIn &txin);

int64_t AbcdBridgeTxOutAmount(const CTxOut &txout);
const CScript &AbcdBridgeTxOutScriptPubKey(const CTxOut &txout);

rust::Slice<const uint8_t> AbcdBridgeTxIdSlice(const TxId &txid);

rust::Slice<const uint8_t> AbcdBridgeScriptSlice(const CScript &script);

extern "C" CBlockIndex* AbcdBridgeBlockIndexPrev(CBlockIndex *block_index);

int32_t AbcdBridgeBlockIndexFile(const CBlockIndex &block_index);
uint32_t AbcdBridgeBlockIndexDataPos(const CBlockIndex &block_index);

rust::String AbcdBridgeDataDir(bool fNetSpecific);
DecodedCashAddr AbcdBridgeDecodeCashAddr(rust::Str address, rust::Str expectedPrefix);

std::unique_ptr<FlatFileSeq> AbcdBridgeBlockFileSeq();
rust::String AbcdBridgeFlatFileSeqFileName(const FlatFileSeq &flatFileSeq, int32_t fileNum);
