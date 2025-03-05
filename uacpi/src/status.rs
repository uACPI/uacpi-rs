
#[repr(i32)]
#[derive(Clone,Copy,PartialEq, Eq, PartialOrd, Ord)]
pub enum Status{
    OK = uacpi_sys::UACPI_STATUS_OK,
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

impl TryFrom<i32> for Status {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            uacpi_sys::UACPI_STATUS_OK => Ok(Status::OK),
            uacpi_sys::UACPI_STATUS_MAPPING_FAILED => Ok(Status::MappingFailed),
            uacpi_sys::UACPI_STATUS_OUT_OF_MEMORY => Ok(Status::OutOfMemory),
            uacpi_sys::UACPI_STATUS_BAD_CHECKSUM => Ok(Status::BadChecksum),
            uacpi_sys::UACPI_STATUS_INVALID_SIGNATURE => Ok(Status::InvalidSignature),
            uacpi_sys::UACPI_STATUS_INVALID_TABLE_LENGTH => Ok(Status::InvalidTableLength),
            uacpi_sys::UACPI_STATUS_NOT_FOUND => Ok(Status::NotFound),
            uacpi_sys::UACPI_STATUS_INVALID_ARGUMENT => Ok(Status::InvalidArgument),
            uacpi_sys::UACPI_STATUS_UNIMPLEMENTED => Ok(Status::Unimplemented),
            uacpi_sys::UACPI_STATUS_ALREADY_EXISTS => Ok(Status::AlreadyExists),
            uacpi_sys::UACPI_STATUS_INTERNAL_ERROR => Ok(Status::InternalError),
            uacpi_sys::UACPI_STATUS_TYPE_MISMATCH => Ok(Status::TypeMismatch),
            uacpi_sys::UACPI_STATUS_INIT_LEVEL_MISMATCH => Ok(Status::InitLevelMismatch),
            uacpi_sys::UACPI_STATUS_NAMESPACE_NODE_DANGLING => Ok(Status::NamespaceNodeDangling),
            uacpi_sys::UACPI_STATUS_NO_HANDLER => Ok(Status::NoHandler),
            uacpi_sys::UACPI_STATUS_NO_RESOURCE_END_TAG => Ok(Status::NoResourceEndTag),
            uacpi_sys::UACPI_STATUS_COMPILED_OUT => Ok(Status::CompiledOut),
            uacpi_sys::UACPI_STATUS_HARDWARE_TIMEOUT => Ok(Status::HardwareTimeout),
            uacpi_sys::UACPI_STATUS_TIMEOUT => Ok(Status::Timeout),
            uacpi_sys::UACPI_STATUS_OVERRIDDEN => Ok(Status::Overriden),
            uacpi_sys::UACPI_STATUS_DENIED => Ok(Status::Denied),

            uacpi_sys::UACPI_STATUS_AML_UNDEFINED_REFERENCE => Ok(Status::AmlUndefindedReference),
            uacpi_sys::UACPI_STATUS_AML_INVALID_NAMESTRING => Ok(Status::AmlInvalidNamestring),
            uacpi_sys::UACPI_STATUS_AML_OBJECT_ALREADY_EXISTS => Ok(Status::AmlObjectAlreadyExists),
            uacpi_sys::UACPI_STATUS_AML_INVALID_OPCODE => Ok(Status::AmlInvalidOpcode),
            uacpi_sys::UACPI_STATUS_AML_INCOMPATIBLE_OBJECT_TYPE => Ok(Status::AmlIncompatibleObjectType),
            uacpi_sys::UACPI_STATUS_AML_BAD_ENCODING => Ok(Status::AmlBadEncoding),
            uacpi_sys::UACPI_STATUS_AML_OUT_OF_BOUNDS_INDEX => Ok(Status::AmlOutOfBondsIndex),
            uacpi_sys::UACPI_STATUS_AML_SYNC_LEVEL_TOO_HIGH => Ok(Status::AmlSyncLevelTooHigh),
            uacpi_sys::UACPI_STATUS_AML_INVALID_RESOURCE => Ok(Status::AmlInvalidResource),
            uacpi_sys::UACPI_STATUS_AML_LOOP_TIMEOUT => Ok(Status::AmlLoopTimeout),
            uacpi_sys::UACPI_STATUS_AML_CALL_STACK_DEPTH_LIMIT => Ok(Status::AmlCallStackDepthLimit),

            _ => Err(())
        }
    }
}

impl Status {
    pub fn to_string(&self) -> &str {
        match self {
            Status::OK => "no error",
            Status::MappingFailed => "failed to map memory",
            Status::OutOfMemory => "out of memory",
            Status::BadChecksum => "bad table checksum",
            Status::InvalidSignature => "invalid table signature",
            Status::InvalidTableLength => "invalid table length",
            Status::NotFound => "not found",
            Status::InvalidArgument => "invalid argument",
            Status::Unimplemented => "unimplemented",
            Status::AlreadyExists => "already exists",
            Status::InternalError => "internal error",
            Status::TypeMismatch => "object type mismatch",
            Status::InitLevelMismatch => "init level to low/high for this action",
            Status::NamespaceNodeDangling => "attempting to use a dangling namespace node",
            Status::NoHandler => "no handler found",
            Status::NoResourceEndTag => "resource template without an end tag",
            Status::CompiledOut => "this functionality has been compiled out of this build",
            Status::HardwareTimeout => "timed out waiting for hardware response",
            Status::Timeout => "wait timed out",
            Status::Overriden => "the requested action has been overridden",
            Status::Denied => "the requested action has been denied",
            Status::AmlUndefindedReference => "AML referenced an undefined object",
            Status::AmlInvalidNamestring => "invalid AML name string",
            Status::AmlObjectAlreadyExists => "object already exists",
            Status::AmlInvalidOpcode => "invalid AML opcode",
            Status::AmlIncompatibleObjectType => "incompatible AML object type",
            Status::AmlBadEncoding => "bad AML instruction encoding",
            Status::AmlOutOfBondsIndex => "out of bounds AML index",
            Status::AmlSyncLevelTooHigh => "AML attempted to acquire a mutex with a lower sync level",
            Status::AmlInvalidResource => "invalid resource template encoding or type",
            Status::AmlLoopTimeout => "hanging AML while loop",
            Status::AmlCallStackDepthLimit => "reached maximum AML call stack depth",
            _ => "<invalid status>",
        }
    }
}