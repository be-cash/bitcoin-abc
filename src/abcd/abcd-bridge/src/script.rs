use crate::Script;

pub enum ScriptKind<'a> {
    P2PKH(&'a [u8]),
    P2SH(&'a [u8]),
    DATA(&'a [u8]),
    Other,
}

impl Script {
    pub fn kind(&self) -> ScriptKind {
        use crate::opcode::*;
        match self.bytecode() {
            [OP_DUP, OP_HASH160, 20, hash @ .., OP_EQUALVERIFY, OP_CHECKSIG] => {
                ScriptKind::P2PKH(hash)
            }
            [OP_HASH160, 20, hash @ .., OP_EQUAL] => ScriptKind::P2SH(hash),
            [OP_RETURN, payload @ ..] => ScriptKind::DATA(payload),
            _ => ScriptKind::Other,
        }
    }
}
