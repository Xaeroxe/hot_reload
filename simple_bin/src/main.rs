use std::fs::{copy, metadata};
use std::time::SystemTime;
use libloading::{Library, Result, Symbol};

fn main() -> Result<()> {
    copy("..\\simple_math\\target\\release\\simple_math.dll", "simple_math.dll")?;
    let mut last_time = SystemTime::now();
    let mut lib = Some(Library::new("simple_math.dll")?);
    let mut func: Option<Symbol<extern fn(i32) -> i32>>;
    unsafe {
        func = Some(lib.as_ref().unwrap().get(b"double\0")?);
    }
    loop {
        println!("Output: {}", func.as_ref().unwrap()(2));
        if should_reload(&mut last_time) {
            lib = None;
            func = None;
            while copy("..\\simple_math\\target\\release\\simple_math.dll", "simple_math.dll").is_err() {}
            println!("Reloading DLL...");
            lib = Some(Library::new("simple_math.dll")?);
            unsafe {
                func = Some(lib.as_ref().unwrap().get(b"double\0")?);
            }
        }
    }
}

fn should_reload(last_time: &mut SystemTime) -> bool {
    if let Ok(modified) = metadata("..\\simple_math\\target\\release\\simple_math.dll").and_then(|m| m.modified()) {
        if modified > *last_time {
            *last_time = modified;
            return true;
        }
    }
    false
}
