#[repr(transparent)]
pub struct NamespaceNode(pub(crate) *mut uacpi_sys::uacpi_namespace_node);

impl NamespaceNode {
    pub fn root() -> Self {
        Self(core::ptr::null_mut())
    }

    pub unsafe fn from_raw(ptr: *mut uacpi_sys::uacpi_namespace_node) -> Self {
        Self(ptr)
    }
}
