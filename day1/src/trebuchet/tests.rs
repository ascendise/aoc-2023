use rstest::*;

#[rstest]
#[case("1abc2", 12)]
#[case("1ab23c4", 14)]
#[case("ab1c2d", 12)]
#[case("9182", 92)]
fn recover_should_return_correct_value(#[case] input: &str, #[case] expected: u32) {
    //Arrange
    //Act
    let result = super::recover(input);
    //Assert
    assert_eq!(expected, result);
}

#[rstest]
#[case("1abc", 11)]
#[case("ab9c", 99)]
#[case("abc2", 22)]
fn recover_should_handle_strings_with_single_digit(#[case] input: &str, #[case] expected: u32) {
    //Arrange
    //Act
    let result = super::recover(input);
    //Assert
    assert_eq!(expected, result);
}

#[rstest]
#[case("two1nine", 29)]
#[case("oneight", 18)]
#[case("abcone2threexyz", 13)]
fn recover_should_handle_spelled_out_digits(#[case] input: &str, #[case] expected: u32) {
    //Arrange
    //Act
    let result = super::recover(input);
    //Assert
    assert_eq!(expected, result);
}

#[rstest]
#[case("slconeightfoureight557m38", 18)]
#[case("gdkzeightjbmgffzqrseight862mstxshpg", 82)]
#[case("gnmkdm7sevenseven3four7fhrhppmtkpzvtlfqoneighth", 78)]
#[case("fournr8ltltldqsmcd5threetwothree", 43)]
fn recover_should_work_with_puzzle_input(#[case] input: &str, #[case] expected: u32) {
    //Arrange
    //Act
    let result = super::recover(input);
    //Assert
    assert_eq!(expected, result);
}