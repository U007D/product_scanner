use super::*;

#[test]
fn args_can_be_tested_by_constructing_from_an_iterator_of_values() {
    // setup
    let app = "test 0 3";
    let rows = "0";
    let cols = "3";
    let args = vec![app, rows, cols];

    // given a `StructOpt::from_iter()` constructor
    let sut = Args::from_iter_safe::<&Vec<&str>>;

    // when it is invoked
    let result = sut(&args);

    // then it should return an error
    assert_eq!(true, result.is_err());
}
