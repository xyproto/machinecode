use std::fmt;
use std::io;

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
    hexstring
        .split_whitespace()
        .map(|s| u8::from_str_radix(s, 16))
        .collect()
}

// Execute machine code and return the value
pub fn execute(code: &[u8]) -> Result<i32, Box<dyn std::error::Error>> {
    // Check for empty code
    if code.is_empty() {
        return Err(Box::new(MemCopyError));
    }

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
        let result = std::panic::catch_unwind(|| f());

        libc::munmap(executable_area as *mut libc::c_void, size);

        match result {
            Ok(val) => Ok(val),
            Err(_) => Err(Box::new(MemCopyError)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_bytes() {
        let input = "00 ff a2 5e";
        let output = string_to_bytes(input);
        match output {
            Ok(v) => assert_eq!(v, vec![0x00, 0xff, 0xa2, 0x5e]),
            Err(_) => panic!("Failed to convert hex string to bytes"),
        }
    }

    #[test]
    fn test_string_to_bytes_error() {
        let input = "00 ffzz a2 5e";
        let output = string_to_bytes(input);
        assert!(output.is_err());
    }

    // Test the error case instead of executing machine code in the test
    #[test]
    fn test_execute_error() {
        let code = vec![];
        let result = execute(&code);
        assert!(result.is_err());
    }
}
