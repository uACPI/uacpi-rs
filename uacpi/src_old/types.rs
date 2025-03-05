use core::ffi::CStr;
use core::fmt::Debug;
use core::slice;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogLevel(pub(crate) uacpi_sys::uacpi_log_level);

impl LogLevel {
    pub const DEBUG: LogLevel = LogLevel(uacpi_sys::UACPI_LOG_DEBUG);
    pub const TRACE: LogLevel = LogLevel(uacpi_sys::UACPI_LOG_TRACE);
    pub const INFO: LogLevel = LogLevel(uacpi_sys::UACPI_LOG_INFO);
    pub const WARN: LogLevel = LogLevel(uacpi_sys::UACPI_LOG_WARN);
    pub const ERROR: LogLevel = LogLevel(uacpi_sys::UACPI_LOG_ERROR);
}

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Handle(pub(crate) uacpi_sys::uacpi_handle);

impl Handle {
    /// Creates a new opaque kernel handle. Using 0 here is not allowed.
    pub fn new(handle: u64) -> Handle {
        assert_ne!(
            handle,
            0,
            "using 0 for success is not allowed, if you want an invalid handle use invalid() instead");
        Handle(handle as _)
    }

    /// Creates a new invalid kernel handle.
    pub fn invalid() -> Handle {
        Handle(0 as _)
    }

    pub fn as_u64(self) -> u64 {
        self.0 as _
    }
}

impl Debug for Handle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#x}", self.as_u64())
    }
}

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct PhysAddr(pub(crate) uacpi_sys::uacpi_phys_addr);

impl PhysAddr {
    pub fn new(phys_addr: u64) -> PhysAddr {
        PhysAddr(phys_addr as _)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct ThreadId(pub(crate) uacpi_sys::uacpi_thread_id);

impl ThreadId {
    pub fn new(value: *mut core::ffi::c_void) -> Self {
        Self(value)
    }
}

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct CpuFlags(pub(crate) uacpi_sys::uacpi_cpu_flags);

impl CpuFlags {
    pub fn new(value: core::ffi::c_ulong) -> Self {
        Self(value)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InitLevel {
    Early = 0,
    SubsystemInitialized = 1,
    NamespaceLoaded = 2,
    NamespaceInitialized = 3,
}

impl From<uacpi_sys::uacpi_init_level> for InitLevel {
    fn from(level: uacpi_sys::uacpi_init_level) -> Self {
        match level {
            uacpi_sys::UACPI_INIT_LEVEL_EARLY => InitLevel::Early,
            uacpi_sys::UACPI_INIT_LEVEL_SUBSYSTEM_INITIALIZED => InitLevel::SubsystemInitialized,
            uacpi_sys::UACPI_INIT_LEVEL_NAMESPACE_LOADED => InitLevel::NamespaceLoaded,
            uacpi_sys::UACPI_INIT_LEVEL_NAMESPACE_INITIALIZED => InitLevel::NamespaceInitialized,
            _ => panic!("Unknown uacpi_init_level value: {:#x}", level),
        }
    }
}

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct IOAddr(pub(crate) uacpi_sys::uacpi_io_addr);

impl IOAddr {
    pub fn new(phys_addr: u64) -> IOAddr {
        IOAddr(phys_addr as _)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PCIAddress(pub(crate) uacpi_sys::uacpi_pci_address);

impl PCIAddress {
    pub fn segment(&self) -> u16 {
        self.0.segment
    }

    pub fn bus(&self) -> u8 {
        self.0.bus
    }

    pub fn device(&self) -> u8 {
        self.0.device
    }

    pub fn function(&self) -> u8 {
        self.0.function
    }
}

impl Debug for PCIAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:x}:{:x}:{:x}:{:x}",
            self.segment(),
            self.bus(),
            self.device(),
            self.function()
        )
    }
}

#[must_use]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    Ok = uacpi_sys::UACPI_STATUS_OK,
    MappingFailed = uacpi_sys::UACPI_STATUS_MAPPING_FAILED,
    OutOfMemory = uacpi_sys::UACPI_STATUS_OUT_OF_MEMORY,
    BadChecksum = uacpi_sys::UACPI_STATUS_BAD_CHECKSUM,
    InvalidSignature = uacpi_sys::UACPI_STATUS_INVALID_SIGNATURE,
    InvalidTableLenght = uacpi_sys::UACPI_STATUS_INVALID_TABLE_LENGTH,
    NotFound = uacpi_sys::UACPI_STATUS_NOT_FOUND,
    InvalidArgument = uacpi_sys::UACPI_STATUS_INVALID_ARGUMENT,
    Unimplemented = uacpi_sys::UACPI_STATUS_UNIMPLEMENTED,
    AlreadyExists = uacpi_sys::UACPI_STATUS_ALREADY_EXISTS,
    InternalError = uacpi_sys::UACPI_STATUS_INTERNAL_ERROR,
    TypeMismatch = uacpi_sys::UACPI_STATUS_TYPE_MISMATCH,
    InitLevelMismatch = uacpi_sys::UACPI_STATUS_INIT_LEVEL_MISMATCH,
    NamespaceNodeDangling = uacpi_sys::UACPI_STATUS_NAMESPACE_NODE_DANGLING,
    NoHandler = uacpi_sys::UACPI_STATUS_NO_HANDLER,
    NoResourceEndTag = uacpi_sys::UACPI_STATUS_NO_RESOURCE_END_TAG,
    CompiledOut = uacpi_sys::UACPI_STATUS_COMPILED_OUT,
    HardwareTimeout = uacpi_sys::UACPI_STATUS_HARDWARE_TIMEOUT,
    AmlUndefinedReference = uacpi_sys::UACPI_STATUS_AML_UNDEFINED_REFERENCE,
    AmlInvalidNamestring = uacpi_sys::UACPI_STATUS_AML_INVALID_NAMESTRING,
    AmlObjectAlreadyExists = uacpi_sys::UACPI_STATUS_AML_OBJECT_ALREADY_EXISTS,
    AmlInvalidOpcode = uacpi_sys::UACPI_STATUS_AML_INVALID_OPCODE,
    AmlIncompatibleObjectType = uacpi_sys::UACPI_STATUS_AML_INCOMPATIBLE_OBJECT_TYPE,
    AmlBadEncoding = uacpi_sys::UACPI_STATUS_AML_BAD_ENCODING,
    AmlOutOfBoundsIndex = uacpi_sys::UACPI_STATUS_AML_OUT_OF_BOUNDS_INDEX,
    AmlSyncLevelTooHigh = uacpi_sys::UACPI_STATUS_AML_SYNC_LEVEL_TOO_HIGH,
    AmlInvalidResource = uacpi_sys::UACPI_STATUS_AML_INVALID_RESOURCE,
    AmlLoopTimeout = uacpi_sys::UACPI_STATUS_AML_LOOP_TIMEOUT,
}

impl From<uacpi_sys::uacpi_status> for Status {
    fn from(status: uacpi_sys::uacpi_status) -> Self {
        match status {
            uacpi_sys::UACPI_STATUS_OK => Status::Ok,
            uacpi_sys::UACPI_STATUS_MAPPING_FAILED => Status::MappingFailed,
            uacpi_sys::UACPI_STATUS_OUT_OF_MEMORY => Status::OutOfMemory,
            uacpi_sys::UACPI_STATUS_BAD_CHECKSUM => Status::BadChecksum,
            uacpi_sys::UACPI_STATUS_INVALID_SIGNATURE => Status::InvalidSignature,
            uacpi_sys::UACPI_STATUS_INVALID_TABLE_LENGTH => Status::InvalidTableLenght,
            uacpi_sys::UACPI_STATUS_NOT_FOUND => Status::NotFound,
            uacpi_sys::UACPI_STATUS_INVALID_ARGUMENT => Status::InvalidArgument,
            uacpi_sys::UACPI_STATUS_UNIMPLEMENTED => Status::Unimplemented,
            uacpi_sys::UACPI_STATUS_ALREADY_EXISTS => Status::AlreadyExists,
            uacpi_sys::UACPI_STATUS_INTERNAL_ERROR => Status::InternalError,
            uacpi_sys::UACPI_STATUS_TYPE_MISMATCH => Status::TypeMismatch,
            uacpi_sys::UACPI_STATUS_INIT_LEVEL_MISMATCH => Status::InitLevelMismatch,
            uacpi_sys::UACPI_STATUS_NAMESPACE_NODE_DANGLING => {
                Status::NamespaceNodeDangling
            }
            uacpi_sys::UACPI_STATUS_NO_HANDLER => Status::NoHandler,
            uacpi_sys::UACPI_STATUS_NO_RESOURCE_END_TAG => Status::NoResourceEndTag,
            uacpi_sys::UACPI_STATUS_COMPILED_OUT => Status::CompiledOut,
            uacpi_sys::UACPI_STATUS_HARDWARE_TIMEOUT => Status::HardwareTimeout,
            uacpi_sys::UACPI_STATUS_AML_UNDEFINED_REFERENCE => {
                Status::AmlUndefinedReference
            }
            uacpi_sys::UACPI_STATUS_AML_INVALID_NAMESTRING => {
                Status::AmlInvalidNamestring
            }
            uacpi_sys::UACPI_STATUS_AML_OBJECT_ALREADY_EXISTS => {
                Status::AmlObjectAlreadyExists
            }
            uacpi_sys::UACPI_STATUS_AML_INVALID_OPCODE => Status::AmlInvalidOpcode,
            uacpi_sys::UACPI_STATUS_AML_INCOMPATIBLE_OBJECT_TYPE => {
                Status::AmlIncompatibleObjectType
            }
            uacpi_sys::UACPI_STATUS_AML_BAD_ENCODING => Status::AmlBadEncoding,
            uacpi_sys::UACPI_STATUS_AML_OUT_OF_BOUNDS_INDEX => {
                Status::AmlOutOfBoundsIndex
            }
            uacpi_sys::UACPI_STATUS_AML_SYNC_LEVEL_TOO_HIGH => {
                Status::AmlSyncLevelTooHigh
            }
            uacpi_sys::UACPI_STATUS_AML_INVALID_RESOURCE => Status::AmlInvalidResource,
            uacpi_sys::UACPI_STATUS_AML_LOOP_TIMEOUT => Status::AmlLoopTimeout,
            _ => panic!("Unknown uacpi_status value: {:#x}", status),
        }
    }
}

#[derive(Debug)]
pub enum FirmwareRequest {
    Breakpoint { context: Handle },
    Fatal { typ: u8, code: u32, arg: u64 },
}

impl From<uacpi_sys::uacpi_firmware_request> for FirmwareRequest {
    fn from(value: uacpi_sys::uacpi_firmware_request) -> Self {
        match value.type_ as u32 {
            uacpi_sys::UACPI_FIRMWARE_REQUEST_TYPE_BREAKPOINT => {
                FirmwareRequest::Breakpoint {
                    context: Handle(unsafe { value.__bindgen_anon_1.breakpoint.ctx }),
                }
            }
            uacpi_sys::UACPI_FIRMWARE_REQUEST_TYPE_FATAL => {
                FirmwareRequest::Fatal {
                    typ: unsafe { value.__bindgen_anon_1.fatal.type_ },
                    code: unsafe { value.__bindgen_anon_1.fatal.code },
                    arg: unsafe { value.__bindgen_anon_1.fatal.arg },
                }
            }
            _ => panic!("Unknown uacpi_firmware_request_type: {:#x}", value.type_),
        }
    }
}

#[repr(u32)]
#[derive(Debug)]
pub enum WorkType {
    GPEExecution = uacpi_sys::UACPI_WORK_GPE_EXECUTION,
    Notification = uacpi_sys::UACPI_WORK_NOTIFICATION,
}

impl From<uacpi_sys::uacpi_work_type> for WorkType {
    fn from(value: uacpi_sys::uacpi_work_type) -> Self {
        match value {
            uacpi_sys::UACPI_WORK_GPE_EXECUTION => Self::GPEExecution,
            uacpi_sys::UACPI_WORK_NOTIFICATION => Self::Notification,
            _ => panic!("Unknown uacpi_work_type: {:#x}", value),
        }
    }
}

#[repr(transparent)]
pub struct Object(pub(crate) *mut uacpi_sys::uacpi_object);

impl Object {
    fn new(t: uacpi_sys::uacpi_object_type) -> Option<Self> {
        let ptr = unsafe {
            uacpi_sys::uacpi_create_object(t)
        };
        if !ptr.is_null() {
            Some(Self(ptr))
        } else {
            None
        }
    }

    pub fn new_int(value: u64) -> Option<Self> {
        unsafe {
            let s = Self::new(
                uacpi_sys::UACPI_OBJECT_INTEGER
            )?;
            (*s.0).__bindgen_anon_1.integer = value;
            Some(s)
        }
    }

    pub fn get_int(&self) -> Option<u64> {
        unsafe {
            if (*self.0).type_ != uacpi_sys::UACPI_OBJECT_INTEGER as u8 {
                None
            } else {
                Some((*self.0).__bindgen_anon_1.integer)
            }
        }
    }

    pub fn get_buffer(&self) -> Option<&[u8]> {
        unsafe {
            if (*self.0).type_ != uacpi_sys::UACPI_OBJECT_BUFFER as u8 {
                None
            } else {
                let buffer = (*self.0).__bindgen_anon_1.buffer;
                let slice = slice::from_raw_parts(
                    (*buffer).__bindgen_anon_1.byte_data,
                    (*buffer).size
                );
                Some(slice)
            }
        }
    }

    pub fn get_string(&self) -> Option<&CStr> {
        unsafe {
            if (*self.0).type_ != uacpi_sys::UACPI_OBJECT_STRING as u8 {
                None
            } else {
                let buffer = (*self.0).__bindgen_anon_1.buffer;
                let slice = slice::from_raw_parts(
                    (*buffer).__bindgen_anon_1.byte_data,
                    (*buffer).size
                );
                Some(CStr::from_bytes_with_nul(slice).unwrap())
            }
        }
    }

    pub fn get_package(&self) -> Option<impl Iterator<Item=Self>> {
        unsafe {
            if (*self.0).type_ != uacpi_sys::UACPI_OBJECT_PACKAGE as u8 {
                None
            } else {
                let pkg = (*self.0).__bindgen_anon_1.package;
                Some(slice::from_raw_parts(
                    (*pkg).objects,
                    (*pkg).count,
                ).iter().map(|obj| Self(*obj)))
            }
        }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe {
            uacpi_sys::uacpi_object_unref(self.0);
        }
    }
}
