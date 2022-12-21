#[test]
fn ascii() {
	let number: f32 = 123.056;
	let utf: _ = format!("ABCabc: {}", number).into_bytes();
	let ascii: _ = b"ABCabc: 123.056";

	assert_eq!(ascii, utf.as_slice());
}