fn main() {
    #[cfg(windows)] windows::main();
}

#[cfg(windows)] mod windows {
    #![allow(dead_code)]

    use abistr::*;
    use abistr::encoding::windows::*;
    use core::convert::TryInto;
    use core::ffi::*;
    use core::ptr::*;



    pub fn main() {
        dbg!(System.to_ansi_code_page());
        dbg!(CurrentThread.to_ansi_code_page());
        dbg!(ConsoleInput.to_ansi_code_page());
        dbg!(ConsoleOutput.to_ansi_code_page());

        if let Ok(conout) = get_std_handle(STD_OUTPUT_HANDLE) {
            let _ = write_console_a(conout, cstr8!("Hello, world!").to_units(), ());
        }
        //let _ = message_box_a((), cstr8!("te\\xt"), cstr8!("capt\\ion"), 0);
    }



    /// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxa)\]
    /// MessageBoxA
    fn message_box_a(_hwnd: (), text: impl AsOptCStr<System>, caption: impl AsOptCStr<System>, type_: c_uint) -> Result<c_int, ()> {
        let hwnd    = core::ptr::null();
        let text    = text.as_opt_cstr();
        let caption = caption.as_opt_cstr();

        #[link(name = "user32")] extern "system" { fn MessageBoxA(hwnd: *const (), text: *const c_char, caption: *const c_char, type_: c_uint) -> c_int; }
        let r = unsafe { MessageBoxA(hwnd, text.cast(), caption.cast(), type_) };
        if r == 0 { return Err(()); }
        Ok(r)
    }



    #[derive(Clone, Copy, Debug)] struct ConsoleHandle(NonNull<()>);

    /// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/console/getstdhandle)\]
    /// GetStdHandle
    fn get_std_handle(std_handle: i32) -> Result<ConsoleHandle, ()> {
        #[link(name = "user32")] extern "system" { fn GetStdHandle(std_handle: u32) -> *mut (); }
        Ok(ConsoleHandle(NonNull::new(unsafe { GetStdHandle(std_handle as _) }).ok_or(())?))
    }

    /// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/console/writeconsole)\]
    /// WriteConsoleA
    fn write_console_a(console_output: ConsoleHandle, text: impl AsRef<[u8]>, _reserved: ()) -> Result<(), ()> {
        // TODO: apply encoding::windows::ConsoleOutput to type signature
        let text = text.as_ref();
        let len32 : u32 = text.len().try_into().map_err(|_| {})?;
        #[link(name = "user32")] extern "system" { fn WriteConsoleA(handle: *const (), buffer: *const c_char, chars_to_write: c_uint, written: *mut c_uint, reserved: *const ()) -> c_uint; }
        let ok = unsafe { WriteConsoleA(console_output.0.as_ptr(), text.as_ptr().cast(), len32, null_mut(), null()) };
        if ok == 0 { Err(()) } else { Ok(()) }
    }

    const STD_INPUT_HANDLE  : i32 = -10;
    const STD_OUTPUT_HANDLE : i32 = -11;
    const STD_ERROR_HANDLE  : i32 = -12;
}
