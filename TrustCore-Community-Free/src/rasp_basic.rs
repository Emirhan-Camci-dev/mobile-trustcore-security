use obfstr::obfstring;
use std::path::Path;
use std::fs;

pub fn is_device_compromised() -> bool {
    let p1 = obfstring!("/Applications/Cydia.app");
    let p2 = obfstring!("/Library/MobileSubstrate/MobileSubstrate.dylib");
    let p3 = obfstring!("/system/bin/su");
    let p4 = obfstring!("/system/xbin/su");
    let p5 = obfstring!("/data/local/su");
    let p6 = obfstring!("/sbin/su");
    let dangerous_paths = [p1, p2, p3, p4, p5, p6];

    for path in dangerous_paths.iter() {
        if Path::new(path.as_str()).exists() { return true; }
    }
    false
}

pub fn is_emulator() -> bool {
    false // Demo mock
}

pub fn is_debugger_attached() -> bool {
    let path = obfstring!("/proc/self/status");
    if let Ok(status) = fs::read_to_string(path.as_str()) {
        let tracer = obfstring!("TracerPid:");
        for line in status.lines() {
            if line.starts_with(tracer.as_str()) {
                let pid: i32 = line.split_whitespace().nth(1).unwrap_or("0").parse().unwrap_or(0);
                if pid > 0 { return true; } 
            }
        }
    }
    false
}
