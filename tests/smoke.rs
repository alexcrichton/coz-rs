#[test]
fn smoke() {
    coz::progress!();
    coz::progress!("foo");
    coz::begin!("foo");
    coz::end!("foo");
}
