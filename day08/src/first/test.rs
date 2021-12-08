#[test]
pub fn example() {
    let input = include_str!("../../data/example.txt");
    let output = 26;

    let result = super::challenge(input).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn challenge() {
    let input = include_str!("../../data/challenge.txt");
    let output = 318;

    let result = super::challenge(input).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}
