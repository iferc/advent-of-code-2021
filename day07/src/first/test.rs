#[test]
pub fn example() {
    let input = include_str!("../../data/example.txt");
    let output = 37;

    let result = super::challenge(input).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn example_median() {
    let input = vec![16usize, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    let output = 2;

    let result = super::median(&input).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn challenge() {
    let input = include_str!("../../data/challenge.txt");
    let output = 353_800;

    let result = super::challenge(input).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}
