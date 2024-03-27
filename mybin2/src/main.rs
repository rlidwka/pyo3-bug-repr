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
}
