use super::*;
use rstest::*;

#[rstest]
#[case(
    vec![
        "...",
        ".4.",
        "#.."
    ],
    Engine {
        matrix: vec![
            vec!['.','.','.'],
            vec!['.','4','.'],
            vec!['#','.','.'],
        ]
    }
)]
fn from_should_return_correct_engine(#[case] input: Vec<&str>, #[case] expected: Engine) {
    //Arrange
    //Act
    let engine = Engine::from(input);
    //Assert
    assert_eq!(expected, engine);
}