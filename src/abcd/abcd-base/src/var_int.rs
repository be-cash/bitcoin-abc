use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read, Result, Write};
use std::mem::size_of;

pub fn read_var_int(slice: &[u8]) -> Result<(u64, usize)> {
    read_var_int_stream(&mut Cursor::new(slice))
}

pub fn read_var_int_stream(stream: &mut impl Read) -> Result<(u64, usize)> {
    let first_byte = stream.read_u8()?;
    match first_byte {
        0..=0xfc => Ok((first_byte as u64, size_of::<u8>())),
        0xfd => Ok((
            stream.read_u16::<LittleEndian>()? as u64,
            size_of::<u8>() + size_of::<u16>(),
        )),
        0xfe => Ok((
            stream.read_u32::<LittleEndian>()? as u64,
            size_of::<u8>() + size_of::<u32>(),
        )),
        0xff => Ok((
            stream.read_u64::<LittleEndian>()?,
            size_of::<u8>() + size_of::<u64>(),
        )),
    }
}

pub fn var_int_size(number: u64) -> usize {
    match number {
        0..=0xfc => size_of::<u8>(),
        0xfd..=0xffff => size_of::<u8>() + size_of::<u16>(),
        0x10000..=0xffff_ffff => size_of::<u8>() + size_of::<u32>(),
        _ => size_of::<u8>() + size_of::<u64>(),
    }
}

pub fn write_var_int(slice: &mut [u8], number: u64) -> Result<usize> {
    write_var_int_stream(&mut Cursor::new(slice), number)
}

pub fn write_var_int_stream(stream: &mut impl Write, number: u64) -> Result<usize> {
    Ok(match number {
        0..=0xfc => {
            stream.write_u8(number as u8)?;
            size_of::<u8>()
        }
        0xfd..=0xffff => {
            stream.write_u8(0xfd)?;
            stream.write_u16::<LittleEndian>(number as u16)?;
            size_of::<u8>() + size_of::<u16>()
        }
        0x10000..=0xffff_ffff => {
            stream.write_u8(0xfe)?;
            stream.write_u32::<LittleEndian>(number as u32)?;
            size_of::<u8>() + size_of::<u32>()
        }
        _ => {
            stream.write_u8(0xff)?;
            stream.write_u64::<LittleEndian>(number as u64)?;
            size_of::<u8>() + size_of::<u64>()
        }
    })
}
