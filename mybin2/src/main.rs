use dlopen2::wrapper::{Container, WrapperApi};

#[derive(WrapperApi)]
struct Api {
    pytest: extern "C" fn(),
}

fn main() {
    let container: Container<Api> = unsafe {
        Container::load("./target/debug/libmylib.so")
    }.expect("could not open library");

    container.pytest();

    // same thing with libloading as well:
    /*unsafe {
        let lib = libloading::Library::new("./target/debug/libmylib.so").unwrap();
        let func: libloading::Symbol<extern "C" fn()> = lib.get(b"pytest").unwrap();
        func()
    }*/
}
