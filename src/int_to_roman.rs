/**
12. Integer to Roman

Seven different symbols represent Roman numerals with the following values:
Symbol	Value
I	1
V	5
X	10
L	50
C	100
D	500
M	1000

Roman numerals are formed by appending the conversions of decimal place values from highest to lowest. Converting a decimal place value into a Roman numeral has the following rules:

If the value does not start with 4 or 9, select the symbol of the maximal value that can be subtracted from the input, append that symbol to the result, subtract its value, and convert the remainder to a Roman numeral.
If the value starts with 4 or 9 use the subtractive form representing one symbol subtracted from the following symbol, for example, 4 is 1 (I) less than 5 (V): IV and 9 is 1 (I) less than 10 (X): IX. Only the following subtractive forms are used: 4 (IV), 9 (IX), 40 (XL), 90 (XC), 400 (CD) and 900 (CM).
Only powers of 10 (I, X, C, M) can be appended consecutively at most 3 times to represent multiples of 10. You cannot append 5 (V), 50 (L), or 500 (D) multiple times. If you need to append a symbol 4 times use the subtractive form.

Given an integer, convert it to a Roman numeral.


Example 1:

Input: num = 3749

Output: "MMMDCCXLIX"

Explanation:

3000 = MMM as 1000 (M) + 1000 (M) + 1000 (M)
 700 = DCC as 500 (D) + 100 (C) + 100 (C)
  40 = XL as 10 (X) less of 50 (L)
   9 = IX as 1 (I) less of 10 (X)
Note: 49 is not 1 (I) less of 50 (L) because the conversion is based on decimal places

Example 2:

Input: num = 58

Output: "LVIII"

Explanation:

50 = L
 8 = VIII

Example 3:

Input: num = 1994

Output: "MCMXCIV"

Explanation:

1000 = M
 900 = CM
  90 = XC
   4 = IV


Constraints:

    1 <= num <= 3999

**/

fn convert_decimal(val: i32, unit: u8, five: u8, ten: u8) -> [Option<u8>; 4] {
    match val {
        0 => [None, None, None, None],
        1 => [Some(unit), None, None, None],
        2 => [Some(unit), Some(unit), None, None],
        3 => [Some(unit), Some(unit), Some(unit), None],
        4 => [Some(unit), Some(five), None, None],
        5 => [Some(five), None, None, None],
        6 => [Some(five), Some(unit), None, None],
        7 => [Some(five), Some(unit), Some(unit), None],
        8 => [Some(five), Some(unit), Some(unit), Some(unit)],
        9 => [Some(unit), Some(ten), None, None],
        _ => unreachable!(),
    }
}

#[allow(dead_code)]
pub fn int_to_roman(num: i32) -> String {
    let (I, V, X, L, C, D, M) = (
        "I".as_bytes()[0],
        "V".as_bytes()[0],
        "X".as_bytes()[0],
        "L".as_bytes()[0],
        "C".as_bytes()[0],
        "D".as_bytes()[0],
        "M".as_bytes()[0],
    );
    let mut result: [Option<u8>; 16] = [None; 16];
    let mut num = num;
    let val = num / 1000;
    let thousands: [Option<u8>; 4] = match val {
        0 => [None, None, None, None],
        1 => [Some(M), None, None, None],
        2 => [Some(M), Some(M), None, None],
        3 => [Some(M), Some(M), Some(M), None],
        _ => unreachable!(),
    };
    result[..4].copy_from_slice(&thousands);
    num -= val * 1000;
    let (result, _) = [(100, C, D, M), (10, X, L, C), (1, I, V, X)]
        .into_iter()
        .fold(
            (result, num),
            |(mut result, num), (magnitude, unit, five, ten)| {
                let decimal = num / magnitude;
                let roman = convert_decimal(decimal, unit, five, ten);
                let (start, end) = match magnitude {
                    1 => (12, 16),
                    10 => (8, 12),
                    100 => (4, 8),
                    _ => unreachable!(),
                };
                result[start..end].copy_from_slice(&roman);
                let num = num - decimal * magnitude;
                (result, num)
            },
        );
    String::from_utf8(result[..16].iter().filter_map(|&c| c).collect::<Vec<_>>()).unwrap()
}

#[test]
fn test() {
    assert_eq!(int_to_roman(3), "III".to_string());
    assert_eq!(int_to_roman(4), "IV".to_string());
    assert_eq!(int_to_roman(9), "IX".to_string());
    assert_eq!(int_to_roman(58), "LVIII".to_string());
    assert_eq!(int_to_roman(1994), "MCMXCIV".to_string());
    assert_eq!(int_to_roman(1005), "MV".to_string());
}
