pub fn minesweeper() {
}
use std::str;

pub fn increase(input: u8) -> u8{
    return if input != 42 {1} else {0}
}

pub fn annotate_2(minefield: &[&str]) -> Vec<String> {
    if minefield.len() == 0 {return vec![]};
    if minefield[0].len() == 0 {return vec![format!("")]};
    let len = minefield[0].len();
    let mut temp = minefield.into_iter().map(|f| f.as_bytes()).collect::<Vec<&[u8]>>().concat();
    temp.iter_mut().for_each(|f| if f == &32 {*f = 48});
    temp.clone().iter_mut().enumerate().for_each(|(i, f)| {
        let left = i%len != 0;
        let right = i%len + 1 != len;
        let top = i >= len;
        let bot = i + len < temp.len();
        if f == &42{
            if left {temp[i-1] += increase(temp[i-1])};
            if right {temp[i+1] += increase(temp[i+1])};
            if top {temp[i-len] += increase(temp[i-len])};
            if bot {temp[i+len] += increase(temp[i+len])};
            if left && top {temp[i-len-1] += increase(temp[i-len-1])};
            if left && bot {temp[i+len-1] += increase(temp[i+len-1])};
            if right && top {temp[i-len+1] += increase(temp[i-len+1])};
            if right && bot {temp[i+len+1] += increase(temp[i+len+1])};
        }
    });
    temp.iter_mut().for_each(|f| if f == &48 {*f = 32});
    let temp = temp.into_iter().map(|f| str::from_utf8(&[f]).unwrap().to_string()).collect::<Vec<String>>();

    let mut res = Vec::<String>::new();
    let mut count = 0;
    while count < temp.len() {
        let ch = temp[count..count+len].join("");
        res.push(ch);
        count += len;
    }
    res
}

const NEIGBOURHOOD_OFFSETS: &'static[(i32, i32)] = &[
    (-1,-1), (-1, 0), (-1,1),
    (0, -1),          (0, 1),
    (1, -1), (1,  0), (1, 1)
];
pub fn annotate(field: &[&str]) -> Vec<String> {
    let height = field.len() as i32;
    (0..height).map(|y| {
        let width = field[y as usize].len() as i32;
        (0..width).map(|x| {
            if field[y as usize].as_bytes()[x as usize] != b'*'{
                match NEIGBOURHOOD_OFFSETS.iter()
                    .map(|&(ox, oy)| (x + ox, y + oy))
                    .filter(|&(x, y)| (0 <= x && x < width) && (0 <= y && y < height))
                    .filter(|&(x, y)| field[y as usize].as_bytes()[x as usize] == b'*')
                    .count() {
                        0 => ' ',
                        n => (n as u8 + '0' as u8) as char
                    }
            } else {
                '*'
            }
        }).collect()
    }).collect()
}
/*
        " ",    "1",
        "*",    "*",
        " ",    "2",
        "*",    "*",
        " ",    "1",
" 2*2 ",
"25*52",
"*****",
"25*52",
" 2*2 ",

"  *  ",
"  *  ",
"*****",
"  *  ",
"  *  "
 */

#[cfg(test)]
mod tests {
    use super::*;

    fn remove_annotations(board: &[&str]) -> Vec<String> {
        board.iter().map(|r| remove_annotations_in_row(r)).collect()
    }
    fn remove_annotations_in_row(row: &str) -> String {
        row.chars()
            .map(|ch| match ch {
                '*' => '*',
                _ => ' ',
            })
            .collect()
    }
    fn run_test(test_case: &[&str]) {
        let cleaned = remove_annotations(test_case);
        let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
        let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
        assert_eq!(expected, annotate(&cleaned_strs));
    }
    #[test]
    fn no_rows() {
        #[rustfmt::skip]
        run_test(&[
        ]);
    }
    #[test]
    fn no_columns() {
        #[rustfmt::skip]
        run_test(&[
            "",
        ]);
    }
    #[test]
    fn no_mines() {
        #[rustfmt::skip]
        run_test(&[
            "   ",
            "   ",
            "   ",
        ]);
    }
    #[test]
    fn board_with_only_mines() {
        #[rustfmt::skip]
        run_test(&[
            "***",
            "***",
            "***",
        ]);
    }
    #[test]
    fn mine_surrounded_by_spaces() {
        #[rustfmt::skip]
        run_test(&[
            "111",
            "1*1",
            "111",
        ]);
    }
    #[test]
    fn space_surrounded_by_mines() {
        #[rustfmt::skip]
        run_test(&[
            "***",
            "*8*",
            "***",
        ]);
    }
    #[test]
    fn horizontal_line() {
        #[rustfmt::skip]
        run_test(&[
            "1*2*1",
        ]);
    }
    #[test]
    fn horizontal_line_mines_at_edges() {
        #[rustfmt::skip]
        run_test(&[
            "*1 1*",
        ]);
    }
    #[test]
    fn vertical_line() {
        #[rustfmt::skip]
        run_test(&[
            "1",
            "*",
            "2",
            "*",
            "1",
        ]);
    }
    #[test]
    fn vertical_line_mines_at_edges() {
        #[rustfmt::skip]
        run_test(&[
            "*",
            "1",
            " ",
            "1",
            "*",
        ]);
    }
    #[test]
    fn cross() {
        #[rustfmt::skip]
        run_test(&[
            " 2*2 ",
            "25*52",
            "*****",
            "25*52",
            " 2*2 ",
        ]);
    }
    #[test]
    fn large_board() {
        #[rustfmt::skip]
        run_test(&[
            "1*22*1",
            "12*322",
            " 123*2",
            "112*4*",
            "1*22*2",
            "111111",
        ]);
    }
}
