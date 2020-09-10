#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");

impl Default for TidyBuffer {
    fn default() -> Self {
        return Self {
            allocator: std::ptr::null_mut(),
            bp: std::ptr::null_mut(),
            size: 0,
            allocated: 0,
            next: 0,
        };
    }
}
