use crate::{read_var_int_stream, write_var_int_stream, BlockFileSeq, BlockIndexFilePos};
use std::io::{Read, Result};

pub struct BlockDb {
    block_file_seq: BlockFileSeq,
}

impl BlockDb {
    pub fn new() -> Self {
        BlockDb {
            block_file_seq: BlockFileSeq::new(),
        }
    }

    pub fn read_tx(
        &self,
        file_pos: BlockIndexFilePos,
        tx_pos: usize,
    ) -> Result<([u8; 80], Vec<u8>)> {
        use std::io::{Seek, SeekFrom};
        let mut file = std::fs::File::open(self.block_file_seq.file_name(file_pos.file_num))?;
        file.seek(SeekFrom::Start(file_pos.data_pos as u64))?;
        let mut header = [0; 80];
        file.read_exact(&mut header)?;
        file.seek(SeekFrom::Current(tx_pos as i64))?;
        let raw_tx = read_raw_tx(&mut file)?;
        Ok((header, raw_tx))
    }
}

pub fn read_raw_tx(stream: &mut impl Read) -> Result<Vec<u8>> {
    let mut raw_tx = Vec::with_capacity(64);
    let mut version = [0; 4];
    stream.read_exact(&mut version)?;
    raw_tx.extend_from_slice(&version);
    let (num_inputs, _) = read_var_int_stream(stream)?;
    write_var_int_stream(&mut raw_tx, num_inputs)?;
    for _ in 0..num_inputs {
        let mut outpoint = [0; 36];
        stream.read_exact(&mut outpoint)?;
        raw_tx.extend_from_slice(&outpoint);
        let (input_script_size, _) = read_var_int_stream(stream)?;
        write_var_int_stream(&mut raw_tx, input_script_size)?;
        let mut script = vec![0; input_script_size as usize];
        stream.read_exact(&mut script)?;
        raw_tx.extend_from_slice(&script);
        let mut sequence = [0; 4];
        stream.read_exact(&mut sequence)?;
        raw_tx.extend_from_slice(&sequence);
    }
    let (num_outputs, _) = read_var_int_stream(stream)?;
    write_var_int_stream(&mut raw_tx, num_outputs)?;
    for _ in 0..num_outputs {
        let mut value = [0; 8];
        stream.read_exact(&mut value)?;
        raw_tx.extend_from_slice(&value);
        let (output_script_size, _) = read_var_int_stream(stream)?;
        write_var_int_stream(&mut raw_tx, output_script_size)?;
        let mut script = vec![0; output_script_size as usize];
        stream.read_exact(&mut script)?;
        raw_tx.extend_from_slice(&script);
    }
    let mut locktime = [0; 4];
    stream.read_exact(&mut locktime)?;
    raw_tx.extend_from_slice(&locktime);
    Ok(raw_tx)
}
