mod test;

fn main() {
    unsafe {
        let mut a = test::foo::new();
    }
    println!("foo::new() OK");
    unsafe {
        let mut a = test::foo::new1(::std::ptr::null_mut());
    }
    println!("foo::new(bar *) OK");
}
