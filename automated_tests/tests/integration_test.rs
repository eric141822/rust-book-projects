use automated_tests;

#[test]
fn it_adds_two() {
    assert_eq!(4, automated_tests::add(2, 2));
}

#[test]
#[should_panic(expected = "less than or equal to 100")]
fn it_should_panic() {
    automated_tests::Guess::new(200);
}

#[test]
#[ignore]
fn ignored_test() {
    assert_eq!(2, automated_tests::add(1, 1));
}

