
/*
From https://github.com/am0nsec/HellsGate/blob/1d860c0734c0e35a2f026d9a04856ded19dfdf31/HellsGate/main.c#L92
DWORD64 djb2(PBYTE str) {
 	DWORD64 dw_hash = 0x7734773477347734;
 	INT c;
 	while (c = *str++)
 		dw_hash = ((dw_hash << 0x5) + dw_hash) + c;
   	return dw_hash;
}
*/

use crate::defs;

pub fn djb2(_str: &str) -> u64 {
    let mut dw_hash: u64 = defs::DW_HASH;
        
    for _chr in _str.as_bytes() {
        dw_hash = dw_hash.wrapping_shl(5).wrapping_add(dw_hash).wrapping_add(*_chr as u64);
    }
    return dw_hash;
}
