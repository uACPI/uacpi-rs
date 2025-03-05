use core::ffi::{c_void, CStr};
use core::mem::MaybeUninit;
use crate::Status;

pub const RSDP_SIGNATURE: &'static CStr = c"RSD PTR ";
pub const RSDT_SIGNATURE: &'static CStr = c"RSDT";
pub const XSDT_SIGNATURE: &'static CStr = c"XSDT";
pub const MADT_SIGNATURE: &'static CStr = c"APIC";
pub const FADT_SIGNATURE: &'static CStr = c"FACP";
pub const FACS_SIGNATURE: &'static CStr = c"FACS";
pub const MCFG_SIGNATURE: &'static CStr = c"MCFG";
pub const HPET_SIGNATURE: &'static CStr = c"HPET";
pub const SRAT_SIGNATURE: &'static CStr = c"SRAT";
pub const SLIT_SIGNATURE: &'static CStr = c"SLIT";
pub const DSDT_SIGNATURE: &'static CStr = c"DSDT";
pub const SSDT_SIGNATURE: &'static CStr = c"SSDT";
pub const PSDT_SIGNATURE: &'static CStr = c"PSDT";
pub const ECDT_SIGNATURE: &'static CStr = c"ECDT";
pub type Gas = uacpi_sys::acpi_gas;
pub type Rsdp = uacpi_sys::acpi_rsdp;
pub type SdtHdr = uacpi_sys::acpi_sdt_hdr;
pub type Rsdt = uacpi_sys::acpi_rsdt;
pub type Xsdt = uacpi_sys::acpi_xsdt;
pub type EntryHdr = uacpi_sys::acpi_entry_hdr;
pub type Madt = uacpi_sys::acpi_madt;
pub type MadtLapic = uacpi_sys::acpi_madt_lapic;
pub type MadtIoapic = uacpi_sys::acpi_madt_ioapic;
pub type MadtIrqSourceOverride = uacpi_sys::acpi_madt_interrupt_source_override;
pub type MadtNmiSource = uacpi_sys::acpi_madt_nmi_source;
pub type MadtLapicNmi = uacpi_sys::acpi_madt_lapic_nmi;
pub type MadtLapicAddressOverride = uacpi_sys::acpi_madt_lapic_address_override;
pub type MadtIosapic = uacpi_sys::acpi_madt_iosapic;
pub type MadtLsapic = uacpi_sys::acpi_madt_lsapic;
pub type MadtPlatformIrqSource = uacpi_sys::acpi_madt_platform_interrupt_source;
pub type MadtX2apic = uacpi_sys::acpi_madt_x2apic;
pub type MadtX2apicNmi = uacpi_sys::acpi_madt_x2apic_nmi;
pub type MadtGicc = uacpi_sys::acpi_madt_gicc;
pub type MadtGicd = uacpi_sys::acpi_madt_gicd;
pub type MadtGicMsiFrame = uacpi_sys::acpi_madt_gic_msi_frame;
pub type MadtGicr = uacpi_sys::acpi_madt_gicr;
pub type MadtGicIts = uacpi_sys::acpi_madt_gic_its;
pub type MadtMultiprocessorWakeup = uacpi_sys::acpi_madt_multiprocessor_wakeup;
pub type MadtCorePic = uacpi_sys::acpi_madt_core_pic;
pub type MadtLioPic = uacpi_sys::acpi_madt_lio_pic;
pub type MadtHtPic = uacpi_sys::acpi_madt_ht_pic;
pub type MadtEioPic = uacpi_sys::acpi_madt_eio_pic;
pub type MadtMsiPic = uacpi_sys::acpi_madt_msi_pic;
pub type MadtBioPic = uacpi_sys::acpi_madt_bio_pic;
pub type MadtLpcPic = uacpi_sys::acpi_madt_lpc_pic;
pub type Srat = uacpi_sys::acpi_srat;
pub type SratProcessorAffinity = uacpi_sys::acpi_srat_processor_affinity;
pub type SratMemoryAffinity = uacpi_sys::acpi_srat_memory_affinity;
pub type SratX2apicAffinity = uacpi_sys::acpi_srat_x2apic_affinity;
pub type SratGiccAffinity = uacpi_sys::acpi_srat_gicc_affinity;
pub type SratGicItsAffinity = uacpi_sys::acpi_srat_gic_its_affinity;
pub type SratGenericAffinity = uacpi_sys::acpi_srat_generic_affinity;
pub type SratRintcAffinity = uacpi_sys::acpi_srat_rintc_affinity;
pub type Slit = uacpi_sys::acpi_slit;
pub type Gtdt = uacpi_sys::acpi_gtdt;
pub type GtdtEntryHdr = uacpi_sys::acpi_gtdt_entry_hdr;
pub type GtdtTimer = uacpi_sys::acpi_gtdt_timer;
pub type GtdtTimerEntry = uacpi_sys::acpi_gtdt_timer_entry;
pub type GtdtWatchdog = uacpi_sys::acpi_gtdt_watchdog;
pub type Fadt = uacpi_sys::acpi_fadt;
pub type Facs = uacpi_sys::acpi_facs;
pub type McfgAllocation = uacpi_sys::acpi_mcfg_allocation;
pub type Mcfg = uacpi_sys::acpi_mcfg;
pub type Hpet = uacpi_sys::acpi_hpet;
pub type Dsdt = uacpi_sys::acpi_dsdt;
pub type Ssdt = uacpi_sys::acpi_ssdt;
pub type Ecdt = uacpi_sys::acpi_ecdt;

#[repr(transparent)]
pub struct Table(pub(crate) uacpi_sys::uacpi_table);

impl Table {
    pub fn get_virt_addr(&self) -> *mut c_void {
        unsafe {
            self.0.__bindgen_anon_1.ptr
        }
    }

    pub fn get_index(&self) -> usize {
        self.0.index
    }
}

/// Finds a table with a given signature.
pub fn table_find_by_signature(signature: &CStr) -> Result<Table, Status> {
    let mut ret = MaybeUninit::uninit();
    let status: Status = unsafe {
        uacpi_sys::uacpi_table_find_by_signature(signature.as_ptr(), ret.as_mut_ptr()).into()
    };

    match status {
        Status::Ok => Ok(Table(unsafe { ret.assume_init() })),
        _ => Err(status)
    }
}

/// Returns the pointer to a sanitized internal version of FADT.
/// The revision is guaranteed to be correct. All of the registers are converted
/// to GAS format. Fields that might contain garbage are cleared.
pub fn table_fadt() -> Result<&'static Fadt, Status> {
    let mut ret = core::ptr::null_mut();
    let status: Status = unsafe {
        uacpi_sys::uacpi_table_fadt(&mut ret).into()
    };

    match status {
        Status::Ok => Ok(unsafe { &*(ret as *const _ as *const Fadt) }),
        _ => Err(status)
    }
}
