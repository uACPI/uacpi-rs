use core::ffi::c_char;

use crate::{tables::{AcpiHeader, AcpiTable, AcpiTableInternal, AcpiTableMulti, AcpiTableSignature, AcpiTableStruct}, types::{GenericAddressStructure, GenericAddressStructurePacked}};


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DBGP(AcpiTableStruct);

impl DBGP {
    //Implement Table here

    pub fn get_interface_type(&self) -> InterfaceType {

        let value  = unsafe { 
            (
                *(self.internal_start_of_data() as *const DBGPInternal)
            ).interface_type };

        match value {
            0 => {
                InterfaceType::Full16550
            },
            1 => {
                InterfaceType::Subset16550
            },
            _ => {
                InterfaceType::Reserved(value)
            },
        }
    }

    pub fn get_base_address(&self) -> GenericAddressStructure {
        unsafe { 
            (
                *(self.internal_start_of_data() as *const DBGPInternal)
            ).base_address.into() 
        }
    }

}

#[repr(C, packed)]
struct DBGPInternal {
    interface_type: u8,
    rsvd: [u8; 3],
    base_address: GenericAddressStructurePacked
}

pub enum InterfaceType {
    Full16550,
    Subset16550,
    Reserved(u8)
}

impl AcpiTable for DBGP {
    
    const TABLE_SIGNATURE: [c_char;4] = AcpiTableSignature!(b"DBGP"); //<< Change Table Signature here

}

impl AcpiTableInternal for DBGP {
    
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

impl AcpiTableMulti for DBGP {}
