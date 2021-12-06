#[test]
pub fn example_at_256_days() {
    let input = include_str!("../../data/example.txt");
    let output = 26984457539;

    let result = super::challenge(input, 256).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn example_at_80_days() {
    let input = include_str!("../../data/example.txt");
    let output = 5934;

    let result = super::challenge(input, 80).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn example_at_18_days() {
    let input = include_str!("../../data/example.txt");
    let output = 26;

    let result = super::challenge(input, 18).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn example_at_17_days() {
    let input = include_str!("../../data/example.txt");
    let output = 22;

    let result = super::challenge(input, 17).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn example_at_16_days() {
    let input = include_str!("../../data/example.txt");
    let output = 21;

    let result = super::challenge(input, 16).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn example_at_15_days() {
    let input = include_str!("../../data/example.txt");
    let output = 20;

    let result = super::challenge(input, 15).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn example_at_14_days() {
    let input = include_str!("../../data/example.txt");
    let output = 20;

    let result = super::challenge(input, 14).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn example_at_13_days() {
    let input = include_str!("../../data/example.txt");
    let output = 19;

    let result = super::challenge(input, 13).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn example_at_12_days() {
    let input = include_str!("../../data/example.txt");
    let output = 17;

    let result = super::challenge(input, 12).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn example_at_11_days() {
    let input = include_str!("../../data/example.txt");
    let output = 15;

    let result = super::challenge(input, 11).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn example_at_10_days() {
    let input = include_str!("../../data/example.txt");
    let output = 12;

    let result = super::challenge(input, 10).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn example_at_9_days() {
    let input = include_str!("../../data/example.txt");
    let output = 11;

    let result = super::challenge(input, 9).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn example_at_8_days() {
    let input = include_str!("../../data/example.txt");
    let output = 10;

    let result = super::challenge(input, 8).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn example_at_7_days() {
    let input = include_str!("../../data/example.txt");
    let output = 10;

    let result = super::challenge(input, 7).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn example_at_6_days() {
    let input = include_str!("../../data/example.txt");
    let output = 10;

    let result = super::challenge(input, 6).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn example_at_5_days() {
    let input = include_str!("../../data/example.txt");
    let output = 10;

    let result = super::challenge(input, 5).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn example_at_4_days() {
    let input = include_str!("../../data/example.txt");
    let output = 9;

    let result = super::challenge(input, 4).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn example_at_3_days() {
    let input = include_str!("../../data/example.txt");
    let output = 7;

    let result = super::challenge(input, 3).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn example_at_2_days() {
    let input = include_str!("../../data/example.txt");
    let output = 6;

    let result = super::challenge(input, 2).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn example_at_1_days() {
    let input = include_str!("../../data/example.txt");
    let output = 5;

    let result = super::challenge(input, 1).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn example_at_0_days() {
    let input = include_str!("../../data/example.txt");
    let output = 5;

    let result = super::challenge(input, 0).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn challenge() {
    let input = include_str!("../../data/challenge.txt");
    let output = 353274;

    let result = super::challenge(input, 80).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}

#[test]
pub fn challenge_part_two() {
    let input = include_str!("../../data/challenge.txt");
    let output = 1609314870967;

    let result = super::challenge(input, 256).expect("challenge threw an error");

    assert_eq!(result, output, "challenge produced an unexpected result");
}
