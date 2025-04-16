
use core::ffi::c_char;

use super::{AcpiTable, AcpiTableStruct};




type AcpiHpet = AcpiTableStruct;


impl AcpiHpet {
    //TODO Hpet getter
}

impl AcpiTable for AcpiHpet {
    //const TABLE_SIGNATURE: [core::ffi::c_char;4] = [110,120,105,124]; //HPET //TODO find a way to directly write chars/string there instead of numbers
    const TABLE_SIGNATURE: [c_char;4] = unsafe { *(c"HPET".as_ptr() as *const [c_char;4]) };
}