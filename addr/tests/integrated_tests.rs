use addr;

#[test]
#[should_panic(expected = "Guess value must be less than or equal to 100")]
fn greater_than_100() {
    addr::Guess::new(200);
}
