
#[repr(C)]
pub(crate) struct Status(i32);

impl Status {
    
    pub(crate) fn evaluate_uacpi_status(value: i32) -> Result<(), UacpiError> {
        match value {
            uacpi_sys::UACPI_STATUS_OK => Ok(()),
            uacpi_sys::UACPI_STATUS_MAPPING_FAILED => Err(UacpiError::MappingFailed),
            uacpi_sys::UACPI_STATUS_OUT_OF_MEMORY => Err(UacpiError::OutOfMemory),
            uacpi_sys::UACPI_STATUS_BAD_CHECKSUM => Err(UacpiError::BadChecksum),
            uacpi_sys::UACPI_STATUS_INVALID_SIGNATURE => Err(UacpiError::InvalidSignature),
            uacpi_sys::UACPI_STATUS_INVALID_TABLE_LENGTH => Err(UacpiError::InvalidTableLength),
            uacpi_sys::UACPI_STATUS_NOT_FOUND => Err(UacpiError::NotFound),
            uacpi_sys::UACPI_STATUS_INVALID_ARGUMENT => Err(UacpiError::InvalidArgument),
            uacpi_sys::UACPI_STATUS_UNIMPLEMENTED => Err(UacpiError::Unimplemented),
            uacpi_sys::UACPI_STATUS_ALREADY_EXISTS => Err(UacpiError::AlreadyExists),
            uacpi_sys::UACPI_STATUS_INTERNAL_ERROR => Err(UacpiError::InternalError),
            uacpi_sys::UACPI_STATUS_TYPE_MISMATCH => Err(UacpiError::TypeMismatch),
            uacpi_sys::UACPI_STATUS_INIT_LEVEL_MISMATCH => Err(UacpiError::InitLevelMismatch),
            uacpi_sys::UACPI_STATUS_NAMESPACE_NODE_DANGLING => {
                Err(UacpiError::NamespaceNodeDangling)
            }
            uacpi_sys::UACPI_STATUS_NO_HANDLER => Err(UacpiError::NoHandler),
            uacpi_sys::UACPI_STATUS_NO_RESOURCE_END_TAG => Err(UacpiError::NoResourceEndTag),
            uacpi_sys::UACPI_STATUS_COMPILED_OUT => Err(UacpiError::CompiledOut),
            uacpi_sys::UACPI_STATUS_HARDWARE_TIMEOUT => Err(UacpiError::HardwareTimeout),
            uacpi_sys::UACPI_STATUS_TIMEOUT => Err(UacpiError::Timeout),
            uacpi_sys::UACPI_STATUS_OVERRIDDEN => Err(UacpiError::Overriden),
            uacpi_sys::UACPI_STATUS_DENIED => Err(UacpiError::Denied),
    
            uacpi_sys::UACPI_STATUS_AML_UNDEFINED_REFERENCE => {
                Err(UacpiError::AmlUndefindedReference)
            }
            uacpi_sys::UACPI_STATUS_AML_INVALID_NAMESTRING => Err(UacpiError::AmlInvalidNamestring),
            uacpi_sys::UACPI_STATUS_AML_OBJECT_ALREADY_EXISTS => {
                Err(UacpiError::AmlObjectAlreadyExists)
            }
            uacpi_sys::UACPI_STATUS_AML_INVALID_OPCODE => Err(UacpiError::AmlInvalidOpcode),
            uacpi_sys::UACPI_STATUS_AML_INCOMPATIBLE_OBJECT_TYPE => {
                Err(UacpiError::AmlIncompatibleObjectType)
            }
            uacpi_sys::UACPI_STATUS_AML_BAD_ENCODING => Err(UacpiError::AmlBadEncoding),
            uacpi_sys::UACPI_STATUS_AML_OUT_OF_BOUNDS_INDEX => Err(UacpiError::AmlOutOfBondsIndex),
            uacpi_sys::UACPI_STATUS_AML_SYNC_LEVEL_TOO_HIGH => Err(UacpiError::AmlSyncLevelTooHigh),
            uacpi_sys::UACPI_STATUS_AML_INVALID_RESOURCE => Err(UacpiError::AmlInvalidResource),
            uacpi_sys::UACPI_STATUS_AML_LOOP_TIMEOUT => Err(UacpiError::AmlLoopTimeout),
            uacpi_sys::UACPI_STATUS_AML_CALL_STACK_DEPTH_LIMIT => {
                Err(UacpiError::AmlCallStackDepthLimit)
            }
    
            _ => unreachable!("Undefined Value returned by UACPI"),
        }
    }

    pub(crate) fn to_status(value: Result<(),UacpiError>) -> Status {
        match value {
            Ok(()) => Self(0),
            Err(error) => Self(error as i32),
        }
    }


}


#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum UacpiError {
    MappingFailed = uacpi_sys::UACPI_STATUS_MAPPING_FAILED,
    OutOfMemory = uacpi_sys::UACPI_STATUS_OUT_OF_MEMORY,
    BadChecksum = uacpi_sys::UACPI_STATUS_BAD_CHECKSUM,
    InvalidSignature = uacpi_sys::UACPI_STATUS_INVALID_SIGNATURE,
    InvalidTableLength = uacpi_sys::UACPI_STATUS_INVALID_TABLE_LENGTH,
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
    Timeout = uacpi_sys::UACPI_STATUS_TIMEOUT,
    Overriden = uacpi_sys::UACPI_STATUS_OVERRIDDEN,
    Denied = uacpi_sys::UACPI_STATUS_DENIED,

    AmlUndefindedReference = uacpi_sys::UACPI_STATUS_AML_UNDEFINED_REFERENCE,
    AmlInvalidNamestring = uacpi_sys::UACPI_STATUS_AML_INVALID_NAMESTRING,
    AmlObjectAlreadyExists = uacpi_sys::UACPI_STATUS_AML_OBJECT_ALREADY_EXISTS,
    AmlInvalidOpcode = uacpi_sys::UACPI_STATUS_AML_INVALID_OPCODE,
    AmlIncompatibleObjectType = uacpi_sys::UACPI_STATUS_AML_INCOMPATIBLE_OBJECT_TYPE,
    AmlBadEncoding = uacpi_sys::UACPI_STATUS_AML_BAD_ENCODING,
    AmlOutOfBondsIndex = uacpi_sys::UACPI_STATUS_AML_OUT_OF_BOUNDS_INDEX,
    AmlSyncLevelTooHigh = uacpi_sys::UACPI_STATUS_AML_SYNC_LEVEL_TOO_HIGH,
    AmlInvalidResource = uacpi_sys::UACPI_STATUS_AML_INVALID_RESOURCE,
    AmlLoopTimeout = uacpi_sys::UACPI_STATUS_AML_LOOP_TIMEOUT,
    AmlCallStackDepthLimit = uacpi_sys::UACPI_STATUS_AML_CALL_STACK_DEPTH_LIMIT,
}

impl UacpiError {

    pub fn as_str(&self) -> &str {
        match self {
            UacpiError::MappingFailed => "failed to map memory",
            UacpiError::OutOfMemory => "out of memory",
            UacpiError::BadChecksum => "bad table checksum",
            UacpiError::InvalidSignature => "invalid table signature",
            UacpiError::InvalidTableLength => "invalid table length",
            UacpiError::NotFound => "not found",
            UacpiError::InvalidArgument => "invalid argument",
            UacpiError::Unimplemented => "unimplemented",
            UacpiError::AlreadyExists => "already exists",
            UacpiError::InternalError => "internal error",
            UacpiError::TypeMismatch => "object type mismatch",
            UacpiError::InitLevelMismatch => "init level to low/high for this action",
            UacpiError::NamespaceNodeDangling => "attempting to use a dangling namespace node",
            UacpiError::NoHandler => "no handler found",
            UacpiError::NoResourceEndTag => "resource template without an end tag",
            UacpiError::CompiledOut => "this functionality has been compiled out of this build",
            UacpiError::HardwareTimeout => "timed out waiting for hardware response",
            UacpiError::Timeout => "wait timed out",
            UacpiError::Overriden => "the requested action has been overridden",
            UacpiError::Denied => "the requested action has been denied",
            UacpiError::AmlUndefindedReference => "AML referenced an undefined object",
            UacpiError::AmlInvalidNamestring => "invalid AML name string",
            UacpiError::AmlObjectAlreadyExists => "object already exists",
            UacpiError::AmlInvalidOpcode => "invalid AML opcode",
            UacpiError::AmlIncompatibleObjectType => "incompatible AML object type",
            UacpiError::AmlBadEncoding => "bad AML instruction encoding",
            UacpiError::AmlOutOfBondsIndex => "out of bounds AML index",
            UacpiError::AmlSyncLevelTooHigh => {
                "AML attempted to acquire a mutex with a lower sync level"
            }
            UacpiError::AmlInvalidResource => "invalid resource template encoding or type",
            UacpiError::AmlLoopTimeout => "hanging AML while loop",
            UacpiError::AmlCallStackDepthLimit => "reached maximum AML call stack depth",
        }
    }
}
