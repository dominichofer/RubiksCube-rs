#[cfg(target_os = "windows")]
pub fn pin_process_to_core() -> std::io::Result<()> {
    use windows_sys::Win32::Foundation::HANDLE;
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, SetProcessAffinityMask};

    unsafe {
        let process: HANDLE = GetCurrentProcess();

        if SetProcessAffinityMask(process, 1) == 0 {
            return Err(std::io::Error::last_os_error());
        }
    }

    Ok(())
}

#[cfg(target_os = "linux")]
pub fn pin_process_to_core() -> std::io::Result<()> {
    unsafe {
        let mut cpu_set = std::mem::zeroed::<libc::cpu_set_t>();
        libc::CPU_ZERO(&mut cpu_set);
        libc::CPU_SET(0, &mut cpu_set);

        if libc::sched_setaffinity(0, std::mem::size_of::<libc::cpu_set_t>(), &cpu_set) != 0 {
            return Err(std::io::Error::last_os_error());
        }
    }

    Ok(())
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub fn pin_process_to_core() -> std::io::Result<()> {
    Ok(())
}
