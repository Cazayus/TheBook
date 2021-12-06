use the_book;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, the_book::add_two(2));
}
