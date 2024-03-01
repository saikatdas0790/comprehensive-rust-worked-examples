#![allow(dead_code)]
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    matrix
        .iter()
        .enumerate()
        .fold([[0; 3]; 3], |mut acc, (i, row)| {
            row.iter().enumerate().for_each(|(j, &el)| {
                acc[j][i] = el;
            });
            acc
        })
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}
