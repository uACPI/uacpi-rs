use core::ffi::c_char;

use crate::tables::{AcpiHeader, AcpiTable, AcpiTableInternal, AcpiTableSignature, AcpiTableStruct, AcpiTableMulti};


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TEMPLATE(AcpiTableStruct);

impl TEMPLATE {
    //Implement Table here
}



impl AcpiTable for TEMPLATE {
    
    const TABLE_SIGNATURE: [c_char;4] = AcpiTableSignature!(b"TEST"); //<< Change Table Signature here

}

impl AcpiTableInternal for TEMPLATE {
    
    fn internal_acpi_table_to_self(table: AcpiTableStruct ) -> Self where Self: Sized {
        Self(table)
    }
    
    fn internal_get_acpi_table(&self) -> &AcpiTableStruct {
        &self.0
    }
    
    fn internal_start_of_data(&self) -> *const u8 {
        unsafe { self.0.table_ptr.byte_add(size_of::<AcpiHeader>()) }
    }

}

impl AcpiTableMulti for TEMPLATE {}
