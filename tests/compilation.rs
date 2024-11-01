#[test]
fn compilation() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compilation/not_a_struct.rs");
    t.compile_fail("tests/compilation/no_named_fields.rs");
    t.compile_fail("tests/compilation/no_variable.rs");
    t.compile_fail("tests/compilation/no_function_auto_compute.rs");
}