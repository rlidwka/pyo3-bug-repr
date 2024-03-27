#[no_mangle]
pub extern "C" fn pytest() {
    pyo3::prepare_freethreaded_python();

    let source = r#"
import numpy
def hello():
    print(numpy.version.version)
hello()
"#;

    pyo3::Python::with_gil(|py| {
        match pyo3::types::PyModule::from_code_bound(py, source, "<script>", "<script>") {
            Ok(_) => {
                println!("ok");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    });
}
