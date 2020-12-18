use abcd_base::{
    decode_cash_addr, read_var_int, var_int_size, write_var_int, AbcEventHandler, Block, BlockDb,
    BlockIndex, BlockIndexFilePos, ScriptKind, Tx,
};
use abcdaddress::{
    address_service_server::{AddressService, AddressServiceServer},
    GetAddressRawTxsReply, GetAddressRawTxsRequest, GetAddressTxidsReply, GetAddressTxidsRequest,
    RawTx,
};
use anyhow::Result;
use itertools::Itertools;
use plain::Plain;
use std::{path::Path, sync::Arc};
use tonic::{Request, Response, Status};

pub mod abcdaddress {
    tonic::include_proto!("abcdaddress");
}

const ADDRINDEX_FOLDER: &'static str = "addrindex";

pub struct AbcdAddressIndexer {
    db: Arc<rocksdb::DB>,
}

pub struct AbcdAddressServer {
    db: Arc<rocksdb::DB>,
}

const P2PKH_KIND: u8 = 0;
const P2SH_KIND: u8 = 1;

#[repr(C, align(1))]
#[derive(Default)]
struct Key {
    kind: u8,
    addr_hash: [u8; 20],
    data: [u8; 24],
    tx_pos_idx: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AddressEntry {
    file_pos: BlockIndexFilePos,
    tx_pos: usize,
    out_idx: u32,
}

impl AbcdAddressIndexer {
    pub fn new(abcd_datadir: impl AsRef<Path>) -> Result<Self> {
        let path = abcd_datadir.as_ref().join(ADDRINDEX_FOLDER);
        std::fs::create_dir_all(&path)?;
        Ok(AbcdAddressIndexer {
            db: Arc::new(rocksdb::DB::open_default(&path)?),
        })
    }

    pub fn make_server(&self) -> AbcdAddressServer {
        AbcdAddressServer {
            db: Arc::clone(&self.db),
        }
    }
}

impl AbcdAddressServer {
    pub fn get_address_entries(
        &self,
        kind: u8,
        addr_hash: &[u8],
    ) -> impl Iterator<Item = AddressEntry> + '_ {
        let search_prefix = [[kind].as_ref(), addr_hash].concat();
        self.db
            .prefix_iterator(search_prefix)
            .map(|(key, _)| AddressEntry::from_key_slice(&key))
            .dedup()
    }

    pub fn service(self) -> AddressServiceServer<Self> {
        AddressServiceServer::new(self)
    }
}

impl AbcEventHandler for AbcdAddressIndexer {
    fn block_connected(
        &self,
        block: &Block,
        block_index: &BlockIndex,
        _tx_conflicted: &cxx::CxxVector<Tx>,
    ) {
        let mut key = Key::new(block_index.file_pos());
        let mut tx_offset = var_int_size(block.txs().len() as u64); // starts after block header
        for tx in block.txs().iter() {
            for (out_idx, txout) in tx.outputs().iter().enumerate() {
                let should_index = key.set_script_kind(&txout.script_pub_key().kind());
                if should_index {
                    let key = key.make_slice(tx_offset, out_idx);
                    let value = b"";
                    self.db.put(key, value).unwrap();
                }
            }
            tx_offset += tx.serialize_size();
        }
    }
}

#[tonic::async_trait]
impl AddressService for AbcdAddressServer {
    async fn get_address_txids(
        &self,
        request: Request<GetAddressTxidsRequest>,
    ) -> Result<Response<GetAddressTxidsReply>, Status> {
        let request = request.into_inner();
        let decoded = decode_cash_addr(&request.address, "bitcoincash")
            .ok_or_else(|| Status::invalid_argument("Invalid address"))?;

        let block_db = BlockDb::new();
        let txids = self
            .get_address_entries(decoded.kind.repr, &decoded.hash)
            .map(|entry| {
                use sha2::{Digest, Sha256};
                let (_, raw_tx) = block_db.read_tx(entry.file_pos, entry.tx_pos)?;
                let txid = Sha256::digest(Sha256::digest(&raw_tx).as_slice()).to_vec();
                Ok(txid)
            })
            .collect::<Result<Vec<_>, std::io::Error>>()
            .map_err(|err| Status::data_loss(format!("Couldn't read transaction: {}", err)))?;

        let reply = GetAddressTxidsReply { txids };

        Ok(Response::new(reply))
    }

    async fn get_address_raw_txs(
        &self,
        request: Request<GetAddressRawTxsRequest>,
    ) -> Result<Response<GetAddressRawTxsReply>, Status> {
        let request = request.into_inner();
        let decoded = decode_cash_addr(&request.address, "bitcoincash")
            .ok_or_else(|| Status::invalid_argument("Invalid address"))?;

        let block_db = BlockDb::new();
        let txs = self
            .get_address_entries(decoded.kind.repr, &decoded.hash)
            .map(|entry| {
                use sha2::{Digest, Sha256};
                let (blockheader, raw_tx) = block_db.read_tx(entry.file_pos, entry.tx_pos)?;
                let blockhash = Sha256::digest(Sha256::digest(&blockheader).as_slice()).to_vec();
                Ok(RawTx { raw_tx, blockhash })
            })
            .collect::<Result<Vec<_>, std::io::Error>>()
            .map_err(|err| Status::data_loss(format!("Couldn't read transaction: {}", err)))?;

        let reply = GetAddressRawTxsReply { txs };

        Ok(Response::new(reply))
    }
}

impl AddressEntry {
    fn from_key_slice(key: &[u8]) -> Self {
        let mut offset = 21;
        let (file_num, file_num_size) = read_var_int(&key[offset..]).unwrap();
        offset += file_num_size;
        let (data_pos, data_pos_size) = read_var_int(&key[offset..]).unwrap();
        offset += data_pos_size;
        let (tx_pos, tx_pos_size) = read_var_int(&key[offset..]).unwrap();
        offset += tx_pos_size;
        let (out_idx, _) = read_var_int(&key[offset..]).unwrap();
        AddressEntry {
            file_pos: BlockIndexFilePos {
                file_num: file_num as i32,
                data_pos: data_pos as u32,
            },
            tx_pos: tx_pos as usize,
            out_idx: out_idx as u32,
        }
    }
}

unsafe impl Plain for Key {} // this is safe because Key is tightly packed POD

impl Key {
    const SIZE_WITHOUT_OUT_IDX: usize = 21;

    fn new(block_file_pos: BlockIndexFilePos) -> Self {
        let mut key = Key::default();
        let mut tx_pos_idx = 0;
        tx_pos_idx +=
            write_var_int(&mut key.data[tx_pos_idx..], block_file_pos.file_num as u64).unwrap();
        tx_pos_idx +=
            write_var_int(&mut key.data[tx_pos_idx..], block_file_pos.data_pos as u64).unwrap();
        key.tx_pos_idx = tx_pos_idx;
        key
    }

    fn _as_slice(&self) -> &[u8] {
        unsafe { plain::as_bytes(self) } // this is safe because Key has no padding
    }

    fn set_script_kind(&mut self, script_kind: &ScriptKind<'_>) -> bool {
        let (kind, hash) = match *script_kind {
            ScriptKind::P2PKH(hash) => (P2PKH_KIND, hash),
            ScriptKind::P2SH(hash) => (P2SH_KIND, hash),
            _ => return false,
        };
        self.kind = kind;
        self.addr_hash.copy_from_slice(hash);
        true
    }

    fn make_slice(&mut self, tx_pos: usize, out_idx: usize) -> &[u8] {
        let mut data_offset = self.tx_pos_idx;
        data_offset += write_var_int(&mut self.data[data_offset..], tx_pos as u64).unwrap();
        data_offset += write_var_int(&mut self.data[data_offset..], out_idx as u64).unwrap();
        &self._as_slice()[..Self::SIZE_WITHOUT_OUT_IDX + data_offset]
    }
}
