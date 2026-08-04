

//For some reason bindgen cant import UACPI_MAP_FAILED so here is it manually :/
pub const MAP_FAILED: usize = usize::MAX;

/// # FUNCTION SIGNATURES
/// 
/// To leave a api function unimplemented use ()
/// 
/// kernel_api!(
///     kernel_get_rsdp = ()
/// )
/// 
/// ## Expected Function Signatures:
/// 
/// kernel_get_rsdp => fn(out: PhysAddr) -> UacpiError
/// 
/// kernel_map      => fn(addr: PhysAddr, len: usize) -> Option<*mut u8> //Return 'None' if mapping failed
/// 
/// kernel_unmap    => fn(addr: PhysAddr, len: usize)
/// 
/// kernel_log      => fn(level: LogLevel, message: &'a str)
/// 
/// 
/// # IMPLEMENTATION NOTE
/// 
/// ## kernel_map/kernel_unmap
/// NOTE: 'addr' may be misaligned, in this case the host is expected to round it
///     down to the nearest page-aligned boundary and map that, while making
///     sure that at least 'len' bytes are still mapped starting at 'addr'. The
///     return value preserves the misaligned offset.
/// 
///     Example for uacpi_kernel_map(0x1ABC, 0xF00):
///          1. Round down the 'addr' we got to the nearest page boundary.
///             Considering a PAGE_SIZE of 4096 (or 0x1000), 0x1ABC rounded down
///             is 0x1000, offset within the page is 0x1ABC - 0x1000 => 0xABC
///          2. Requested 'len' is 0xF00 bytes, but we just rounded the address
///             down by 0xABC bytes, so add those on top. 0xF00 + 0xABC => 0x19BC
///          3. Round up the final 'len' to the nearest PAGE_SIZE boundary, in
///             this case 0x19BC is 0x2000 bytes (2 pages if PAGE_SIZE is 4096)
///          4. Call the VMM to map the aligned address 0x1000 (from step 1)
///             with length 0x2000 (from step 3). Let's assume the returned
///             virtual address for the mapping is 0xF000.
///          5. Add the original offset within page 0xABC (from step 1) to the
///             resulting virtual address 0xF000 + 0xABC => 0xFABC. Return it
///             to uACPI.
/// 
/// 
#[cfg(not(feature = "full_acpi_hardware"))]
#[macro_export]
macro_rules! kernel_api {
    (
        kernel_get_rsdp = $get_rsdp:tt,
        kernel_map = $map:tt,
        kernel_unmap = $unmap:tt,
        kernel_log = $log:tt,
    ) => {
        kernel_api!(@get_rsdp $get_rsdp);
        kernel_api!(@map $map);
        kernel_api!(@unmap $unmap);
        kernel_api!(@log $log);
    };

    (@get_rsdp ()) => {

        #[unsafe(no_mangle)]
        pub extern "C" fn uacpi_kernel_get_rsdp(out: *$crate::types::PhysAddr) -> i32 {
            $crate::status::UacpiError::Unimplemented
        }

    };

    (@get_rsdp $f:expr) => {

        #[unsafe(no_mangle)]
        pub extern "C" fn uacpi_kernel_get_rsdp(out: *$crate::types::PhysAddr) -> i32 {
            
            let f: fn(out: *$crate::types::PhysAddr) -> $crate::status::UacpiError = $f;

            f(out).into()

        }

    };


    (@map ()) => {
        
        #[unsafe(no_mangle)]
        pub extern "C" uacpi_kernel_map(addr: $crate::types::PhysAddr, len: usize) -> *mut u8 {
            core::ptr::null_mut()
        }

    };

    (@map $f:expr) => {
        
        #[unsafe(no_mangle)]
        pub extern "C" uacpi_kernel_map(addr: $crate::types::PhysAddr, len: usize) -> *mut u8 {
            
            let f: fn(addr: $crate::types::PhysAddr, len: usize) -> Option<*mut u8> = $f;

            match f(addr, len) {
                Some(value) => {
                    return value;
                }
                None => {
                    return  $crate::kernel_api::MAP_FAILED as *mut u8;
                }
            }

        }

    };


    (@unmap ()) => {

        #[unsafe(no_mangle)]
        pub extern "C" uacpi_kernel_unmap(addr: *mut u8, len: usize) {
            //Do nothing
        }

    };

    (@unmap $f:expr) => {

        #[unsafe(no_mangle)]
        pub extern "C" uacpi_kernel_unmap(addr: *mut u8, len: usize) {
            
            let f: fn(addr: $crate::types::PhysAddr, len: usize) = $f;

            f(addr, len)

        }
    
    };


    (@log ()) => {

        #[unsafe(no_mangle)]
        pub extern "C" fn uacpi_kernel_log(level: i32, log_message: *const core::ffi::c_char) {
            //Do nothing
        }

    };

    (@log $f:expr) => {

        #[unsafe(no_mangle)]
        pub extern "C" fn uacpi_kernel_log(level: i32, log_message: *const core::ffi::c_char) {
            
            let f: fn(level: $crate::types::LogLevel, message: &'a str) = $f;

            f(
                $crate::types::LogLevel::from(level),
                unsafe { core::ffi::CStr::from_ptr(char_ptr) }.to_str().unwrap()
            );

        }

    };





}


/// # IMPLEMENTATION NOTE
/// 
/// ## kernel_map/kernel_unmap
/// NOTE: 'addr' may be misaligned, in this case the host is expected to round it
///     down to the nearest page-aligned boundary and map that, while making
///     sure that at least 'len' bytes are still mapped starting at 'addr'. The
///     return value preserves the misaligned offset.
/// 
///     Example for uacpi_kernel_map(0x1ABC, 0xF00):
///          1. Round down the 'addr' we got to the nearest page boundary.
///             Considering a PAGE_SIZE of 4096 (or 0x1000), 0x1ABC rounded down
///             is 0x1000, offset within the page is 0x1ABC - 0x1000 => 0xABC
///          2. Requested 'len' is 0xF00 bytes, but we just rounded the address
///             down by 0xABC bytes, so add those on top. 0xF00 + 0xABC => 0x19BC
///          3. Round up the final 'len' to the nearest PAGE_SIZE boundary, in
///             this case 0x19BC is 0x2000 bytes (2 pages if PAGE_SIZE is 4096)
///          4. Call the VMM to map the aligned address 0x1000 (from step 1)
///             with length 0x2000 (from step 3). Let's assume the returned
///             virtual address for the mapping is 0xF000.
///          5. Add the original offset within page 0xABC (from step 1) to the
///             resulting virtual address 0xF000 + 0xABC => 0xFABC. Return it
///             to uACPI.
/// 
/// 
/// 
#[cfg(feature = "full_acpi_hardware")]
#[macro_export]
macro_rules! kernel_api {
    () => {
        
    };
}