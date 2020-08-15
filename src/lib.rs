pub mod oapi_consts;

// #[cxx::bridge(namespace = oapi)]
// mod oapi {
//     extern "C"
//     {
//         include!("Orbitersdk.h");
//         type Sketchpad;
        
//         // fn SetTextColor(self:&mut Sketchpad, color: DWORD) -> u32;
//     }
// }
// #[cxx::bridge(namespace = oapi::Sketchpad)]
// mod oapi_sketchpad {
//     extern "C"
//     {
//         include!("Orbitersdk.h");
//         type TAlign_horizontal;
//     }
// }

struct MFDImpl {

}

impl MFDImpl {
    pub fn Update(&self, sketchpad: &mut ffi::OapiSketchpad, W: u32, H: u32)
    {
        let H = H as i32;
        let W = W as i32;
        // let a: oapi_sketchpad::TAlign_horizontal;
        sketchpad.Text(W/2, H/2, "Hello from Rust!!");
    }
}
#[cxx::bridge]
mod ffi {
    struct WrappedMFD {
        mfd: UniquePtr<MFDTemplate>
    }
    extern "C" {
        include!("src/wrapper.h");
        include!("src/MFDTemplate.h");

        type HINSTANCE;
        type OapiSketchpad;
        type Font;
        type MFDTemplate;
        
        // OapiSketchpad class methods
        fn SetFont(self: &OapiSketchpad, font: UniquePtr<Font>);
        fn SetTextColor(self: &mut OapiSketchpad, color: u32) -> u32;
        // fn SetTextAlign(self: &mut OapiSketchpad, tah: TAlign_horizontal, tav: TAlign_vertical);
        fn Text(self: &mut OapiSketchpad, x: i32, y: i32, str: &str) -> bool;
        fn Rectangle(self: &mut OapiSketchpad, x0: i32, y0: i32, x1: i32, y1: i32);

        fn debugLog(line: &str);
        fn InitModuleSpec(name: &str, key: u32);
        fn ExitModuleSpec();
    }
    extern "Rust" {
        fn UpdateMFD(sketchpad: &mut OapiSketchpad, W: u32, H: u32);
    }
}

fn UpdateMFD(sketchpad: &mut ffi::OapiSketchpad, W: u32, H: u32)
{
    let H = H as i32;
    let W = W as i32;
    // let a: oapi_sketchpad::TAlign_horizontal;
    sketchpad.Text(W/2, H/2, "Hello from Rust!!");
}

#[no_mangle]
pub extern fn InitModule (_h_dll: ffi::HINSTANCE)
{
    ffi::debugLog("Initializing RustMFD...");
    ffi::InitModuleSpec("RustMFD", oapi_consts::OAPI_KEY_T);
}
#[no_mangle]
pub extern fn ExitModule (_h_dll: ffi::HINSTANCE)
{
	ffi::ExitModuleSpec();
}
