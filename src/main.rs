mod test;

fn main() {
    unsafe {
        let mut a = test::foo::new(::std::ptr::null_mut());
        //a.hihihi();
    }
}
