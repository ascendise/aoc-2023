use rstest::*;
use crate::game::*;

#[rstest]
#[case(
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
    Game {
        id: 1,
        sets: vec![
            CubeSet::new(4, 0, 3),
            CubeSet::new(1, 2, 6),
            CubeSet::new(0, 2, 0)
    ] }
)]
#[case(
    "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
    Game {
        id: 2,
        sets: vec![
            CubeSet::new(0, 2, 1),
            CubeSet::new(1, 3, 4),
            CubeSet::new(0, 1, 1)
        ]
    }
)]
#[case(
    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    Game {
        id: 4,
        sets: vec![
            CubeSet::new(3, 1, 6),
            CubeSet::new(6, 3, 0),
            CubeSet::new(14, 3, 15)
        ]
    }
)]
fn from_should_construct_game_from_line(#[case] input: &str, #[case] expected: Game) {
    //Arrange
    //Act
    let game = Game::from(input);
    //Assert
    assert_eq!(expected, game);
}

#[rstest]
#[case(
    Game {
        id: 1,
        sets: vec![
            CubeSet::new(1, 0, 1),
            CubeSet::new(1, 0, 0),
            CubeSet::new(0, 1, 1)
        ]
    },
    CubeSet::new(1, 1, 1)
)]
#[case(
    Game {
        id: 1,
        sets: vec![
            CubeSet::new(4, 0, 3),
            CubeSet::new(6, 3, 1),
            CubeSet::new(1, 2, 2)
        ]
    },
    CubeSet::new(6, 3, 3)
)]
#[case(
    Game {
        id: 1,
        sets: vec![
            CubeSet::new(999, 999, 999),
            CubeSet::new(999, 999, 999),
            CubeSet::new(0, 0, 0)
        ]
    },
    CubeSet::new(1000, 1000, 1000)
)]
fn is_content_should_return_true_if_passed_set_contains_more_balls_than_were_used_in_game(#[case] game: Game, #[case] content: CubeSet) {
    //Arrange
    //Act
    let res = game.is_content(&content);
    //Assert
    assert_eq!(true, res);
}

#[rstest]
#[case(
    Game {
        id: 1,
        sets: vec![
            CubeSet::new(1, 0, 1),
            CubeSet::new(1, 0, 0),
            CubeSet::new(0, 1, 1)
        ]
    },
    CubeSet::new(0, 1, 0)
)]
#[case(
    Game {
        id: 1,
        sets: vec![
            CubeSet::new(4, 0, 3),
            CubeSet::new(6, 3, 1),
            CubeSet::new(1, 2, 2)
        ]
    },
    CubeSet::new(2, 3, 6)
)]
#[case(
    Game {
        id: 1,
        sets: vec![
            CubeSet::new(999, 999, 999),
            CubeSet::new(999, 999, 999),
            CubeSet::new(0, 0, 0)
        ]
    },
    CubeSet::new(10000, 1000000, 2)
)]
fn is_content_should_return_false_if_passed_set_contains_less_balls_than_were_used_in_game(#[case] game: Game, #[case] content: CubeSet) {
    //Arrange
    //Act
    let res = game.is_content(&content);
    //Assert
    assert_eq!(false, res);
}

#[rstest]
#[case(
    Game {
        id: 1,
        sets: vec![
            CubeSet::new(1, 0, 1),
            CubeSet::new(1, 0, 0),
            CubeSet::new(0, 1, 1)
        ]
    },
    CubeSet::new(1, 1, 1)
)]
#[case(
    Game {
        id: 1,
        sets: vec![
            CubeSet::new(4, 0, 3),
            CubeSet::new(6, 3, 1),
            CubeSet::new(1, 2, 2)
        ]
    },
    CubeSet::new(6, 3, 3)
)]
#[case(
    Game {
        id: 1,
        sets: vec![
            CubeSet::new(999, 999, 999),
            CubeSet::new(999, 999, 999),
            CubeSet::new(0, 0, 0)
        ]
    },
    CubeSet::new(999, 999, 999)
)]
fn get_minimum_content_should_return_the_least_amount_of_balls_in_the_game(#[case] game: Game, #[case] expected_content: CubeSet) {
    //Arrange
    //Act
    let content = game.get_minimum_content();
    //Assert
    assert_eq!(expected_content, content);
}