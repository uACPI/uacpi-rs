use crate::{PhysAddr, Status};

#[repr(C)]
pub enum SleepState {
    S0 = 0,
    S1 = 1,
    S2 = 2,
    S3 = 3,
    S4 = 4,
    S5 = 5,
}

#[cfg(not(feature = "reduced-hardware"))]
/// Sets the firmware waking vector in FACS.
/// `addr32` is the real mode entry-point address
/// `addr64` is the protected mode entry-point address
pub fn set_waking_vector(addr32: PhysAddr, addr64: PhysAddr) -> Result<(), Status> {
    let status: Status = unsafe { uacpi_sys::uacpi_set_waking_vector(addr32.0, addr64.0).into() };

    match status {
        Status::Ok => Ok(()),
        _ => Err(status)
    }
}

#[cfg(feature = "reduced-hardware")]
/// Sets the firmware waking vector in FACS.
/// `addr32` is the real mode entry-point address
/// `addr64` is the protected mode entry-point address
pub fn set_waking_vector(addr32: PhysAddr, addr64: PhysAddr) -> Result<(), Status> {
    Err(Status::CompiledOut)
}

/// Prepare for a given sleep state.
/// Must be called with interrupts ENABLED.
pub fn prepare_for_sleep(state: SleepState) -> Result<(), Status> {
    let status: Status = unsafe { uacpi_sys::uacpi_prepare_for_sleep_state(state as _).into() };

    match status {
        Status::Ok => Ok(()),
        _ => Err(status)
    }
}

/// Enter the given sleep state after preparation.
/// Must be called with interrupts DISABLED.
pub fn enter_sleep(state: SleepState) -> Result<(), Status> {
    let status: Status = unsafe { uacpi_sys::uacpi_enter_sleep_state(state as _).into() };

    match status {
        Status::Ok => Ok(()),
        _ => Err(status)
    }
}

/// Prepare to leave the given sleep state.
/// Must be called with interrupts DISABLED.
pub fn prepare_for_wake_from_sleep(state: SleepState) -> Result<(), Status> {
    let status: Status = unsafe { uacpi_sys::uacpi_prepare_for_wake_from_sleep_state(state as _).into() };

    match status {
        Status::Ok => Ok(()),
        _ => Err(status)
    }
}

/// Wake from the given sleep state.
/// Must be called with interrupts ENABLED.
pub fn wake_from_sleep(state: SleepState) -> Result<(), Status> {
    let status: Status = unsafe { uacpi_sys::uacpi_wake_from_sleep_state(state as _).into() };

    match status {
        Status::Ok => Ok(()),
        _ => Err(status)
    }
}

/// Attempt reset via the FADT reset register.
pub fn reboot() -> Result<(), Status> {
    let status: Status = unsafe { uacpi_sys::uacpi_reboot().into() };

    match status {
        Status::Ok => Ok(()),
        _ => Err(status)
    }
}
