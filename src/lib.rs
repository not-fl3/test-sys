extern "C" {
    pub fn b();
}

#[test]
fn test_a() {
    unsafe {
        b();
    }
}
