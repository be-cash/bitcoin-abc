pub mod opcode;
mod script;

pub use crate::script::*;

#[cxx::bridge]
mod ffi {
    struct DecodedCashAddr {
        pub kind: CashAddrType,
        pub hash: Vec<u8>,
    }

    enum CashAddrType {
        PUBKEY_TYPE = 0,
        SCRIPT_TYPE = 1,
    }

    unsafe extern "C++" {
        include!("abcd/abcdbridge.h");
        include!("cashaddrenc.h");
        include!("flatfile.h");
        type TxId;
        fn AbcdBridgeTxIdSlice(txid: &TxId) -> &[u8];

        type CBlock;
        fn AbcdBridgeBlockTxs(block: &CBlock) -> &CxxVector<CTransactionRef>;

        type CTransactionRef;
        fn AbcdBridgeTxVersion(tx: &CTransactionRef) -> i32;
        fn AbcdBridgeTxLockTime(tx: &CTransactionRef) -> u32;
        fn AbcdBridgeTxInputs(tx: &CTransactionRef) -> &CxxVector<CTxIn>;
        fn AbcdBridgeTxOutputs(tx: &CTransactionRef) -> &CxxVector<CTxOut>;
        fn AbcdBridgeTxSerializeSize(tx: &CTransactionRef) -> usize;
        fn AbcdBridgeTxId(tx: &CTransactionRef) -> [u8; 32];

        type CTxIn;
        fn AbcdBridgeTxInPrevOut(txin: &CTxIn) -> &COutPoint;
        fn AbcdBridgeTxInScriptSig(txin: &CTxIn) -> &CScript;
        fn AbcdBridgeTxInSequence(txin: &CTxIn) -> u32;

        type CTxOut;
        fn AbcdBridgeTxOutAmount(txin: &CTxOut) -> i64;
        fn AbcdBridgeTxOutScriptPubKey(txin: &CTxOut) -> &CScript;

        type COutPoint;
        fn GetN(self: &COutPoint) -> u32;
        fn GetTxId(self: &COutPoint) -> &TxId;
        fn IsNull(self: &COutPoint) -> bool;

        type CScript;
        fn AbcdBridgeScriptSlice(txin: &CScript) -> &[u8];

        type CBlockIndex;
        fn AbcdBridgeBlockIndexFile(block_index: &CBlockIndex) -> i32;
        fn AbcdBridgeBlockIndexDataPos(block_index: &CBlockIndex) -> u32;

        fn AbcdBridgeDataDir(net_specific: bool) -> String;

        type CashAddrType;
        fn AbcdBridgeDecodeCashAddr(cash_addr: &str, expected_prefix: &str) -> DecodedCashAddr;

        type FlatFileSeq;
        fn AbcdBridgeBlockFileSeq() -> UniquePtr<FlatFileSeq>;
        fn AbcdBridgeFlatFileSeqFileName(flat_file_seq: &FlatFileSeq, file_num: i32) -> String;
    }
}

extern "C" {
    fn AbcdBridgeBlockIndexPrev(block_index: *const ffi::CBlockIndex) -> *const ffi::CBlockIndex;
}

pub type Block = ffi::CBlock;
pub type Tx = ffi::CTransactionRef;
pub type TxId = ffi::TxId;
pub type TxInput = ffi::CTxIn;
pub type TxOutput = ffi::CTxOut;
pub type TxOutpoint = ffi::COutPoint;
pub type Script = ffi::CScript;
pub type BlockIndex = ffi::CBlockIndex;
pub type DecodedCashAddr = ffi::DecodedCashAddr;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BlockIndexFilePos {
    pub file_num: i32,
    pub data_pos: u32,
}

pub struct BlockFileSeq {
    block_file_seq: cxx::UniquePtr<ffi::FlatFileSeq>,
}

impl Block {
    pub fn txs(&self) -> &cxx::CxxVector<Tx> {
        ffi::AbcdBridgeBlockTxs(self)
    }
}

impl Tx {
    pub fn id(&self) -> [u8; 32] {
        ffi::AbcdBridgeTxId(self)
    }

    pub fn version(&self) -> i32 {
        ffi::AbcdBridgeTxVersion(self)
    }

    pub fn lock_time(&self) -> u32 {
        ffi::AbcdBridgeTxLockTime(self)
    }

    pub fn inputs(&self) -> &cxx::CxxVector<TxInput> {
        ffi::AbcdBridgeTxInputs(self)
    }

    pub fn outputs(&self) -> &cxx::CxxVector<TxOutput> {
        ffi::AbcdBridgeTxOutputs(self)
    }

    pub fn serialize_size(&self) -> usize {
        ffi::AbcdBridgeTxSerializeSize(self)
    }
}

impl TxInput {
    pub fn prev_out(&self) -> &TxOutpoint {
        ffi::AbcdBridgeTxInPrevOut(self)
    }

    pub fn script_sig(&self) -> &Script {
        ffi::AbcdBridgeTxInScriptSig(self)
    }

    pub fn sequence(&self) -> u32 {
        ffi::AbcdBridgeTxInSequence(self)
    }
}

impl TxOutput {
    pub fn amount(&self) -> i64 {
        ffi::AbcdBridgeTxOutAmount(self)
    }

    pub fn script_pub_key(&self) -> &Script {
        ffi::AbcdBridgeTxOutScriptPubKey(self)
    }
}

impl TxOutpoint {
    pub fn index(&self) -> u32 {
        self.GetN()
    }
    pub fn txid(&self) -> &TxId {
        self.GetTxId()
    }
    pub fn is_null(&self) -> bool {
        self.IsNull()
    }
}

impl Script {
    pub fn bytecode(&self) -> &[u8] {
        ffi::AbcdBridgeScriptSlice(self)
    }
}

impl BlockIndex {
    pub fn prev(&self) -> Option<&BlockIndex> {
        unsafe {
            let prev = AbcdBridgeBlockIndexPrev(self as *const ffi::CBlockIndex);
            if prev.is_null() {
                return None;
            } else {
                return Some(&*prev);
            }
        };
    }

    pub fn file_pos(&self) -> BlockIndexFilePos {
        BlockIndexFilePos {
            file_num: ffi::AbcdBridgeBlockIndexFile(self),
            data_pos: ffi::AbcdBridgeBlockIndexDataPos(self),
        }
    }
}

pub fn get_data_dir(net_specific: bool) -> String {
    ffi::AbcdBridgeDataDir(net_specific)
}

pub fn decode_cash_addr(cash_addr: &str, expected_prefix: &str) -> Option<DecodedCashAddr> {
    let decoded = ffi::AbcdBridgeDecodeCashAddr(cash_addr, expected_prefix);
    if decoded.hash.is_empty() {
        return None;
    }
    Some(decoded)
}

impl BlockFileSeq {
    pub fn new() -> Self {
        BlockFileSeq {
            block_file_seq: ffi::AbcdBridgeBlockFileSeq(),
        }
    }

    pub fn file_name(&self, file_num: i32) -> String {
        ffi::AbcdBridgeFlatFileSeqFileName(&self.block_file_seq, file_num)
    }
}
