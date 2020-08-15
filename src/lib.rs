pub mod oapi_consts;

#[cxx::bridge]
mod ffi {
    enum TAlignHorizontal {
        Left,
        Center,
        Right
    }
    enum TAlignVertical {
        Top,
        Baseline,
        Bottom
    }

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
        fn SetTextAlign(self: &mut OapiSketchpad, tah: TAlignHorizontal, tav: TAlignVertical);
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
    pub fn Update(&self, sketchpad: &mut ffi::OapiSketchpad, w: u32, h: u32)
    {
        // skp->SetFont (font);
	    
        let h = h as i32;
        let w = w as i32;
        sketchpad.SetTextAlign (ffi::TAlignHorizontal::Center, ffi::TAlignVertical::Baseline);
        sketchpad.SetTextColor (0x00FFFF);
        sketchpad.Text(w/2, h/2, "Hello from RustMFD!!");
        sketchpad.Text(w/2, h/2+20, & format!("Counter: {}", self.counter));
        sketchpad.Rectangle (w/6, h/6, (5*w)/6, (5*h)/6);
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
