pub fn main() {
    println!("Hello World!");
}

#[test]
fn simple_math() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[should_panic(expected = "assertion `left == right` failed")]
fn newspeak_math() {
    assert_eq!(2 + 2, 5);
}
