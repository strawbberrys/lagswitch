#[cfg(target_os = "windows")]

use std::process::Command;
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000; // https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags#CREATE_NO_WINDOW

pub fn enable() {
    Command::new("cmd")
        .args(["/C", "ipconfig", "/release"])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .expect("failed to enable");
}

pub fn disable() {
    Command::new("cmd")
        .args(["/C", "ipconfig", "/renew"])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .expect("failed to disable");
}

// Using netsh
// Get: netsh advfirewall show currentprofile firewallpolicy
// Enable: netsh advfirewall set currentprofile firewallpolicy BlockInbound,BlockOutbound
// Disable: netsh advfirewall set currentprofile firewallpolicy BlockInbound,AllowOutbound
