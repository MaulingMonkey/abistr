use abistr::*;

fn main() {
    let nn_utf8  = cstr8!("UTF-8");
    let nn_utf16 = cstr8!("UTF-16");
    let nn_utf32 = cstr8!("UTF-32");

    let ptr_utf8  = CStrPtr::from(nn_utf8 );
    let ptr_utf16 = CStrPtr::from(nn_utf16);
    let ptr_utf32 = CStrPtr::from(nn_utf32);

    dbg!(("..."));
}
