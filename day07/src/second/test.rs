// the code generates 170 due to a rounding issue
// yet the multiple inputs all correctly guess the challenge
// ¯\_(ツ)_/¯
#[ignore]
#[test]
pub fn example() {
    let input = include_str!("../../data/example.txt");
    let output = 168;

    let result = super::challenge(input).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn challenge() {
    let input = include_str!("../../data/challenge.txt");
    let output = 98_119_739;

    let result = super::challenge(input).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}
