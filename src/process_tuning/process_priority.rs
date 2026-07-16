#[cfg(target_os = "windows")]
pub fn set_process_priority() -> std::io::Result<()> {
    use windows_sys::Win32::Foundation::HANDLE;
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, HIGH_PRIORITY_CLASS, SetPriorityClass};

    unsafe {
        let process: HANDLE = GetCurrentProcess();

        if SetPriorityClass(process, HIGH_PRIORITY_CLASS) == 0 {
            return Err(std::io::Error::last_os_error());
        }
    }

    Ok(())
}

#[cfg(target_os = "linux")]
pub fn set_process_priority() -> std::io::Result<()> {
    unsafe {
        // -10 requests higher priority; may require additional privileges.
        if libc::setpriority(libc::PRIO_PROCESS, 0, -10) != 0 {
            return Err(std::io::Error::last_os_error());
        }
    }

    Ok(())
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub fn set_process_priority() -> std::io::Result<()> {
    Ok(())
}
