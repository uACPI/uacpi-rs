
use bitfield_struct::bitfield;

use crate::{tables::{AcpiHeader, AcpiTable, AcpiTableInternal, AcpiTableMulti, AcpiTableSignature, AcpiTableStruct}, types::{GenericAddressStructure, GenericAddressStructureInternal}};

use core::{ffi::c_char, ptr::addr_of};


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HPET(AcpiTableStruct);


impl HPET {
    
    pub fn get_hpet_hardware_rev_id(&self) -> u8 {

        unsafe {
            addr_of!(
                (
                    *(
                        self.internal_start_of_data() as *mut HpetInternal
                    )
                ).event_timer_block_id
            ).read_unaligned().hardware_rev_id()
        }

    }

    pub fn get_num_comparators(&self) -> u8 {

        unsafe {
            addr_of!(
                (
                    *(
                        self.internal_start_of_data() as *mut HpetInternal
                    )
                ).event_timer_block_id
            ).read_unaligned().num_comparators()
        }

    }

    pub fn get_count_size_cap(&self) -> bool {

        unsafe {
            addr_of!(
                (
                    *(
                        self.internal_start_of_data() as *mut HpetInternal
                    )
                ).event_timer_block_id
            ).read_unaligned().count_size_cap()
        }

    }

    pub fn get_legacy_replacement_irqrouting_capable(&self) -> bool {

        unsafe {
            addr_of!(
                (
                    *(
                        self.internal_start_of_data() as *mut HpetInternal
                    )
                ).event_timer_block_id
            ).read_unaligned().legacy_replacement_irqrouting_capable()
        }

    }

    pub fn get_pci_vendor_id(&self) -> u16 {

        unsafe {
            addr_of!(
                (
                    *(
                        self.internal_start_of_data() as *mut HpetInternal
                    )
                ).event_timer_block_id
            ).read_unaligned().pci_vendor_id()
        }

    }


    pub fn get_base_address(&self) -> GenericAddressStructure {
        
        unsafe { (*(self.internal_start_of_data() as *mut HpetInternal)).base_address_gas }.into()

    }


    pub fn get_hpet_number(&self) -> u8 {

        unsafe { (*(self.internal_start_of_data() as *mut HpetInternal)).hpet_number }

    }


    pub fn get_main_counter_minimum_clock_tick(&self) -> u16 {

        unsafe { (*(self.internal_start_of_data() as *mut HpetInternal)).main_counter_minimum_clock_tick }

    }


    pub fn get_page_protection_bits(&self) -> u8 {

        unsafe {
            addr_of!(
                (
                    *(
                        self.internal_start_of_data() as *mut HpetInternal
                    )
                ).page_protection_and_oem_attribute
            ).read_unaligned().page_protection()
        }

    }


    pub fn get_oem_attributes(&self) -> u8 {

        unsafe {
            addr_of!(
                (
                    *(
                        self.internal_start_of_data() as *mut HpetInternal
                    )
                ).page_protection_and_oem_attribute
            ).read_unaligned().oem_attributes()
        }

    }

    
}

#[repr(C, packed)]
struct HpetInternal {
    event_timer_block_id: EventTimerBlockID,
    base_address_gas: GenericAddressStructureInternal, //todo: add GAS
    hpet_number: u8,
    main_counter_minimum_clock_tick: u16,
    page_protection_and_oem_attribute: PageProtectionAndOEMAttribute
}

#[bitfield(u32)]
struct EventTimerBlockID {
    #[bits(8)]
    hardware_rev_id: u8, 
    #[bits(5)]
    num_comparators: u8,
    #[bits(1)]
    count_size_cap: bool,
    #[bits(1)]
    __:bool,
    #[bits(1)]
    legacy_replacement_irqrouting_capable: bool,
    #[bits(16)]
    pci_vendor_id: u16
}

#[bitfield(u8)]
struct PageProtectionAndOEMAttribute {
    #[bits(4)]
    page_protection: u8,
    #[bits(4)]
    oem_attributes: u8
}

impl AcpiTable for HPET {

    const TABLE_SIGNATURE: [c_char;4] = AcpiTableSignature!(b"HPET");

}

impl AcpiTableInternal for HPET {
    
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

impl AcpiTableMulti for HPET {}