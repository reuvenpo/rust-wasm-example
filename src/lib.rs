
// Exported functions must be marked as `#[no_mangle] pub extern "C" fn`
#[no_mangle]
pub extern "C" fn exported_function(x: i32) -> i32 {
    x + unsafe { imported_function(x) }
}

extern "C" {
    fn imported_function(x: i32) -> i32;
    // More imports here...
}
