use core::{
    alloc::{AllocError, GlobalAlloc},
    ffi::{c_char, c_void, CStr},
    mem::{transmute, MaybeUninit}, ptr::{null_mut, slice_from_raw_parts_mut},
};

use alloc::{alloc::Allocator, boxed::Box, slice};

use crate::status::{Status, UacpiError};

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum InitLevel {
    Early = uacpi_sys::UACPI_INIT_LEVEL_EARLY,
    SubsystemInitialized = uacpi_sys::UACPI_INIT_LEVEL_SUBSYSTEM_INITIALIZED,
    NamespaceLoaded = uacpi_sys::UACPI_INIT_LEVEL_NAMESPACE_LOADED,
    NamespaceInitialized = uacpi_sys::UACPI_INIT_LEVEL_NAMESPACE_INITIALIZED,
}

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
enum LogLevel {
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

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PCIAddress {
    pub segment: u16,
    pub bus: u8,
    pub device: u8,
    pub function: u8,
}

///Generic Uacpi Handle
pub struct Handle(pub(crate) uacpi_sys::uacpi_handle);

//TODO
pub struct PciDeviceHandle(pub(crate) Handle);
//TODO
pub struct IOPortHandle(pub(crate) Handle);

pub struct NamespaceNode(pub(crate) *mut uacpi_sys::uacpi_namespace_node);

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

impl From<ObjectType> for ObjectTypeBits {
    
    fn from(value: ObjectType) -> Self {
        unsafe { transmute(1 << value as i32) }
    }

}

pub struct Object(pub(crate) *mut uacpi_sys::uacpi_object);

impl Object {
    pub fn get_type(&self) -> ObjectType {
        unsafe { uacpi_sys::uacpi_object_get_type(self.0).into() }
    }

    pub fn is_one_of(&self, type_bits: i32) -> bool {
        unsafe { uacpi_sys::uacpi_object_is_one_of(self.0, type_bits) }
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

    pub fn new_uninitialized() -> Self {
        Self(unsafe { uacpi_sys::uacpi_object_create_uninitialized() })
    }

    pub fn new_integer(value: u64) -> Result<Self, UacpiError> {
        Self::new_integer_safe(value, OverflowBehavior::Allow)
    }

    pub fn new_integer_safe(
        value: u64,
        overflow_behavior: OverflowBehavior,
    ) -> Result<Self, UacpiError> {

        let mut ptr: *mut uacpi_sys::uacpi_object = core::ptr::null_mut();
        Status::evaluate_uacpi_status(unsafe {
            uacpi_sys::uacpi_object_create_integer_safe(value, overflow_behavior.into(), &mut ptr)
        })?;

        Ok(Self(ptr))

    }

    pub fn assign_integer(&self, value: u64) -> Result<(), UacpiError> {
        Status::evaluate_uacpi_status(unsafe { uacpi_sys::uacpi_object_assign_integer(self.0, value) })
    }

    pub fn get_integer(&self) -> Result<u64, UacpiError> {
        let mut value: u64 = 0;

        Status::evaluate_uacpi_status(unsafe { uacpi_sys::uacpi_object_get_integer(self.0, &mut value) })?;

        Ok(value)
    }

    
    pub fn new_string(value: &[c_char]) -> Self {

        let uacpi_slice = UacpiDataView::<c_char> {
            ptr: value.as_ptr(),
            length: value.len(),
        };
        Self(unsafe { uacpi_sys::uacpi_object_create_string(transmute(uacpi_slice)) })

    }

    pub fn new_cstring(value: &CStr) -> Self {

        Object(
            unsafe {
                uacpi_sys::uacpi_object_create_cstring(value.as_ptr())
            }
        )

    }

    pub fn new_buffer(value: &[u8]) -> Self {

        let uacpi_slice = UacpiDataView::<u8> {
            ptr: value.as_ptr(),
            length: value.len(),
        };
        Self(unsafe { uacpi_sys::uacpi_object_create_buffer(transmute(uacpi_slice)) })

    }

    pub fn get_string<'a>(&'a self) -> Result<&'a mut [c_char], UacpiError> {
        let mut uacpi_slice: UacpiDataViewMut::<c_char> = UacpiDataViewMut{ ptr: null_mut(), length: 0 };

        Status::evaluate_uacpi_status(unsafe {
            uacpi_sys::uacpi_object_get_string(self.0, transmute(&mut uacpi_slice))
        })?;

        unsafe {
            Ok(&mut *slice_from_raw_parts_mut(uacpi_slice.ptr, uacpi_slice.length))
        }

    }

    pub fn get_buffer<'a>(&'a self) -> Result<&'a mut [u8], UacpiError> {
        let mut uacpi_slice: UacpiDataViewMut::<u8> = UacpiDataViewMut{ ptr: null_mut(), length: 0 };

        Status::evaluate_uacpi_status(unsafe {
            uacpi_sys::uacpi_object_get_string(self.0, transmute(&mut uacpi_slice))
        })?;

        unsafe {
            Ok(&mut *slice_from_raw_parts_mut(uacpi_slice.ptr, uacpi_slice.length))
        }
    }

    pub fn is_aml_namepath(&self) -> bool {
        unsafe { uacpi_sys::uacpi_object_is_aml_namepath(self.0) }
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
            uacpi_sys::uacpi_object_assign_string(self.0, transmute(uacpi_slice))
        })
    }
    
    pub fn assign_buffer(&self, value: &[u8]) -> Result<(), UacpiError> {
        let uacpi_slice = UacpiDataView::<u8> {
            ptr: value.as_ptr(),
            length: value.len(),
        };

        Status::evaluate_uacpi_status(unsafe {
            uacpi_sys::uacpi_object_assign_buffer(self.0, transmute(uacpi_slice))
        })
    }

    pub fn create_package(content: Box<Object>) -> Self {

    }

    pub fn create_package_allocator<A>(content: Box<Object, A>) -> Self
    where
        A: Allocator,
    {
    }

    pub fn get_package(&self) -> Result<Box<Object>, Result<AllocError, Result<(), UacpiError>>> {}
    pub fn get_package_allocator<A: Allocator>(
        &self,
    ) -> Result<Box<Object, A>, Result<AllocError, Result<(), UacpiError>>> {
    }

    pub fn assign_package(&self, content: Box<Object>) -> Result<(), UacpiError> {}
    pub fn assign_package_allocator<A: core::alloc::Allocator>(
        &self,
        content: Box<Object, A>,
    ) -> Result<(), UacpiError> {
    }

    pub fn create_reference(&self) -> Object {}

    pub fn assign_reference(&self, child: &Object) -> Result<(), UacpiError> {}

    pub fn get_referenced_object(&self) -> Result<Object, UacpiError> {}

    pub fn get_processor_info(&self) -> Result<ProcessorInfo, UacpiError> {
        let mut output = ProcessorInfo {
            id: 0,
            block_address: 0,
            block_length: 0,
        };
        let mut output_ptr: *mut ProcessorInfo = &mut output;

        Status::evaluate_uacpi_status(unsafe {
            uacpi_sys::uacpi_object_get_processor_info(
                self.0,
                output_ptr as *mut uacpi_sys::uacpi_processor_info,
            )
        })?;

        Ok(output)
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
                self.0,
                output_ptr as *mut uacpi_sys::uacpi_power_resource_info,
            )
        })?;

        Ok(output)
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        unsafe {
            uacpi_sys::uacpi_object_ref(self.0);
        }
        Self(self.0)
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe {
            uacpi_sys::uacpi_object_unref(self.0);
        }
    }
}

#[repr(C)]
pub(crate) union ObjectName {
    text: [c_char; 4],
    id: u32,
}

#[repr(i32)]
pub enum OverflowBehavior {
    Allow = uacpi_sys::UACPI_OVERFLOW_ALLOW,
    Truncate = uacpi_sys::UACPI_OVERFLOW_TRUNCATE,
    Disallow = uacpi_sys::UACPI_OVERFLOW_DISALLOW,
}

impl Into<i32> for OverflowBehavior {
    fn into(self) -> i32 {
        match self {
            OverflowBehavior::Allow => uacpi_sys::UACPI_OVERFLOW_ALLOW,
            OverflowBehavior::Truncate => uacpi_sys::UACPI_OVERFLOW_TRUNCATE,
            OverflowBehavior::Disallow => uacpi_sys::UACPI_OVERFLOW_DISALLOW,
        }
    }
}

#[repr(C)]
struct UacpiDataView<T> {
    ptr: *const T,
    length: usize,
}

#[repr(C)]
struct UacpiDataViewMut<T> {
    ptr: *mut T,
    length: usize,
}

#[repr(C)]
struct ObjectArray {
    ptr: *mut Object,
    size: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ProcessorInfo {
    id: u8,
    block_address: u32,
    block_length: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PowerResourceInfo {
    system_level: u8,
    resource_order: u16,
}

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

#[repr(C)]
pub(crate) struct GenericRegionInfoInternal {}

#[repr(C)]
pub(crate) struct PCCRegionInfoInternal {}

#[repr(C)]
pub(crate) struct GPIORegionInfoInternal {}

#[repr(C)]
pub(crate) struct RegionAttachDataInternal {
    handler_context: *mut c_void,
    namespace_node: *mut uacpi_sys::uacpi_namespace_node,
    info: Region_Info,
    out_region_context: *mut c_void,
}

#[repr(C)]
pub(crate) union Region_Info {
    generic: GenericRegionInfoInternal,
    pcc: PCCRegionInfoInternal,
    gpio: GPIORegionInfoInternal,
}

#[repr(C)]
pub(crate) struct RegionRWDataInternal {
    handler_context: *mut c_void,
    region_context: *mut c_void,
    something: RegionRWDataUnionInternal,
    value: u64,
    byte_width: u8,
}

#[repr(C)]
pub(crate) union RegionRWDataUnionInternal {
    address: PhysAddr,
    offset: u64,
}

#[repr(C)]
pub(crate) struct RegionPCCSendDataInternal {}

#[repr(C)]
pub(crate) struct RegionGPIORWDataInternal {}

#[repr(C)]
pub(crate) struct RegionIPMIRWDataInternal {}

type RegionFFIXEDHWRWDataInternal = RegionIPMIRWDataInternal;

#[repr(C)]
pub(crate) struct RegionPRMRWDataInternal {}

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

#[repr(C)]
pub(crate) struct RegionSerialRWDataInternal {}

#[repr(C)]
pub(crate) struct RegionDetachDataInternal {
    handler_context: *mut c_void,
    region_context: *mut c_void,
    namespace_node: *mut uacpi_sys::uacpi_namespace_node,
}

pub(crate) type RegionHandlerInternal =
    fn(operation: RegionOp, op_data: uacpi_sys::uacpi_handle) -> Result<(), UacpiError>;

pub(crate) type NotifyHandlerInternal = fn(
    context: uacpi_sys::uacpi_handle,
    namespace_node: NamespaceNode,
    value: u64,
) -> Result<(), UacpiError>;

#[repr(i32)]
pub enum AddressSpace {
    SystemMemory = uacpi_sys::UACPI_ADDRESS_SPACE_SYSTEM_MEMORY,
    SystemIO = uacpi_sys::UACPI_ADDRESS_SPACE_SYSTEM_IO,
    PCIConfig = uacpi_sys::UACPI_ADDRESS_SPACE_PCI_CONFIG,
    EmbeddedController = uacpi_sys::UACPI_ADDRESS_SPACE_EMBEDDED_CONTROLLER,
    SMBus = uacpi_sys::UACPI_ADDRESS_SPACE_SMBUS,
    SystemCmos = uacpi_sys::UACPI_ADDRESS_SPACE_SYSTEM_CMOS,
    PCIBarTarget = uacpi_sys::UACPI_ADDRESS_SPACE_PCI_BAR_TARGET,
    IPMI = uacpi_sys::UACPI_ADDRESS_SPACE_IPMI,
    GeneralPurposeIO = uacpi_sys::UACPI_ADDRESS_SPACE_GENERAL_PURPOSE_IO,
    GenericSerialBus = uacpi_sys::UACPI_ADDRESS_SPACE_GENERIC_SERIAL_BUS,
    PCC = uacpi_sys::UACPI_ADDRESS_SPACE_PCC,
    PRM = uacpi_sys::UACPI_ADDRESS_SPACE_PRM,
    FFIXEDHW = uacpi_sys::UACPI_ADDRESS_SPACE_FFIXEDHW,
    VendorSpecific(i32),
}

impl From<i32> for AddressSpace {
    fn from(value: i32) -> Self {
        match value {
            uacpi_sys::UACPI_ADDRESS_SPACE_SYSTEM_MEMORY => AddressSpace::SystemMemory,
            uacpi_sys::UACPI_ADDRESS_SPACE_SYSTEM_IO => AddressSpace::SystemIO,
            uacpi_sys::UACPI_ADDRESS_SPACE_PCI_CONFIG => AddressSpace::PCIConfig,
            uacpi_sys::UACPI_ADDRESS_SPACE_EMBEDDED_CONTROLLER => AddressSpace::EmbeddedController,
            uacpi_sys::UACPI_ADDRESS_SPACE_SMBUS => AddressSpace::SMBus,
            uacpi_sys::UACPI_ADDRESS_SPACE_SYSTEM_CMOS => AddressSpace::SystemCmos,
            uacpi_sys::UACPI_ADDRESS_SPACE_PCI_BAR_TARGET => AddressSpace::PCIBarTarget,
            uacpi_sys::UACPI_ADDRESS_SPACE_IPMI => AddressSpace::IPMI,
            uacpi_sys::UACPI_ADDRESS_SPACE_GENERAL_PURPOSE_IO => AddressSpace::GeneralPurposeIO,
            uacpi_sys::UACPI_ADDRESS_SPACE_GENERIC_SERIAL_BUS => AddressSpace::GenericSerialBus,
            uacpi_sys::UACPI_ADDRESS_SPACE_PCC => AddressSpace::PCC,
            uacpi_sys::UACPI_ADDRESS_SPACE_PRM => AddressSpace::PRM,
            uacpi_sys::UACPI_ADDRESS_SPACE_FFIXEDHW => AddressSpace::FFIXEDHW,
            uacpi_sys::UACPI_ADDRESS_SPACE_TABLE_DATA => unreachable!("Internal data type leaked!"),
            _ => AddressSpace::VendorSpecific(value),
        }
    }
}

impl AddressSpace {
    pub fn as_str(&self) -> &str {
        match self {
            AddressSpace::SystemMemory => "SystemMemory",
            AddressSpace::SystemIO => "SystemIO",
            AddressSpace::PCIConfig => "PCI_Config",
            AddressSpace::EmbeddedController => "EmbeddedControl",
            AddressSpace::SMBus => "SMBus",
            AddressSpace::SystemCmos => "SystemCMOS",
            AddressSpace::PCIBarTarget => "PciBarTarget",
            AddressSpace::IPMI => "IPMI",
            AddressSpace::GeneralPurposeIO => "GeneralPurposeIO",
            AddressSpace::GenericSerialBus => "GenericSerialBus",
            AddressSpace::PCC => "PCC",
            AddressSpace::PRM => "PRM",
            AddressSpace::FFIXEDHW => "FFixedHW",
            AddressSpace::VendorSpecific(_) => "<vendor specific>",
        }
    }
}

#[repr(i32)]
pub enum FirmwareRequestType {
    Breackpoint = uacpi_sys::UACPI_FIRMWARE_REQUEST_TYPE_BREAKPOINT,
    Fatal = uacpi_sys::UACPI_FIRMWARE_REQUEST_TYPE_FATAL,
}

#[repr(C)]
pub(crate) struct FirmwareRequestInternal {}

#[repr(u32)]
pub enum InterruptRet {
    NotHandled = uacpi_sys::UACPI_INTERRUPT_NOT_HANDLED,
    Handled = uacpi_sys::UACPI_INTERRUPT_HANDLED,
}

pub(crate) type InterruptHandlerInternal = fn(handle: uacpi_sys::uacpi_handle) -> InterruptRet;

#[repr(i32)]
pub enum IterationDecision {
    Continue = uacpi_sys::UACPI_ITERATION_DECISION_CONTINUE,
    Break = uacpi_sys::UACPI_ITERATION_DECISION_BREAK,

    // Only applicable for uacpi_namespace_for_each_child
    NextPeer = uacpi_sys::UACPI_ITERATION_DECISION_NEXT_PEER,
}
