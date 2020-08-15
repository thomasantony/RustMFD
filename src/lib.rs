pub mod oapi_consts;

#[cxx::bridge]
mod ffi {
    struct MFDBridge {
        mfd: Box<RustMFD>
    }
    extern "C" {
        include!("src/wrapper.h");

        type HINSTANCE;
        type OapiSketchpad;
        // type Font;

        // OapiSketchpad class methods
        // fn SetFont(self: &OapiSketchpad, font: UniquePtr<Font>);
        fn SetTextColor(self: &mut OapiSketchpad, color: u32) -> u32;
        // fn SetTextAlign(self: &mut OapiSketchpad, tah: TAlign_horizontal, tav: TAlign_vertical);
        fn Text(self: &mut OapiSketchpad, x: i32, y: i32, str: &str) -> bool;
        fn Rectangle(self: &mut OapiSketchpad, x0: i32, y0: i32, x1: i32, y1: i32);

        fn debugLog(line: &str);
        fn InitModuleSpec(name: &str, key: u32, mfd_bridge: Box<MFDBridge>);
        // fn InitModuleSpec(name: &str, key: u32);
        fn ExitModuleSpec();
    }
    extern "Rust" {
        type RustMFD;
        fn Update(self: &RustMFD, sketchpad: &mut OapiSketchpad, W: u32, H: u32);
        fn ButtonLabel(self: &RustMFD, btn: i32) -> &str;
        fn ConsumeButton(self: &RustMFD, bt: i32, event: i32);
    }
}

pub struct RustMFD {
}
impl RustMFD {
    pub fn ButtonLabel(&self, btn: i32) -> &str
    {
        if btn == 1
        {
            return "BTN1\0";
        }
        else if btn == 2
        {
            return "BTN2\0";
        } else{
            return "\0";
        }
    }
    pub fn Update(&self, sketchpad: &mut ffi::OapiSketchpad, W: u32, H: u32)
    {
        // skp->SetFont (font);
	    // skp->SetTextAlign (oapi::Sketchpad::CENTER, oapi::Sketchpad::BASELINE);	    
        let H = H as i32;
        let W = W as i32;
        // let a: oapi_sketchpad::TAlign_horizontal;
        sketchpad.SetTextColor (0x00FFFF);
        sketchpad.Text(W/2-20, H/2, "Hello from RustMFD!!");
        sketchpad.Rectangle (W/4, H/4, (3*W)/4, (3*H)/4);
    }
    pub fn ConsumeButton(&self, bt: i32, event: i32)
    {
        ffi::debugLog(& format!("Pressed button {}", bt));
    }
}

#[no_mangle]
pub extern fn InitModule (_h_dll: ffi::HINSTANCE)
{
    ffi::debugLog("Initializing RustMFD...");
    let mfd_bridge = ffi::MFDBridge{
        mfd: Box::new(RustMFD{})
    };
    ffi::InitModuleSpec("RustMFD", oapi_consts::OAPI_KEY_T, Box::new(mfd_bridge));
}
#[no_mangle]
pub extern fn ExitModule (_h_dll: ffi::HINSTANCE)
{
	ffi::ExitModuleSpec();
}
