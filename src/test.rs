/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bar {
    pub someWeirdData: [::std::os::raw::c_int; 666usize],
}
#[test]
fn bindgen_test_layout_bar() {
    assert_eq!(
        ::std::mem::size_of::<bar>(),
        2664usize,
        concat!("Size of: ", stringify!(bar))
    );
    assert_eq!(
        ::std::mem::align_of::<bar>(),
        4usize,
        concat!("Alignment of ", stringify!(bar))
    );
    assert_eq!(
        unsafe { &(*(0 as *const bar)).someWeirdData as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(bar),
            "::",
            stringify!(someWeirdData)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct foo {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        1usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        1usize,
        concat!("Alignment of ", stringify!(foo))
    );
}
extern "thiscall" {
    #[link_name = "\u{1}__ZN3foo6hihihiEv"]
    pub fn foo_hihihi(this: *mut foo);
}
extern "thiscall" {
    #[link_name = "\u{1}__ZN3fooC1Ev"]
    pub fn foo_foo(this: *mut foo);
}
extern "thiscall" {
    #[link_name = "\u{1}__ZN3fooC1EP3bar"]
    pub fn foo_foo1(this: *mut foo, bar: *mut bar);
}
impl foo {
    #[inline]
    pub unsafe fn hihihi(&mut self) {
        foo_hihihi(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        foo_foo(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(bar: *mut bar) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        foo_foo1(&mut __bindgen_tmp, bar);
        __bindgen_tmp
    }
}
