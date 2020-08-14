#[cxx::bridge]
mod ffi {
    extern "C" {
        include!("src/wrapper.h");
        include!("src/MFDTemplate.h");

        type HINSTANCE;
        fn debugLog(line: &str);
        fn InitModuleSpec();
        fn ExitModuleSpec();
    }
}

#[no_mangle]
pub extern fn InitModule (_h_dll: ffi::HINSTANCE)
{
    ffi::debugLog("Initializing RustMFD...");
    ffi::InitModuleSpec();
}
#[no_mangle]
pub extern fn ExitModule (_h_dll: ffi::HINSTANCE)
{
	ffi::ExitModuleSpec();
}

#[no_mangle]
pub extern fn lib_test() {
    println!("Hello from the library!");
}