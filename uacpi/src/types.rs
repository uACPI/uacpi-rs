
use core::{alloc::{AllocError, GlobalAlloc}, ffi::{c_char, c_void, CStr}, mem::{transmute, MaybeUninit}};

use alloc::boxed::Box;

use crate::status::Status;



#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum InitLevel {
    Early = uacpi_sys::UACPI_INIT_LEVEL_EARLY,
    SubsystemInitialized = uacpi_sys::UACPI_INIT_LEVEL_SUBSYSTEM_INITIALIZED,
    NamespaceLoaded = uacpi_sys::UACPI_INIT_LEVEL_NAMESPACE_LOADED,
    NamespaceInitialized = uacpi_sys::UACPI_INIT_LEVEL_NAMESPACE_INITIALIZED
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum LogLevel{
    Debug = uacpi_sys::UACPI_LOG_DEBUG,
    Trace = uacpi_sys::UACPI_LOG_TRACE,
    Info = uacpi_sys::UACPI_LOG_INFO,
    Warn = uacpi_sys::UACPI_LOG_WARN,
    Error = uacpi_sys::UACPI_LOG_ERROR,
}

#[cfg(target_pointer_width = "64")]
pub type PhysAddr = u64;

#[cfg(target_pointer_width = "64")]
pub type IOAddr = u64;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PCIAddress{
    pub segment: u16,
    pub bus: u8,
    pub device: u8,
    pub function: u8
}

#[repr(C)]
struct DataView<T>{
    ptr: *mut T,
    size: uacpi_sys::uacpi_size
}

pub struct Handle(pub(crate) uacpi_sys::uacpi_handle);

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
            _ => core::unreachable!()
        }
    }
}

pub const OBJECT_TYPE_MAX_VALUE: i32 = uacpi_sys::UACPI_OBJECT_MAX_TYPE_VALUE;

#[repr(i32)]
pub enum ObjectTypeBits{
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
    Any = -1
}

pub struct Object(pub(crate) *mut uacpi_sys::uacpi_object);

impl Object {
    
    pub fn get_type(&self) -> ObjectType {
        unsafe { uacpi_sys::uacpi_object_get_type(self.0).into() }
    }

    pub fn is_one_of(&self, type_bits: i32) -> bool {
        unsafe { uacpi_sys::uacpi_object_is_one_of(self.0, type_bits)}
    }

    pub fn to_string(&self) -> &str {
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
        Self(
            unsafe {
                uacpi_sys::uacpi_object_create_uninitialized()
            }
        )
    }

    pub fn new_integer(value: u64) -> Self {
        Self::new_integer_safe(value, OverflowBehavior::Allow).expect("uACPI error")
    }

    pub fn new_integer_safe(value: u64, overflow_behavior: OverflowBehavior) -> Result<Self,()> {
        let mut ptr: *mut uacpi_sys::uacpi_object = core::ptr::null_mut();
        let ptr2: *mut *mut uacpi_sys::uacpi_object = &mut ptr;
        let status: Status = unsafe { uacpi_sys::uacpi_object_create_integer_safe(value, overflow_behavior.into(), ptr2).try_into().expect("uACPI Status Error") };

        match status {
            Status::OK => Ok(Self(ptr)),
            Status::InvalidArgument => Err(()),
            _ => unreachable!()
        }

    }

    pub fn assign_integer(&self, value: u64) -> Status {
        unsafe { uacpi_sys::uacpi_object_assign_integer(self.0, value).try_into().expect("uACPI Status Error") }
    }

    #[allow(unused_mut)]
    pub fn get_integer(&self) -> Result<u64,Status> {
        let mut value: u64 = 0;
        let mut ptr: *mut u64 = &mut value;

        let output = unsafe { uacpi_sys::uacpi_object_get_integer(self.0, ptr).try_into().expect("uACPI Status Error") };

        if output == Status::OK {
            return Ok(value);
        }

        Err(output)
    }

    pub fn new_string(value: &[c_char]) -> Self {}
    pub fn new_cstring(value: &CStr) -> Self {}
    pub fn new_buffer(value: &[u8]) -> Self {
        let uacpi_slice = UacpiDataView::<u8>{ ptr: value.as_ptr(), lenth: value.len() };
        Self(
            unsafe {
                uacpi_sys::uacpi_object_create_buffer(transmute(uacpi_slice))
            }
        )
    }

    pub fn get_string(&self) -> Result<&mut[c_char],Status> {}
    pub fn get_buffer(&self) -> Result<&mut[u8],Status> {}

    pub fn is_aml_namepath(&self) -> bool {
        unsafe { uacpi_sys::uacpi_object_is_aml_namepath(self.0) }
    }

    pub fn resolve_as_aml_namepath(&self, scope: &NamespaceNode) -> Result<NamespaceNode,Status> {}

    pub fn assign_string(&self, value: &mut[c_char]) -> Status {}
    pub fn assign_buffer(&self, value: &mut[u8]) -> Status {
        let uacpi_slice = UacpiDataView::<u8>{ ptr: value.as_ptr(), lenth: value.len() };

        unsafe { uacpi_sys::uacpi_object_assign_buffer(self.0, transmute(uacpi_slice)).try_into().expect("uACPI Status Error")}

    }

    pub fn create_package<A>(content: Box<Object,A>) -> Self where A : core::alloc::Allocator {}

    pub fn get_package(&self) -> Result<Box<Object>,Result<AllocError, Status>>{}
    pub fn get_package_allocator<A: core::alloc::Allocator>(&self) -> Result<Box<Object,A>,Result<AllocError, Status>>{}

    pub fn assign_package(&self, content: Box<Object>) -> Status {}
    pub fn assign_package_allocator<A: core::alloc::Allocator>(&self, content: Box<Object,A>) -> Status {}

    pub fn create_reference(&self) -> Object {}

    pub fn assign_reference(&self, child: &Object) -> Status {}

    pub fn get_referenced_object(&self) ->Result<Object,Status> {}

    #[allow(unused_mut)]
    pub fn get_processor_info(&self) -> Result<ProcessorInfo,Status> {
        let mut output = ProcessorInfo{ id: 0, block_address: 0, block_length: 0 };
        let mut output_ptr: *mut ProcessorInfo = &mut output;

        let output_status: Status = unsafe { uacpi_sys::uacpi_object_get_processor_info(self.0, output_ptr as *mut uacpi_sys::uacpi_processor_info).try_into().expect("uACPI Status Error") };
        
        if output_status == Status::OK {
            return Ok(output);
        }

        Err(output_status)
    }

    #[allow(unused_mut)]
    pub fn get_power_resource_info(&self) -> Result<PowerResourceInfo, Status> {
        let mut output = PowerResourceInfo{ system_level: 0, resource_order: 0 };
        let mut output_ptr: *mut PowerResourceInfo = &mut output;

        let output_status: Status = unsafe { uacpi_sys::uacpi_object_get_power_resource_info(self.0, output_ptr as *mut uacpi_sys::uacpi_power_resource_info).try_into().expect("") };
        
        if output_status == Status::OK {
            return Ok(output);
        }

        Err(output_status)
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

#[repr(i32)]
pub enum OverflowBehavior{
    Allow = uacpi_sys::UACPI_OVERFLOW_ALLOW,
    Truncate = uacpi_sys::UACPI_OVERFLOW_TRUNCATE,
    Disallow = uacpi_sys::UACPI_OVERFLOW_DISALLOW
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
    lenth: usize
}

#[repr(C)]
struct UacpiDataViewMut<T> {
    ptr: *mut T,
    lenth: usize
}

#[repr(C)]
struct ObjectArray{
    ptr: *mut Object,
    size: usize
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ProcessorInfo{
    id: u8,
    block_address: u32,
    block_length: u8
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PowerResourceInfo{
    system_level: u8,
    resource_order: u16
}

#[repr(i32)]
pub enum RegionOp {
    OpAttach = uacpi_sys::UACPI_REGION_OP_ATTACH,
    OpRead = uacpi_sys::UACPI_REGION_OP_READ,
    OpWrite = uacpi_sys::UACPI_REGION_OP_WRITE,
    OpDetach = uacpi_sys::UACPI_REGION_OP_DETACH,
}

#[repr(C)]
pub(crate) struct RegionAttachDataInternal{
    handler_context: *mut c_void,
    namespace_node: *mut uacpi_sys::uacpi_namespace_node,
    output: *mut c_void
}

#[repr(C)]
pub(crate) struct RegionRWDataInternal{
    handler_context: *mut c_void,
    region_context: *mut c_void,
    something: RegionRWDataUnionInternal,
    value: u64,
    byte_width: u8
}

pub(crate) union RegionRWDataUnionInternal {
    address: PhysAddr,
    offset: u64
}

#[repr(C)]
pub(crate) struct RegionDetachDataInternal{
    handler_context: *mut c_void,
    region_context: *mut c_void,
    namespace_node: *mut uacpi_sys::uacpi_namespace_node,
}

pub(crate) type RegionHandlerInternal = fn(operation: RegionOp, op_data: uacpi_sys::uacpi_handle) -> Status;

pub(crate) type NotifyHandlerInternal = fn(context: uacpi_sys::uacpi_handle, namespace_node: NamespaceNode, value: u64) -> Status;

#[repr(i32)]
pub enum AddressSpace{
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
}

impl AddressSpace {
    
    pub fn to_string(&self) -> &str {
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
        }
    }

}

#[repr(i32)]
pub enum FirmwareRequestType{
    Breackpoint = uacpi_sys::UACPI_FIRMWARE_REQUEST_TYPE_BREAKPOINT,
    Fatal = uacpi_sys::UACPI_FIRMWARE_REQUEST_TYPE_FATAL,
}

#[repr(u32)]
pub enum InterruptRet {
    NotHandled = uacpi_sys::UACPI_INTERRUPT_NOT_HANDLED,
    Handled = uacpi_sys::UACPI_INTERRUPT_HANDLED,
}

pub(crate) type InterruptHandlerInternal = fn(handle: uacpi_sys::uacpi_handle) -> InterruptRet;

#[repr(i32)]
pub enum IterationDecision{
    Continue = uacpi_sys::UACPI_ITERATION_DECISION_CONTINUE,
    Break = uacpi_sys::UACPI_ITERATION_DECISION_BREAK,

    // Only applicable for uacpi_namespace_for_each_child
    NextPeer = uacpi_sys::UACPI_ITERATION_DECISION_NEXT_PEER,
}