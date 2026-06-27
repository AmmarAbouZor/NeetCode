// Same algorithm as multiplication by hand, with a result vector:
// * We put results in a vector here and do the carry work in separate step.
// * Product of num1[i] and num2[j] contributes to res[i + j + 1].
// * Results vector length is the sum of the two numbers lengths.
// * Results vector may have leading zeros that needs to be skipped.

// Time: O(n1 * n2) where n1 and n2 are the length of num1 and num2
// Space: O(n1 + n2)

pub fn multiply(num1: String, num2: String) -> String {
    //Edge cases
    if num1 == "0" || num2 == "0" {
        return "0".to_string();
    }

    let num1 = num1.as_bytes();
    let num2 = num2.as_bytes();

    let len1 = num1.len();
    let len2 = num2.len();

    let mut res = vec![0; len1 + len2];

    // Fill results with digits multiplications.
    for i in (0..len1).rev() {
        for j in (0..len2).rev() {
            let digit1 = (num1[i] - b'0') as i32;
            let digit2 = (num2[j] - b'0') as i32;

            res[i + j + 1] += digit1 * digit2;
        }
    }

    // Apply carry adjustments.
    for i in (1..res.len()).rev() {
        let carry = res[i] / 10;
        res[i] %= 10;
        res[i - 1] += carry;
    }

    // Cleanup leading zeros.
    let mut start = 0;
    while start < res.len() && res[start] == 0 {
        start += 1;
    }

    res[start..]
        .iter()
        .map(|&d| (d as u8 + b'0') as char)
        .collect()
}
