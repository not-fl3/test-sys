mod test;

#[test()]
fn wtf_test() {
    unsafe {
        let mut a = test::B::new();
        test::B_B_destructor(&mut a as *mut _);
    }
}

fn main() {
}
