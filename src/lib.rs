use std::io;
use std::fmt;

// Error for copying machine code to executable memory
#[derive(Debug, Clone)]
pub struct MemCopyError;

impl fmt::Display for MemCopyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not copy code over to the executable memory area")
    }
}

impl std::error::Error for MemCopyError {}

// Convert a space-separated hex string to a Vec<u8>
pub fn string_to_bytes(hexstring: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    hexstring.split_whitespace()
        .map(|s| u8::from_str_radix(s, 16))
        .collect()
}

// Execute machine code and return the value
pub fn execute(code: &[u8]) -> Result<i32, Box<dyn std::error::Error>> {
    let size = code.len();
    let executable_area = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            size,
            libc::PROT_WRITE | libc::PROT_EXEC,
            libc::MAP_ANONYMOUS | libc::MAP_PRIVATE,
            -1,
            0,
        ) as *mut u8
    };

    if executable_area.is_null() {
        return Err(Box::new(io::Error::last_os_error()));
    }

    unsafe {
        std::ptr::copy_nonoverlapping(code.as_ptr(), executable_area, size);
        let f: extern "C" fn() -> i32 = std::mem::transmute(executable_area);
        Ok(f())
    }
}
