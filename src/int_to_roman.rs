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

const I: u8 = "I".as_bytes()[0];
const V: u8 = "V".as_bytes()[0];
const X: u8 = "X".as_bytes()[0];
const L: u8 = "L".as_bytes()[0];
const C: u8 = "C".as_bytes()[0];
const D: u8 = "D".as_bytes()[0];
const M: u8 = "M".as_bytes()[0];

fn convert_decimal(
    val: i32,
    unit: u8,
    five: u8,
    ten: u8,
    buffer: &mut [u8; 15],
    index: usize,
) -> usize {
    match val {
        0 => index,
        1 => {
            buffer[index] = unit;
            index + 1
        }
        2 => {
            buffer[index] = unit;
            buffer[index + 1] = unit;
            index + 2
        }
        3 => {
            buffer[index] = unit;
            buffer[index + 1] = unit;
            buffer[index + 2] = unit;
            index + 3
        }
        4 => {
            buffer[index] = unit;
            buffer[index + 1] = five;
            index + 2
        }
        5 => {
            buffer[index] = five;
            index + 1
        }
        6 => {
            buffer[index] = five;
            buffer[index + 1] = unit;
            index + 2
        }
        7 => {
            buffer[index] = five;
            buffer[index + 1] = unit;
            buffer[index + 2] = unit;
            index + 3
        }
        8 => {
            buffer[index] = five;
            buffer[index + 1] = unit;
            buffer[index + 2] = unit;
            buffer[index + 3] = unit;
            index + 4
        }
        9 => {
            buffer[index] = unit;
            buffer[index + 1] = ten;
            index + 2
        }
        _ => unreachable!(),
    }
}

#[allow(dead_code)]
pub fn int_to_roman(num: i32) -> String {
    let mut result = [0; 15];
    let mut index: usize = 0;
    let mut num = num;
    for (magnitude, unit, five, ten) in
        [(1000, M, M, M), (100, C, D, M), (10, X, L, C), (1, I, V, X)]
    {
        let decimal = num / magnitude;
        index = convert_decimal(decimal, unit, five, ten, &mut result, index);
        num -= decimal * magnitude;
    }
    unsafe { String::from_utf8_unchecked(result[..index].to_vec()) }
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
