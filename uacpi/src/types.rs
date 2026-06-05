
#[cfg(feature = "aml_interpreter")]
use core::{
    ffi::{c_char, c_void, CStr},
    mem::{transmute, ManuallyDrop}, 
    ptr::{null_mut, slice_from_raw_parts_mut, NonNull},
    slice
};

#[cfg(feature = "alloc")]
use core::alloc::AllocError;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

#[cfg(feature = "allocator_api")]
use alloc::alloc::Allocator;
use bitfield_struct::bitfield;

#[cfg(feature = "aml_interpreter")]
use crate::status::{Status, UacpiError};

#[cfg(feature = "aml_interpreter")]
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum InitLevel {
    Early = uacpi_sys::UACPI_INIT_LEVEL_EARLY,
    SubsystemInitialized = uacpi_sys::UACPI_INIT_LEVEL_SUBSYSTEM_INITIALIZED,
    NamespaceLoaded = uacpi_sys::UACPI_INIT_LEVEL_NAMESPACE_LOADED,
    NamespaceInitialized = uacpi_sys::UACPI_INIT_LEVEL_NAMESPACE_INITIALIZED,
}

#[cfg(feature = "aml_interpreter")]
impl From<i32> for InitLevel {
    fn from(value: i32) -> Self {
        match value {
            uacpi_sys::UACPI_INIT_LEVEL_EARLY => InitLevel::Early,
            uacpi_sys::UACPI_INIT_LEVEL_SUBSYSTEM_INITIALIZED => InitLevel::SubsystemInitialized,
            uacpi_sys::UACPI_INIT_LEVEL_NAMESPACE_LOADED => InitLevel::NamespaceLoaded,
            uacpi_sys::UACPI_INIT_LEVEL_NAMESPACE_INITIALIZED => InitLevel::NamespaceInitialized,
            _ => unreachable!("Undefined Value returned by UACPI"),
        }
    }
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug = uacpi_sys::UACPI_LOG_DEBUG,
    Trace = uacpi_sys::UACPI_LOG_TRACE,
    Info = uacpi_sys::UACPI_LOG_INFO,
    Warn = uacpi_sys::UACPI_LOG_WARN,
    Error = uacpi_sys::UACPI_LOG_ERROR,
}

impl From<i32> for LogLevel {
    fn from(value: i32) -> Self {
        match value {
            uacpi_sys::UACPI_LOG_DEBUG => LogLevel::Debug,
            uacpi_sys::UACPI_LOG_TRACE => LogLevel::Trace,
            uacpi_sys::UACPI_LOG_INFO => LogLevel::Info,
            uacpi_sys::UACPI_LOG_WARN => LogLevel::Warn,
            uacpi_sys::UACPI_LOG_ERROR => LogLevel::Error,
            _ => unreachable!("Undefined Value returned by UACPI"),
        }
    }
}

#[cfg(target_pointer_width = "64")]
pub type PhysAddr = u64;

#[cfg(target_pointer_width = "64")]
pub type IOAddr = u64;

#[cfg(target_pointer_width = "32")]
compile_error!("Open a Issue on Github, i didnt really expect anyone to use this on a 32bit platform");


//Compacted Version
#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct GenericAddressStructureInternal {
    address_space_id: u8,
    register_bit_width: u8,
    register_bit_offset: u8,
    access_size: u8,
    address: u64
}

impl From<GenericAddressStructure> for  GenericAddressStructureInternal {
    fn from(value: GenericAddressStructure) -> Self {
        
        Self { 
            address_space_id: match value.address_space {
                AddressSpace::SystemMemory(_) => 0x00,
                AddressSpace::SystemIO(_) => 0x01,
                AddressSpace::PCIConfig(_) => 0x02,
                AddressSpace::EmbeddedController(_) => 0x03,
                AddressSpace::SMBus(_) => 0x04,
                AddressSpace::SystemCmos(_) => 0x05,
                AddressSpace::PCIBarTarget(_) => 0x06,
                AddressSpace::IPMI(_) => 0x007,
                AddressSpace::GeneralPurposeIO(_) => 0x08,
                AddressSpace::GenericSerialBus(_) => 0x09,
                AddressSpace::PCC(_) => 0x0A,
                AddressSpace::PRM(_) => 0x0B,
                AddressSpace::Reserved(id) => id,
                AddressSpace::FFIXEDHW(_) => 0x7F,
                AddressSpace::VendorSpecific(id) => id.0,
            },
            register_bit_width: value.register_bit_width,
            register_bit_offset: value.register_bit_offset,
            access_size: value.access_size as u8,
            address: match value.address_space {
                AddressSpace::SystemMemory(address) => address,
                AddressSpace::SystemIO(address) => address,
                AddressSpace::PCIConfig(pciconfig_space) => pciconfig_space.into(),
                AddressSpace::EmbeddedController(address) => address,
                AddressSpace::SMBus(address) => address,
                AddressSpace::SystemCmos(address) => address,
                AddressSpace::PCIBarTarget(pcibar_target) => pcibar_target.into(),
                AddressSpace::IPMI(address) => address,
                AddressSpace::GeneralPurposeIO(address) => address,
                AddressSpace::GenericSerialBus(address) => address,
                AddressSpace::PCC(address) => address,
                AddressSpace::PRM(address) => address,
                AddressSpace::Reserved(_) => 0,
                AddressSpace::FFIXEDHW(address) => address,
                AddressSpace::VendorSpecific(address) => address.1,
            }    
        }

    }
}

#[derive(Copy, Clone, Debug)]
pub struct GenericAddressStructure {
    address_space: AddressSpace,
    access_size: AccessSize,
    register_bit_width: u8,
    register_bit_offset: u8,
}

impl From<GenericAddressStructureInternal> for GenericAddressStructure {
    fn from(value: GenericAddressStructureInternal) -> Self {
        
        Self { 
            address_space: 
            match value.address_space_id {
                0x00 => AddressSpace::SystemMemory(value.address),
                0x01 => AddressSpace::SystemIO(value.address),
                0x02 => AddressSpace::PCIConfig(PCIConfigSpace(value.address)),
                0x03 => AddressSpace::EmbeddedController(value.address),
                0x04 => AddressSpace::SMBus(value.address),
                0x05 => AddressSpace::SystemCmos(value.address),
                0x06 => AddressSpace::PCIBarTarget(PCIBarTarget(value.address)),
                0x07 => AddressSpace::IPMI(value.address),
                0x08 => AddressSpace::GeneralPurposeIO(value.address),
                0x09 => AddressSpace::GenericSerialBus(value.address),
                0x0A => AddressSpace::PCC(value.address),
                0x0B => AddressSpace::PRM(value.address),
                0x0C ..= 0x7E => AddressSpace::Reserved(value.address_space_id),
                0x7F => AddressSpace::FFIXEDHW(value.address),
                0x80 ..= 0xFF => AddressSpace::VendorSpecific((value.address_space_id, value.address)),
            },
            access_size: value.access_size.into(),
            register_bit_width: value.register_bit_width,
            register_bit_offset: value.register_bit_offset 
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum AccessSize {
    Undefined = 0,
    Byte = 1,
    Word = 2,
    DWord = 3,
    QWord = 4 
}

impl From<u8> for AccessSize {
    fn from(value: u8) -> Self {
        match value {
            0 => AccessSize::Undefined,
            1 => AccessSize::Byte,
            2 => AccessSize::Word,
            3 => AccessSize::DWord,
            4 => AccessSize::QWord,
            _ => unreachable!("Invalid Value from uacpi"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum AddressSpace {
    SystemMemory(u64),
    SystemIO(u64),
    PCIConfig(PCIConfigSpace),
    EmbeddedController(u64),
    SMBus(u64),
    SystemCmos(u64),
    PCIBarTarget(PCIBarTarget),
    IPMI(u64),
    GeneralPurposeIO(u64),
    GenericSerialBus(u64),
    PCC(u64),
    PRM(u64),
    Reserved(u8), //0x0C ..= 0x7E
    FFIXEDHW(u64),
    VendorSpecific((u8, u64)), //0x80 ..= 0xFF
}

/*
impl From<u8> for AddressSpace {
    fn from(value: u8) -> Self {
        match value {
            0x00 => AddressSpace::SystemMemory,
            0x01 => AddressSpace::SystemIO,
            0x02 => AddressSpace::PCIConfig,
            0x03 => AddressSpace::EmbeddedController,
            0x04 => AddressSpace::SMBus,
            0x05 => AddressSpace::SystemCmos,
            0x06 => AddressSpace::PCIBarTarget,
            0x07 => AddressSpace::IPMI,
            0x08 => AddressSpace::GeneralPurposeIO,
            0x09 => AddressSpace::GenericSerialBus,
            0x0A => AddressSpace::PCC,
            0x0B => AddressSpace::PRM,
            0x0C ..= 0x7E => AddressSpace::Reserved(value),
            0x7F => AddressSpace::FFIXEDHW,
            0x80 ..= 0xFF => AddressSpace::VendorSpecific(value),
        }
    }
}

impl TryFrom<i32> for AddressSpace {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value > u8::MAX.into() { 
            Err(()) 
        } else {
            let value_u8 = value as u8;
            Ok(value_u8.into())
        }

    }
}

impl Into<u8> for AddressSpace {
    fn into(self) -> u8 {
        AddressSpace as u8
    }
}
*/
impl AddressSpace {
    pub fn as_str(&self) -> &str {
        match self {
            AddressSpace::SystemMemory(_) => "SystemMemory",
            AddressSpace::SystemIO(_) => "SystemIO",
            AddressSpace::PCIConfig(_) => "PCI_Config",
            AddressSpace::EmbeddedController(_) => "EmbeddedControl",
            AddressSpace::SMBus(_) => "SMBus",
            AddressSpace::SystemCmos(_) => "SystemCMOS",
            AddressSpace::PCIBarTarget(_) => "PciBarTarget",
            AddressSpace::IPMI(_) => "IPMI",
            AddressSpace::GeneralPurposeIO(_) => "GeneralPurposeIO",
            AddressSpace::GenericSerialBus(_) => "GenericSerialBus",
            AddressSpace::PCC(_) => "PCC",
            AddressSpace::PRM(_) => "PRM",
            AddressSpace::Reserved(_) => "<reserved>",
            AddressSpace::FFIXEDHW(_) => "FFixedHW",
            AddressSpace::VendorSpecific(_) => "<vendor specific>",
        }
    }
}


#[bitfield(u64)]
pub struct PCIConfigSpace {
    offset: u16,
    function: u16,
    device: u16,
    __: u16,
}


#[bitfield(u64)]
pub struct PCIBarTarget {
    #[bits(37)]
    offset: u64,
    #[bits(3)]
    bar_index: u8,
    #[bits(3)]
    function: u8,
    #[bits(5)]
    device: u8,
    #[bits(8)]
    bus: u8,
    #[bits(8)]
    segment: u8,
}


#[cfg(feature = "aml_interpreter")]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PCIAddress {
    pub segment: u16,
    pub bus: u8,
    pub device: u8,
    pub function: u8,
}

///Generic Uacpi Handle (replace with specific variants where possible and make this internal only)
#[cfg(feature = "aml_interpreter")]
pub struct Handle(pub(crate) uacpi_sys::uacpi_handle);

//TODO
#[cfg(feature = "aml_interpreter")]
pub struct PciDeviceHandle(pub(crate) Handle);

//TODO
#[cfg(feature = "aml_interpreter")]
pub struct IOPortHandle(pub(crate) Handle);

#[cfg(feature = "aml_interpreter")]
pub struct NamespaceNode(pub(crate) *mut uacpi_sys::uacpi_namespace_node);

#[cfg(feature = "aml_interpreter")]
#[repr(i32)]
pub enum ObjectType {
    Uninitialized = uacpi_sys::UACPI_OBJECT_UNINITIALIZED,
    Integer = uacpi_sys::UACPI_OBJECT_INTEGER,
    String = uacpi_sys::UACPI_OBJECT_STRING,
    Buffer = uacpi_sys::UACPI_OBJECT_BUFFER,
    Package = uacpi_sys::UACPI_OBJECT_PACKAGE,
    FieldUnit = uacpi_sys::UACPI_OBJECT_FIELD_UNIT,
    Device = uacpi_sys::UACPI_OBJECT_DEVICE,
    Event = uacpi_sys::UACPI_OBJECT_EVENT,
    Method = uacpi_sys::UACPI_OBJECT_METHOD,
    Mutex = uacpi_sys::UACPI_OBJECT_MUTEX,
    OperationRegion = uacpi_sys::UACPI_OBJECT_OPERATION_REGION,
    PowerResource = uacpi_sys::UACPI_OBJECT_POWER_RESOURCE,
    Processor = uacpi_sys::UACPI_OBJECT_PROCESSOR,
    ThermalZone = uacpi_sys::UACPI_OBJECT_THERMAL_ZONE,
    BufferField = uacpi_sys::UACPI_OBJECT_BUFFER_FIELD,
    Debug = uacpi_sys::UACPI_OBJECT_DEBUG,

    Reference = uacpi_sys::UACPI_OBJECT_REFERENCE,
    BufferIndex = uacpi_sys::UACPI_OBJECT_BUFFER_INDEX,
}

#[cfg(feature = "aml_interpreter")]
impl From<i32> for ObjectType {
    fn from(value: i32) -> Self {
        match value {
            uacpi_sys::UACPI_OBJECT_UNINITIALIZED => ObjectType::Uninitialized,
            uacpi_sys::UACPI_OBJECT_INTEGER => ObjectType::Integer,
            uacpi_sys::UACPI_OBJECT_STRING => ObjectType::String,
            uacpi_sys::UACPI_OBJECT_BUFFER => ObjectType::Buffer,
            uacpi_sys::UACPI_OBJECT_PACKAGE => ObjectType::Package,
            uacpi_sys::UACPI_OBJECT_FIELD_UNIT => ObjectType::FieldUnit,
            uacpi_sys::UACPI_OBJECT_DEVICE => ObjectType::Device,
            uacpi_sys::UACPI_OBJECT_EVENT => ObjectType::Event,
            uacpi_sys::UACPI_OBJECT_METHOD => ObjectType::Method,
            uacpi_sys::UACPI_OBJECT_MUTEX => ObjectType::Mutex,
            uacpi_sys::UACPI_OBJECT_OPERATION_REGION => ObjectType::OperationRegion,
            uacpi_sys::UACPI_OBJECT_POWER_RESOURCE => ObjectType::PowerResource,
            uacpi_sys::UACPI_OBJECT_PROCESSOR => ObjectType::Processor,
            uacpi_sys::UACPI_OBJECT_THERMAL_ZONE => ObjectType::ThermalZone,
            uacpi_sys::UACPI_OBJECT_BUFFER_FIELD => ObjectType::BufferField,
            uacpi_sys::UACPI_OBJECT_DEBUG => ObjectType::Debug,

            uacpi_sys::UACPI_OBJECT_REFERENCE => ObjectType::Reference,
            uacpi_sys::UACPI_OBJECT_BUFFER_INDEX => ObjectType::BufferIndex,
            _ => unreachable!("Undefined Value returned by UACPI"),
        }
    }
}

#[cfg(feature = "aml_interpreter")]
#[repr(i32)]
pub enum ObjectTypeBits {
    None = 0,
    Integer = (1 << ObjectType::Integer as i32),
    String = (1 << ObjectType::String as i32),
    Buffer = (1 << ObjectType::Buffer as i32),
    Package = (1 << ObjectType::Package as i32),
    FieldUnit = (1 << ObjectType::FieldUnit as i32),
    Device = (1 << ObjectType::Device as i32),
    Event = (1 << ObjectType::Event as i32),
    Method = (1 << ObjectType::Method as i32),
    Mutex = (1 << ObjectType::Mutex as i32),
    OperationRegion = (1 << ObjectType::OperationRegion as i32),
    PowerResource = (1 << ObjectType::PowerResource as i32),
    Processor = (1 << ObjectType::Processor as i32),
    ThermalZone = (1 << ObjectType::ThermalZone as i32),
    BufferField = (1 << ObjectType::BufferField as i32),
    Debug = (1 << ObjectType::Debug as i32),
    Reference = (1 << ObjectType::Reference as i32),
    BufferIndex = (1 << ObjectType::BufferIndex as i32),
    Any = -1,
}

#[cfg(feature = "aml_interpreter")]
impl From<ObjectType> for ObjectTypeBits {
    
    fn from(value: ObjectType) -> Self {
        unsafe { transmute(1 << value as i32) }
    }

}

#[cfg(feature = "aml_interpreter")]
#[repr(C)]
pub struct Object(pub(crate) NonNull<uacpi_sys::uacpi_object>);

#[cfg(feature = "aml_interpreter")]
impl Object {
    pub fn get_type(&self) -> ObjectType {
        unsafe { uacpi_sys::uacpi_object_get_type(self.0.as_ptr()).into() }
    }

    pub fn is_one_of(&self, type_bits: i32) -> bool {
        unsafe { uacpi_sys::uacpi_object_is_one_of(self.0.as_ptr(), type_bits) }
    }

    pub fn as_str(&self) -> &str {
        match self.get_type() {
            ObjectType::Uninitialized => "Uninitialized",
            ObjectType::Integer => "Integer",
            ObjectType::String => "String",
            ObjectType::Buffer => "Buffer",
            ObjectType::Package => "Package",
            ObjectType::FieldUnit => "Field Unit",
            ObjectType::Device => "Device",
            ObjectType::Event => "Event",
            ObjectType::Method => "Method",
            ObjectType::Mutex => "Mutex",
            ObjectType::OperationRegion => "Operation Region",
            ObjectType::PowerResource => "Power Resource",
            ObjectType::Processor => "Processor",
            ObjectType::ThermalZone => "Thermal Zone",
            ObjectType::BufferField => "Buffer Field",
            ObjectType::Debug => "Debug",
            ObjectType::Reference => "Reference",
            ObjectType::BufferIndex => "Buffer Index",
        }
    }

    pub fn new_uninitialized() -> Option<Self> {
        Some( Self( NonNull::new(unsafe { uacpi_sys::uacpi_object_create_uninitialized() } )?))
    }

    pub fn new_integer(value: u64) -> Result<Option<Self>, UacpiError> {
        Self::new_integer_safe(value, OverflowBehavior::Allow)
    }

    pub fn new_integer_safe(
        value: u64,
        overflow_behavior: OverflowBehavior,
    ) -> Result<Option<Self>, UacpiError> {

        let mut ptr: *mut uacpi_sys::uacpi_object = core::ptr::null_mut();
        Status::evaluate_uacpi_status(unsafe {
            uacpi_sys::uacpi_object_create_integer_safe(value, overflow_behavior.into(), &mut ptr)
        })?;

        //Workaround as Rust is apparently to fucking stupid to function correctly if the single line of code is directly used in Ok()
        let m  = |ptr: *mut uacpi_sys::uacpi_object| -> Option<Self> {
            Some(Self(NonNull::new(ptr)?))
        };

        Ok(
            m(ptr)
        )

    }

    pub fn assign_integer(&self, value: u64) -> Result<(), UacpiError> {
        Status::evaluate_uacpi_status(unsafe { uacpi_sys::uacpi_object_assign_integer(self.0.as_ptr(), value) })
    }

    pub fn get_integer(&self) -> Result<u64, UacpiError> {
        let mut value: u64 = 0;

        Status::evaluate_uacpi_status(unsafe { uacpi_sys::uacpi_object_get_integer(self.0.as_ptr(), &mut value) })?;

        Ok(value)
    }

    
    pub fn new_string(value: &[c_char]) -> Option<Self> {

        let uacpi_slice = UacpiDataView::<c_char> {
            ptr: value.as_ptr(),
            length: value.len(),
        };
        Some( Self( NonNull::new(unsafe { uacpi_sys::uacpi_object_create_string(transmute(uacpi_slice)) } )?))

    }

    pub fn new_cstring(value: &CStr) -> Option<Self> {
       
        Some( Self( NonNull::new(unsafe { uacpi_sys::uacpi_object_create_cstring(value.as_ptr()) } )?))

    }

    pub fn new_buffer(value: &[u8]) -> Option<Self> {

        let uacpi_slice = UacpiDataView::<u8> {
            ptr: value.as_ptr(),
            length: value.len(),
        };
        Some( Self( NonNull::new(unsafe { uacpi_sys::uacpi_object_create_buffer(transmute(uacpi_slice)) } )?))

    }

    pub fn get_string<'a>(&'a self) -> Result<&'a mut [c_char], UacpiError> {
        let mut uacpi_slice: UacpiDataViewMut::<c_char> = UacpiDataViewMut{ ptr: null_mut(), length: 0 };

        Status::evaluate_uacpi_status(unsafe {
            uacpi_sys::uacpi_object_get_string(self.0.as_ptr(), transmute(&mut uacpi_slice))
        })?;

        unsafe {
            Ok(&mut *slice_from_raw_parts_mut(uacpi_slice.ptr, uacpi_slice.length))
        }

    }

    pub fn get_buffer<'a>(&'a self) -> Result<&'a mut [u8], UacpiError> {
        let mut uacpi_slice: UacpiDataViewMut::<u8> = UacpiDataViewMut{ ptr: null_mut(), length: 0 };

        Status::evaluate_uacpi_status(unsafe {
            uacpi_sys::uacpi_object_get_string(self.0.as_ptr(), transmute(&mut uacpi_slice))
        })?;

        unsafe {
            Ok(&mut *slice_from_raw_parts_mut(uacpi_slice.ptr, uacpi_slice.length))
        }
    }

    pub fn is_aml_namepath(&self) -> bool {
        unsafe { uacpi_sys::uacpi_object_is_aml_namepath(self.0.as_ptr()) }
    }

    pub fn resolve_as_aml_namepath(
        &self,
        scope: &NamespaceNode,
    ) -> Result<NamespaceNode, UacpiError> {
    }

    pub fn assign_string(&self, value: &[c_char]) -> Result<(), UacpiError> {
        let uacpi_slice = UacpiDataView::<c_char> {
            ptr: value.as_ptr(),
            length: value.len(),
        };

        Status::evaluate_uacpi_status(unsafe {
            uacpi_sys::uacpi_object_assign_string(self.0.as_ptr(), transmute(uacpi_slice))
        })
    }
    
    pub fn assign_buffer(&self, value: &[u8]) -> Result<(), UacpiError> {
        let uacpi_slice = UacpiDataView::<u8> {
            ptr: value.as_ptr(),
            length: value.len(),
        };

        Status::evaluate_uacpi_status(unsafe {
            uacpi_sys::uacpi_object_assign_buffer(self.0.as_ptr(), transmute(uacpi_slice))
        })
    }

    #[cfg(feature = "alloc")]
    pub fn create_package(content: Box<Object>) -> Option<Self> {

    }

    #[cfg(feature = "allocator_api")]
    pub fn create_package_in<A>(content: Box<Object, A>) -> Option<Self>
    where
        A: Allocator,
    {
    }

    #[cfg(feature = "alloc")]
    pub fn get_package(&self) -> Result<Box<Object>, Result<AllocError, Result<(), UacpiError>>> {}

    #[cfg(feature = "allocator_api")]
    pub fn get_package_in<A: Allocator>(
        &self,
    ) -> Result<Box<Object, A>, Result<AllocError, Result<(), UacpiError>>> {
    }

    #[cfg(feature = "alloc")]
    pub fn assign_package(&self, content: Box<Object>) -> Result<(), UacpiError> {}

    #[cfg(feature = "allocator_api")]
    pub fn assign_package_in<A: core::alloc::Allocator>(
        &self,
        content: Box<Object, A>,
    ) -> Result<(), UacpiError> {
    }

    pub fn create_reference(&self) -> Option<Object> {}

    pub fn assign_reference(&self, child: &Object) -> Result<(), UacpiError> {}

    pub fn get_dereferenced_object(&self) -> Result<Option<Object>, UacpiError> {}

    #[allow(unused_mut)]
    pub fn get_processor_info(&self) -> Result<ProcessorInfo, UacpiError> {
        let mut output = ProcessorInfo {
            id: 0,
            block_address: 0,
            block_length: 0,
        };
        let mut output_ptr: *mut ProcessorInfo = &mut output;

        Status::evaluate_uacpi_status(unsafe {
            uacpi_sys::uacpi_object_get_processor_info(
                self.0.as_ptr(),
                output_ptr as *mut uacpi_sys::uacpi_processor_info,
            )
        })?;

        //Ok(output)

        compile_error!("WRONG")

    }

    #[allow(unused_mut)]
    pub fn get_power_resource_info(&self) -> Result<PowerResourceInfo, UacpiError> {
        let mut output = PowerResourceInfo {
            system_level: 0,
            resource_order: 0,
        };
        let mut output_ptr: *mut PowerResourceInfo = &mut output;

        Status::evaluate_uacpi_status(unsafe {
            uacpi_sys::uacpi_object_get_power_resource_info(
                self.0.as_ptr(),
                output_ptr as *mut uacpi_sys::uacpi_power_resource_info,
            )
        })?;

        compile_error!("WRONG");

        Ok(output)
    }
}
#[cfg(feature = "aml_interpreter")]
impl Clone for Object {
    fn clone(&self) -> Self {
        unsafe {
            uacpi_sys::uacpi_object_ref(self.0.as_ptr());
        }
        Self(self.0)
    }
}

#[cfg(feature = "aml_interpreter")]
impl Drop for Object {
    fn drop(&mut self) {
        unsafe {
            uacpi_sys::uacpi_object_unref(self.0.as_ptr());
        }
    }
}

#[cfg(feature = "aml_interpreter")]
#[repr(C)]
pub(crate) union ObjectName {
    text: [c_char; 4],
    id: u32,
}

#[cfg(feature = "aml_interpreter")]
#[repr(i32)]
pub enum OverflowBehavior {
    Allow = uacpi_sys::UACPI_OVERFLOW_ALLOW,
    Truncate = uacpi_sys::UACPI_OVERFLOW_TRUNCATE,
    Disallow = uacpi_sys::UACPI_OVERFLOW_DISALLOW,
}

#[cfg(feature = "aml_interpreter")]
impl Into<i32> for OverflowBehavior {
    fn into(self) -> i32 {
        match self {
            OverflowBehavior::Allow => uacpi_sys::UACPI_OVERFLOW_ALLOW,
            OverflowBehavior::Truncate => uacpi_sys::UACPI_OVERFLOW_TRUNCATE,
            OverflowBehavior::Disallow => uacpi_sys::UACPI_OVERFLOW_DISALLOW,
        }
    }
}

#[cfg(feature = "aml_interpreter")]
#[repr(C)]
struct UacpiDataView<T> {
    ptr: *const T,
    length: usize,
}

#[cfg(feature = "aml_interpreter")]
#[repr(C)]
struct UacpiDataViewMut<T> {
    ptr: *mut T,
    length: usize,
}

#[cfg(feature = "aml_interpreter")]
#[repr(C)]
struct ObjectArray {
    ptr: *mut Object,
    size: usize,
}

#[cfg(feature = "aml_interpreter")]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ProcessorInfo {
    id: u8,
    block_address: u32,
    block_length: u8,
}

#[cfg(feature = "aml_interpreter")]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PowerResourceInfo {
    system_level: u8,
    resource_order: u16,
}

#[cfg(feature = "aml_interpreter")]
#[repr(i32)]
pub enum RegionOp {
    Attach = uacpi_sys::UACPI_REGION_OP_ATTACH,
    Detach = uacpi_sys::UACPI_REGION_OP_DETACH,
    Read = uacpi_sys::UACPI_REGION_OP_READ,
    Write = uacpi_sys::UACPI_REGION_OP_WRITE,
    PccSend = uacpi_sys::UACPI_REGION_OP_PCC_SEND,
    GPIORead = uacpi_sys::UACPI_REGION_OP_GPIO_READ,
    GPIOWrite = uacpi_sys::UACPI_REGION_OP_GPIO_WRITE,
    IPMICommand = uacpi_sys::UACPI_REGION_OP_IPMI_COMMAND,
    FFIXEDHWCommand = uacpi_sys::UACPI_REGION_OP_FFIXEDHW_COMMAND,
    PRMCommand = uacpi_sys::UACPI_REGION_OP_PRM_COMMAND,
    SerialRead = uacpi_sys::UACPI_REGION_OP_SERIAL_READ,
    SerialWrite = uacpi_sys::UACPI_REGION_OP_SERIAL_WRITE,
}

#[cfg(feature = "aml_interpreter")]
impl From<i32> for RegionOp {
    fn from(value: i32) -> Self {
        match value {
            uacpi_sys::UACPI_REGION_OP_ATTACH => RegionOp::Attach,
            uacpi_sys::UACPI_REGION_OP_DETACH => RegionOp::Detach,
            uacpi_sys::UACPI_REGION_OP_READ => RegionOp::Read,
            uacpi_sys::UACPI_REGION_OP_WRITE => RegionOp::Write,
            uacpi_sys::UACPI_REGION_OP_PCC_SEND => RegionOp::PccSend,
            uacpi_sys::UACPI_REGION_OP_GPIO_READ => RegionOp::GPIORead,
            uacpi_sys::UACPI_REGION_OP_GPIO_WRITE => RegionOp::GPIOWrite,
            uacpi_sys::UACPI_REGION_OP_IPMI_COMMAND => RegionOp::IPMICommand,
            uacpi_sys::UACPI_REGION_OP_FFIXEDHW_COMMAND => RegionOp::FFIXEDHWCommand,
            uacpi_sys::UACPI_REGION_OP_PRM_COMMAND => RegionOp::PRMCommand,
            uacpi_sys::UACPI_REGION_OP_SERIAL_READ => RegionOp::SerialRead,
            uacpi_sys::UACPI_REGION_OP_SERIAL_WRITE => RegionOp::SerialWrite,

            _ => unreachable!("Undefined Value returned by UACPI "),
        }
    }
}

#[cfg(feature = "aml_interpreter")]
#[repr(C)]
pub(crate) struct GenericRegionInfoInternal {}

#[cfg(feature = "aml_interpreter")]
#[repr(C)]
pub(crate) struct PCCRegionInfoInternal {}

#[cfg(feature = "aml_interpreter")]
#[repr(C)]
pub(crate) struct GPIORegionInfoInternal {}

#[cfg(feature = "aml_interpreter")]
#[repr(C)]
pub(crate) struct RegionAttachDataInternal {
    handler_context: *mut c_void,
    namespace_node: *mut uacpi_sys::uacpi_namespace_node,
    info: Region_Info,
    out_region_context: *mut c_void,
}

#[cfg(feature = "aml_interpreter")]
#[repr(C)]
pub(crate) union Region_Info {
    generic: GenericRegionInfoInternal,
    pcc: PCCRegionInfoInternal,
    gpio: GPIORegionInfoInternal,
}

#[cfg(feature = "aml_interpreter")]
#[repr(C)]
pub(crate) struct RegionRWDataInternal {
    handler_context: *mut c_void,
    region_context: *mut c_void,
    something: RegionRWDataUnionInternal,
    value: u64,
    byte_width: u8,
}

#[cfg(feature = "aml_interpreter")]
#[repr(C)]
pub(crate) union RegionRWDataUnionInternal {
    address: PhysAddr,
    offset: u64,
}

#[cfg(feature = "aml_interpreter")]
#[repr(C)]
pub(crate) struct RegionPCCSendDataInternal {}

#[cfg(feature = "aml_interpreter")]
#[repr(C)]
pub(crate) struct RegionGPIORWDataInternal {}

#[cfg(feature = "aml_interpreter")]
#[repr(C)]
pub(crate) struct RegionIPMIRWDataInternal {}

#[cfg(feature = "aml_interpreter")]
type RegionFFIXEDHWRWDataInternal = RegionIPMIRWDataInternal;

#[cfg(feature = "aml_interpreter")]
#[repr(C)]
pub(crate) struct RegionPRMRWDataInternal {}

#[cfg(feature = "aml_interpreter")]
#[repr(i32)]
pub enum AccessAttribute {
    Quick = uacpi_sys::UACPI_ACCESS_ATTRIBUTE_QUICK,
    SendReceive = uacpi_sys::UACPI_ACCESS_ATTRIBUTE_SEND_RECEIVE,
    Byte = uacpi_sys::UACPI_ACCESS_ATTRIBUTE_BYTE,
    Word = uacpi_sys::UACPI_ACCESS_ATTRIBUTE_WORD,
    Block = uacpi_sys::UACPI_ACCESS_ATTRIBUTE_BLOCK,
    Bytes = uacpi_sys::UACPI_ACCESS_ATTRIBUTE_BYTES,
    ProcessCall = uacpi_sys::UACPI_ACCESS_ATTRIBUTE_PROCESS_CALL,
    BlockProcessCall = uacpi_sys::UACPI_ACCESS_ATTRIBUTE_BLOCK_PROCESS_CALL,
    RawBytes = uacpi_sys::UACPI_ACCESS_ATTRIBUTE_RAW_BYTES,
    RawProcessBytes = uacpi_sys::UACPI_ACCESS_ATTRIBUTE_RAW_PROCESS_BYTES,
}

#[cfg(feature = "aml_interpreter")]
#[repr(C)]
pub(crate) struct RegionSerialRWDataInternal {}

#[cfg(feature = "aml_interpreter")]
#[repr(C)]
pub(crate) struct RegionDetachDataInternal {
    handler_context: *mut c_void,
    region_context: *mut c_void,
    namespace_node: *mut uacpi_sys::uacpi_namespace_node,
}

#[cfg(feature = "aml_interpreter")]
pub(crate) type RegionHandlerInternal =
    fn(operation: RegionOp, op_data: uacpi_sys::uacpi_handle) -> Result<(), UacpiError>;

#[cfg(feature = "aml_interpreter")]
pub(crate) type NotifyHandlerInternal = fn(
    context: uacpi_sys::uacpi_handle,
    namespace_node: NamespaceNode,
    value: u64,
) -> Result<(), UacpiError>;



#[cfg(feature = "aml_interpreter")]
#[repr(i32)]
pub enum FirmwareRequestType {
    Breackpoint = uacpi_sys::UACPI_FIRMWARE_REQUEST_TYPE_BREAKPOINT,
    Fatal = uacpi_sys::UACPI_FIRMWARE_REQUEST_TYPE_FATAL,
}

#[cfg(feature = "aml_interpreter")]
#[repr(C)]
pub(crate) struct FirmwareRequestInternal {}

#[cfg(feature = "aml_interpreter")]
#[repr(u32)]
pub enum InterruptRet {
    NotHandled = uacpi_sys::UACPI_INTERRUPT_NOT_HANDLED,
    Handled = uacpi_sys::UACPI_INTERRUPT_HANDLED,
}

#[cfg(feature = "aml_interpreter")]
pub(crate) type InterruptHandlerInternal = fn(handle: uacpi_sys::uacpi_handle) -> InterruptRet;

#[cfg(feature = "aml_interpreter")]
#[repr(i32)]
pub enum IterationDecision {
    Continue = uacpi_sys::UACPI_ITERATION_DECISION_CONTINUE,
    Break = uacpi_sys::UACPI_ITERATION_DECISION_BREAK,

    // Only applicable for uacpi_namespace_for_each_child
    NextPeer = uacpi_sys::UACPI_ITERATION_DECISION_NEXT_PEER,
}
