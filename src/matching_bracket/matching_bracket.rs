pub fn matching_bracket() {}
pub fn brackets_are_balanced_2(string: &str) -> bool {
    let string = string
        .chars()
        .filter(|character| ['[', ']', '{', '}', '(', ')'].contains(character));

    let len = string.clone().count();
    if len % 2 != 0 {
        return false;
    } else if len == 0 {
        return true;
    }

    let mut check = vec![];
    for i in string {
        match i {
            '}' => {
                if let Some(val) = check.last() {
                    if *val == '{' {
                        check.pop();
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            ']' => {
                if let Some(val) = check.last() {
                    if *val == '[' {
                        check.pop();
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            ')' => {
                if let Some(val) = check.last() {
                    if *val == '(' {
                        check.pop();
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => check.push(i),
        }
    }
    check.len() == 0
}

pub fn brackets_are_balanced_3(string: &str) -> bool {
    let mut check = Vec::<char>::new();

    for i in string.chars() {
        if ['[', ']', '{', '}', '(', ')'].contains(&i) {
            match i {
                '}' => {
                    if let Some(val) = check.last() {
                        if *val == '{' {
                            check.pop();
                        } else {
                            return false
                        }
                    } else {
                        return false
                    }
                }
                ']' => {
                    if let Some(val) = check.last() {
                        if *val == '[' {
                            check.pop();
                        } else {
                            return false
                        }
                    } else {
                        return false
                    }
                }
                ')' => {
                    if let Some(val) = check.last() {
                        if *val == '(' {
                            check.pop();
                        } else {
                            return false
                        }
                    } else {
                        return false
                    }
                }
                _ => check.push(i),
            }
        }
    };

    check.len() == 0
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets: Vec<char> = Vec::new();
    for c in string.chars() {
        match c {
            '(' | '{' | '[' => brackets.push(c),
            ')' => if brackets.pop() != Some('(') {return false},
            ']' => if brackets.pop() != Some('[') {return false},
            '}' => if brackets.pop() != Some('{') {return false},
            _ => ()
        }
    }
    brackets.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paired_square_brackets() {
        assert!(brackets_are_balanced("[]"));
    }
    #[test]
    fn empty_string() {
        assert!(brackets_are_balanced(""));
    }
    #[test]
    fn unpaired_brackets() {
        assert!(!brackets_are_balanced("[["));
    }
    #[test]
    fn wrong_ordered_brackets() {
        assert!(!brackets_are_balanced("}{"));
    }
    #[test]
    fn wrong_closing_bracket() {
        assert!(!brackets_are_balanced("{]"));
    }
    #[test]
    fn paired_with_whitespace() {
        assert!(brackets_are_balanced("{ }"));
    }
    #[test]
    fn partially_paired_brackets() {
        assert!(!brackets_are_balanced("{[])"));
    }
    #[test]
    fn simple_nested_brackets() {
        assert!(brackets_are_balanced("{[]}"));
    }
    #[test]
    fn several_paired_brackets() {
        assert!(brackets_are_balanced("{}[]"));
    }
    #[test]
    fn paired_and_nested_brackets() {
        assert!(brackets_are_balanced("([{}({}[])])"));
    }
    #[test]
    fn unopened_closing_brackets() {
        assert!(!brackets_are_balanced("{[)][]}"));
    }
    #[test]
    fn unpaired_and_nested_brackets() {
        assert!(!brackets_are_balanced("([{])"));
    }
    #[test]
    fn paired_and_wrong_nested_brackets() {
        assert!(!brackets_are_balanced("[({]})"));
    }
    #[test]
    fn paired_and_incomplete_brackets() {
        assert!(!brackets_are_balanced("{}["));
    }
    #[test]
    fn too_many_closing_brackets() {
        assert!(!brackets_are_balanced("[]]"));
    }
    #[test]
    fn early_incomplete_brackets() {
        assert!(!brackets_are_balanced(")()"));
    }
    #[test]
    fn early_mismatched_brackets() {
        assert!(!brackets_are_balanced("{)()"));
    }
    #[test]
    fn math_expression() {
        assert!(brackets_are_balanced("(((185 + 223.85) * 15) - 543)/2"));
    }
    #[test]
    fn complex_latex_expression() {
        let input = "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \
                 \\end{array}\\right)";
        assert!(brackets_are_balanced(input));
    }
}
