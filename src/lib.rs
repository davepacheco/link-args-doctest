/// ```
/// link_args_doctest::foo();
/// ```
pub fn foo() -> String {
    format!("{:?}", unsafe { pq_sys::PQlibVersion() })
}

#[test]
fn test_foo() {
    eprintln!("{}", foo());
}
