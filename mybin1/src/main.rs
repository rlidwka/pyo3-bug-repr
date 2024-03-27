extern "C" {
    fn pytest();
}

fn main() {
    unsafe { pytest(); }
}
