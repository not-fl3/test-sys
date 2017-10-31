mod test;

#[test()]
fn create_delete_test() {
    unsafe {
        let mut a = test::B::new();
        test::B_B_destructor(&mut a as *mut _);
    }
}

fn main() {
}
