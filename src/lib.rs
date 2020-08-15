pub mod oapi_consts;

#[cxx::bridge]
mod ffi {
    struct MFDButtonMenu {
        line1: String,
        line2: String,
        selchar: u8
    }
    extern "C" {
        include!("src/wrapper.h");

        type HINSTANCE;
        type OapiSketchpad;
        type Font;

        // OapiSketchpad class methods
        fn SetFont(self: &OapiSketchpad, font: &Font);
        fn SetTextColor(self: &mut OapiSketchpad, color: u32) -> u32;
        // fn SetTextAlign(self: &mut OapiSketchpad, tah: TAlign_horizontal, tav: TAlign_vertical);
        fn Text(self: &mut OapiSketchpad, x: i32, y: i32, str: &str) -> bool;
        fn Rectangle(self: &mut OapiSketchpad, x0: i32, y0: i32, x1: i32, y1: i32);

        fn debugLog(line: &str);
        fn InitModuleSpec(name: &str, key: u32);
        fn ExitModuleSpec();
    }
    extern "Rust" {
        type RustMFD;
        fn Title(self: &RustMFD) -> &str;
        fn Update(self: &RustMFD, sketchpad: &mut OapiSketchpad, W: u32, H: u32);
        fn ButtonLabel(self: &RustMFD, btn: i32) -> &str;
        fn ConsumeButton(self: &mut RustMFD, btn: i32, event: i32);
        fn ButtonMenu(self: &RustMFD, btn: u32) -> MFDButtonMenu;
    }
}

#[no_mangle]
extern "C" fn create_rust_mfd() -> *mut RustMFD {
    Box::into_raw(Box::new(RustMFD{counter: 0}))
}
pub struct RustMFD {
    counter: i32
}
impl RustMFD {
    pub fn Title(&self) -> &str
    {
        "Counter MFD written in Rust"
    }
    pub fn ButtonMenu(&self, btn: u32) -> ffi::MFDButtonMenu
    {
        match btn
        {
            0 => ffi::MFDButtonMenu{
                line1:"Increment counter".into(),
                line2: "".into(),
                selchar: 0x55, // 'U'
                },
            1 => ffi::MFDButtonMenu{
                line1:"Decrement counter".into(),
                line2: "".into(),
                selchar: 0x44, // 'D'
                },
            _ => ffi::MFDButtonMenu{
                line1:"".into(),
                line2: "".into(),
                selchar: 0
            }
        }
    }
    pub fn ButtonLabel(&self, btn: i32) -> &str
    {
        match btn 
        {
            0 => "UP\0",
            1 => "DN\0",
            _ => "\0"
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
        sketchpad.Text(W/4+5, H/4+5, "Hello from RustMFD!!");
        sketchpad.Text(W/4+5, H/4+5+15, & format!("Counter: {}", self.counter));
        sketchpad.Rectangle (W/4, H/4, (3*W)/4, (3*H)/4);
    }
    pub fn ConsumeButton(&mut self, bt: i32, event: i32)
    {
        ffi::debugLog(& format!("Pressed button {}", bt));
        if bt == 0 {
            self.counter += 1;
        }else if bt == 1{
            self.counter -= 1;
        }
    }
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
