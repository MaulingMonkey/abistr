use abistr::*;

fn main() {
    let nn_utf8  = cstr8!("UTF-8");
    let nn_utf16 = cstr16!("UTF-16");
    let nn_utf32 = cstr32!("UTF-32");

    let ptr_utf8  = CStrPtr::from(nn_utf8 );
    let ptr_utf16 = CStrPtr::from(nn_utf16);
    let ptr_utf32 = CStrPtr::from(nn_utf32);

    let nn_utf8ish  = CStrNonNull::<encoding::Utf8ish >::from_units_with_nul(nn_utf8.to_units_with_nul() ).unwrap();
    let nn_utf16ish = CStrNonNull::<encoding::Utf16ish>::from_units_with_nul(nn_utf16.to_units_with_nul()).unwrap();
    let nn_utf32ish = CStrNonNull::<encoding::Utf32ish>::from_units_with_nul(nn_utf32.to_units_with_nul()).unwrap();

    let ptr_utf8ish  = CStrPtr::from(nn_utf8ish );
    let ptr_utf16ish = CStrPtr::from(nn_utf16ish);
    let ptr_utf32ish = CStrPtr::from(nn_utf32ish);

    let nn_unk8  = CStrNonNull::<encoding::Unknown8 >::from_units_with_nul(nn_utf8.to_units_with_nul() ).unwrap();
    let nn_unk16 = CStrNonNull::<encoding::Unknown16>::from_units_with_nul(nn_utf16.to_units_with_nul()).unwrap();
    let nn_unk32 = CStrNonNull::<encoding::Unknown32>::from_units_with_nul(nn_utf32.to_units_with_nul()).unwrap();

    let ptr_unk8  = CStrPtr::from(nn_unk8 );
    let ptr_unk16 = CStrPtr::from(nn_unk16);
    let ptr_unk32 = CStrPtr::from(nn_unk32);

    dbg!((
        nn_utf8, nn_utf16, nn_utf32,
        ptr_utf8, ptr_utf16, ptr_utf32,
    ));
    dbg!((
        nn_utf8ish, nn_utf16ish, nn_utf32ish,
        ptr_utf8ish, ptr_utf16ish, ptr_utf32ish,
    ));
    dbg!((
        nn_unk8, nn_unk16, nn_unk32,
        ptr_unk8, ptr_unk16, ptr_unk32,
    ));
}
