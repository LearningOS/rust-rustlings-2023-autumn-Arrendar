// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.

struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
// unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
//     // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
//     // simply reconstruct the box from that pointer.
//     let mut ret: Box<Foo> = unsafe {  Box::new(Foo { a: 0, b: None })};
//     ptr::write(&mut ret as *mut _ as *mut _ as *mut u128, ptr);  
//     ret  
//     // todo!("The rest of the code goes here")
// }

unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.
    let mut ret: Box<Foo> = Box::from_raw(ptr);
    let b = String::from("hello");
    ret.b = Some(b);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));
    }
}
