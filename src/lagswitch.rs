use std::process::Command;

#[cfg(target_os = "windows")]
pub fn enable() {
    Command::new("cmd")
        .args(["/C", "ipconfig", "/release"])
        .spawn()
        .expect("failed to enable");
}

#[cfg(target_os = "windows")]
pub fn disable() {
    Command::new("cmd")
        .args(["/C", "ipconfig", "/renew"])
        .spawn()
        .expect("failed to disable");
}

// Using netsh
// Get: netsh advfirewall show currentprofile firewallpolicy
// Enable: netsh advfirewall set currentprofile firewallpolicy BlockInbound,BlockOutbound
// Disable: netsh advfirewall set currentprofile firewallpolicy BlockInbound,AllowOutbound