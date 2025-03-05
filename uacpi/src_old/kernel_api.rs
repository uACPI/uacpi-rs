use crate::types::{
    FirmwareRequest, Handle, IOAddr, LogLevel, PCIAddress, PhysAddr, Status, WorkType,
    CpuFlags, ThreadId
};
use alloc::{
    alloc::{alloc, dealloc},
    boxed::Box,
    sync::Arc,
};
use core::{
    alloc::Layout,
    ffi::{c_char, c_void},
};
use core::ffi::CStr;
use log::{debug, error, info, trace, warn};

pub trait KernelApi {
    /// Reads a value of the specified byte width (1, 2, 4 or 8) from memory.
    unsafe fn raw_memory_read(&self, phys: PhysAddr, byte_width: u8) -> Result<u64, Status>;
    /// Writes a value of the specified byte width (1, 2, 4 or 8) to memory.
    unsafe fn raw_memory_write(
        &self,
        phys: PhysAddr,
        byte_width: u8,
        val: u64,
    ) -> Result<(), Status>;

    /// Reads a value of the specified byte width (1, 2 or 4) from io.
    unsafe fn raw_io_read(&self, addr: IOAddr, byte_width: u8) -> Result<u64, Status>;
    /// Writes a value of the specified byte width (1, 2 or 4) to io.
    unsafe fn raw_io_write(&self, addr: IOAddr, byte_width: u8, val: u64) -> Result<(), Status>;

    /// Reads a value of the specified byte width (1, 2 or 4)
    /// from a 0-based offset within the pci configuration space.
    /// Since PCI registers are 32 bits wide
    /// this must be able to handle e.g. 1-byte access by reading at the nearest
    /// 4-byte aligned offset below, then masking the value to select the target
    /// byte.
    unsafe fn pci_read(
        &self,
        address: PCIAddress,
        offset: usize,
        byte_width: u8,
    ) -> Result<u64, Status>;
    /// Writes a value of the specified byte width (1, 2 or 4) to
    /// a 0-based offset within the pci configuration space.
    /// Since PCI registers are 32 bits wide
    /// this must be able to handle e.g. 1-byte access by reading at the nearest
    /// 4-byte aligned offset below, then masking everything except the target byte
    /// and writing that value with the value put in the target byte back.
    unsafe fn pci_write(
        &self,
        address: PCIAddress,
        offset: usize,
        byte_width: u8,
        val: u64,
    ) -> Result<(), Status>;

    /// Maps a SystemIO address at [base, base + len] and return a handle
    /// that can be used for reading and writing to the IO range.
    unsafe fn io_map(&self, base: IOAddr, len: usize) -> Result<Handle, Status>;
    /// Unmaps an IO range previously mapped with io_map.
    unsafe fn io_unmap(&self, handle: Handle);
    /// Reads a value of the specified byte width (1, 2 or 4)
    /// from a 0-based offset within a mapped IO range.
    unsafe fn io_read(&self, handle: Handle, offset: usize, byte_width: u8) -> Result<u64, Status>;
    /// writes a value of the specified byte width (1, 2 or 4)
    /// to a 0-based offset within a mapped IO range.
    unsafe fn io_write(
        &self,
        handle: Handle,
        offset: usize,
        byte_width: u8,
        val: u64,
    ) -> Result<(), Status>;

    /// Creates a mapping used to access
    /// the physical range [phys, phys + len].
    unsafe fn map(&self, phys: PhysAddr, len: usize) -> *mut c_void;
    /// Unmaps a mapping previously returned from map.
    unsafe fn unmap(&self, addr: *mut c_void, len: usize);

    /// Allocates a block of zeroed memory.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        alloc(layout)
    }
    /// Deallocates a block of memory.
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        dealloc(ptr, layout)
    }

    #[cfg(feature = "logging")]
    fn log(&self, log_level: LogLevel, string: &str) {
        if log_level == LogLevel::TRACE {
            trace!("{string}");
        } else if log_level == LogLevel::DEBUG {
            debug!("{string}");
        } else if log_level == LogLevel::INFO {
            info!("{string}");
        } else if log_level == LogLevel::WARN {
            warn!("{string}");
        } else if log_level == LogLevel::ERROR {
            error!("{string}");
        }
    }

    #[cfg(not(feature = "logging"))]
    fn log(&self, log_level: LogLevel, string: &str);

    /// Returns the monotonic count of 100 nanosecond ticks elapsed since boot.
    fn get_ticks(&self) -> u64;

    /// Spins for the specified amount of microseconds.
    fn stall(&self, usec: u8);
    /// Sleeps for the specifier amount of milliseconds.
    fn sleep(&self, msec: u8);

    /// Creates a non-recursive kernel mutex.
    fn create_mutex(&self) -> Handle;
    /// Destroys a mutex previously created by create_mutex.
    fn destroy_mutex(&self, mutex: Handle);
    /// Tries to acquire a mutex with a millisecond timeout.
    /// A timeout value of 0xFFFF implies infinite wait.
    fn acquire_mutex(&self, mutex: Handle, timeout: u16) -> bool;
    /// Releases a previously acquired mutex.
    fn release_mutex(&self, mutex: Handle);

    /// Creates a spinlock.
    fn create_spinlock(&self) -> Handle;
    /// Destroys a spinlock previously created by create_spinlock.
    fn destroy_spinlock(&self, lock: Handle);
    /// Disables interrupts and acquires a spinlock.
    /// Returns the previous state of cpu flags that
    /// can be used to restore the interrupt state when the lock is released.
    fn acquire_spinlock(&self, lock: Handle) -> CpuFlags;
    /// Releases a spinlock and restores the previous interrupt state.
    fn release_spinlock(&self, lock: Handle, cpu_flags: CpuFlags);

    /// Creates a semaphore-line event object.
    fn create_event(&self) -> Handle;
    /// Destroys an event previously created by create_event.
    fn destroy_event(&self, event: Handle);
    /// Waits for an event (counter > 0) with a millisecond timeout.
    /// A timeout value of 0xFFFF implies infinite wait.
    /// The internal counter is decremented by 1 if the wait was successful.
    /// A successful wait is indicated by returning true.
    fn wait_for_event(&self, event: Handle, timeout: u16) -> bool;
    /// Signals an event by incrementing its internal counter by 1.
    /// This functions may be used in interrupt contexts.
    fn signal_event(&self, event: Handle);
    /// Resets an event by setting its internal counter to 0.
    fn reset_event(&self, event: Handle);

    /// Returns a unique identifier of the currently executing thread.
    fn get_thread_id(&self) -> ThreadId;

    /// Handles a firmware request.
    fn firmware_request(&self, req: FirmwareRequest) -> Result<(), Status>;

    /// Installs an interrupt handler for `irq`.
    /// The returned handle can be used to refer to this handler from other API.
    fn install_interrupt_handler(&self, irq: u32, handler: Box<dyn Fn()>,
    ) -> Result<Handle, Status>;
    /// Uninstalls an interrupt handler
    /// previously installed with install_interrupt_handler.
    fn uninstall_interrupt_handler(&self, handle: Handle) -> Result<(), Status>;

    /// Schedules deferred work for execution.
    /// Might be invoked from an interrupt context.
    fn schedule_work(&self, work_type: WorkType, handler: Box<dyn Fn()>) -> Result<(), Status>;

    /// Blocks until all scheduled work is complete and the work queue is empty.
    fn wait_for_work_completion(&self) -> Result<(), Status>;
}

static mut KERNEL_API: Option<Arc<dyn KernelApi>> = None;

pub fn set_kernel_api(api: Arc<dyn KernelApi>) {
    unsafe { KERNEL_API = Some(api) }
}

fn get_kernel_api() -> Arc<dyn KernelApi> {
    unsafe { KERNEL_API.as_ref().expect("No kernel api set").clone() }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_raw_memory_read(
    phys: uacpi_sys::uacpi_phys_addr,
    byte_width: u8,
    val: *mut u64,
) -> Status {
    match get_kernel_api().raw_memory_read(PhysAddr(phys), byte_width) {
        Ok(ret) => {
            *val = ret;
            Status::Ok
        }
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_raw_memory_write(
    phys: uacpi_sys::uacpi_phys_addr,
    byte_width: u8,
    val: u64,
) -> Status {
    match get_kernel_api().raw_memory_write(PhysAddr(phys), byte_width, val) {
        Ok(()) => Status::Ok,
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_raw_io_read(
    addr: uacpi_sys::uacpi_io_addr,
    byte_width: u8,
    val: *mut u64,
) -> Status {
    match get_kernel_api().raw_io_read(IOAddr(addr), byte_width) {
        Ok(ret) => {
            *val = ret;
            Status::Ok
        }
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_raw_io_write(
    addr: uacpi_sys::uacpi_io_addr,
    byte_width: u8,
    val: u64,
) -> Status {
    match get_kernel_api().raw_io_write(IOAddr(addr), byte_width, val) {
        Ok(()) => Status::Ok,
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_pci_read(
    address: *const uacpi_sys::uacpi_pci_address,
    offset: usize,
    byte_width: u8,
    val: *mut u64,
) -> Status {
    match get_kernel_api().pci_read(PCIAddress(*address), offset, byte_width) {
        Ok(ret) => {
            *val = ret;
            Status::Ok
        }
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_pci_write(
    address: *const uacpi_sys::uacpi_pci_address,
    offset: usize,
    byte_width: u8,
    val: u64,
) -> Status {
    match get_kernel_api().pci_write(PCIAddress(*address), offset, byte_width, val) {
        Ok(()) => Status::Ok,
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_io_map(
    base: uacpi_sys::uacpi_io_addr,
    len: usize,
    out_handle: *mut uacpi_sys::uacpi_handle,
) -> Status {
    match get_kernel_api().io_map(IOAddr(base), len) {
        Ok(ret) => {
            *out_handle = ret.0;
            Status::Ok
        }
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_io_unmap(handle: uacpi_sys::uacpi_handle) {
    get_kernel_api().io_unmap(Handle(handle))
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_io_read(
    handle: uacpi_sys::uacpi_handle,
    offset: usize,
    byte_width: u8,
    val: *mut u64,
) -> Status {
    match get_kernel_api().io_read(Handle(handle), offset, byte_width) {
        Ok(ret) => {
            *val = ret;
            Status::Ok
        }
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_io_write(
    handle: uacpi_sys::uacpi_handle,
    offset: usize,
    byte_width: u8,
    val: u64,
) -> Status {
    match get_kernel_api().io_write(Handle(handle), offset, byte_width, val) {
        Ok(()) => Status::Ok,
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_map(
    phys: uacpi_sys::uacpi_phys_addr,
    len: usize,
) -> *mut c_void {
    get_kernel_api().map(PhysAddr(phys), len)
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_unmap(addr: *mut c_void, len: usize) {
    get_kernel_api().unmap(addr, len)
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_alloc(size: usize) -> *mut c_void {
    get_kernel_api()
        .alloc(Layout::from_size_align(size, 8).unwrap())
        .cast()
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_calloc(count: usize, size: usize) -> *mut c_void {
    get_kernel_api()
        .alloc(Layout::from_size_align(count * size, 8).unwrap())
        .cast()
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_free(ptr: *mut c_void, size: usize) {
    if !ptr.is_null() {
        get_kernel_api().dealloc(ptr.cast(), Layout::from_size_align(size, 8).unwrap())
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_log(
    log_level: uacpi_sys::uacpi_log_level,
    str: *const c_char) {
    let s = CStr::from_ptr(str);
    get_kernel_api().log(LogLevel(log_level), s.to_str().unwrap());
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_get_ticks() -> u64 {
    get_kernel_api().get_ticks()
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_stall(usec: u8) {
    get_kernel_api().stall(usec)
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_sleep(msec: u8) {
    get_kernel_api().sleep(msec)
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_create_mutex() -> Handle {
    get_kernel_api().create_mutex()
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_free_mutex(mutex: Handle) {
    get_kernel_api().destroy_mutex(mutex)
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_acquire_mutex(mutex: Handle, timeout: u16) -> bool {
    get_kernel_api().acquire_mutex(mutex, timeout)
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_release_mutex(mutex: Handle) {
    get_kernel_api().release_mutex(mutex)
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_create_event() -> Handle {
    get_kernel_api().create_event()
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_free_event(event: Handle) {
    get_kernel_api().destroy_event(event)
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_wait_for_event(event: Handle, timeout: u16) -> bool {
    get_kernel_api().wait_for_event(event, timeout)
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_signal_event(event: Handle) {
    get_kernel_api().signal_event(event)
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_reset_event(event: Handle) {
    get_kernel_api().reset_event(event)
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_create_spinlock() -> Handle {
    get_kernel_api().create_spinlock()
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_free_spinlock(lock: Handle) {
    get_kernel_api().destroy_spinlock(lock)
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_spinlock_lock(lock: Handle) -> CpuFlags {
    get_kernel_api().acquire_spinlock(lock)
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_spinlock_unlock(lock: Handle, cpu_flags: CpuFlags) {
    get_kernel_api().release_spinlock(lock, cpu_flags)
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_get_thread_id() -> ThreadId {
    get_kernel_api().get_thread_id()
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_handle_firmware_request(
    req: *const uacpi_sys::uacpi_firmware_request,
) -> Status {
    match get_kernel_api().firmware_request(req.read().into()) {
        Ok(()) => Status::Ok,
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn uacpi_kernel_install_interrupt_handler(
    irq: u32,
    handler: extern "C" fn(Handle),
    ctx: Handle,
    out_irq_handle: *mut Handle,
) -> Status {
    match get_kernel_api().install_interrupt_handler(irq, Box::new(move || handler(ctx))) {
        Ok(val) => {
            *out_irq_handle = val;
            Status::Ok
        }
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_uninstall_interrupt_handler(
    _handler: extern "C" fn(Handle),
    irq_handle: Handle,
) -> Status {
    match get_kernel_api().uninstall_interrupt_handler(irq_handle) {
        Ok(()) => Status::Ok,
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_schedule_work(
    work_type: WorkType,
    handler: extern "C" fn(Handle),
    ctx: Handle,
) -> Status {
    match get_kernel_api().schedule_work(work_type, Box::new(move || handler(ctx))) {
        Ok(()) => Status::Ok,
        Err(status) => status,
    }
}

#[no_mangle]
pub(crate) extern "C" fn uacpi_kernel_wait_for_work_completion() -> Status {
    match get_kernel_api().wait_for_work_completion() {
        Ok(()) => Status::Ok,
        Err(status) => status,
    }
}
