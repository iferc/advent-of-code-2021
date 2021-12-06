#[test]
pub fn example() {
    let input = include_str!("../../data/example.txt");
    let output = 26_984_457_539;

    let result = super::challenge(input, 256).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn challenge() {
    let input = include_str!("../../data/challenge.txt");
    let output = 1_609_314_870_967;

    let result = super::challenge(input, 256).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}
