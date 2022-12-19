#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}
use crate::CalculatorInput::*;
pub fn rpn_calculator(inputs: &[CalculatorInput]) -> Option<i32> {
    
    let mut arr: Vec<i32> = Vec::new();
    for ele in inputs {
        if let Value(num) = ele {
            arr.push(num.clone());
        } else {
            if arr.len() < 2 {
                return None;
            }
            let num_2 = arr.pop().unwrap();
            let num_1 = arr.pop().unwrap();
            match ele {
                Add => arr.push(num_1 + num_2),
                Subtract => arr.push(num_1 - num_2),
                Multiply => arr.push(num_1 * num_2),
                Divide => arr.push(num_1 / num_2),
                _ => return None,
            }
        };
    }
    return if arr.len() == 1 {Some(arr[0])} else {None}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing_input() {
        let input = [];
        assert_eq!(rpn_calculator(&input), None);
    }

    #[test]
    fn one_input_num() {
        let input = [
            CalculatorInput::Value(4)
        ];
        assert_eq!(rpn_calculator(&input).unwrap(), 4);
    }

    #[test]
    fn one_input_operand() {
        let input = [
            CalculatorInput::Add
        ];
        assert_eq!(rpn_calculator(&input), None);
    }

    #[test]
    fn too_few_operands_input() {
        let input = [CalculatorInput::Value(2), CalculatorInput::Add];
        assert_eq!(rpn_calculator(&input), None);
    }

    #[test]
    fn simple_input() {
        let input = [
            CalculatorInput::Value(2),
            CalculatorInput::Value(2),
            CalculatorInput::Add,
        ];
        assert_eq!(rpn_calculator(&input).unwrap(), 4);
    }

    #[test]
    fn multiple_input() {
        let input = [
            CalculatorInput::Value(4),
            CalculatorInput::Value(8),
            CalculatorInput::Add,
            CalculatorInput::Value(7),
            CalculatorInput::Value(5),
            CalculatorInput::Subtract,
            CalculatorInput::Divide,
        ];
        assert_eq!(rpn_calculator(&input).unwrap(), 6);
    }

    #[test]
    fn multiple_input_num() {
        let input = [
            CalculatorInput::Value(4),
            CalculatorInput::Value(8),
            CalculatorInput::Value(7),
            CalculatorInput::Value(5)
        ];
        assert_eq!(rpn_calculator(&input), None);
    }

    #[test]
    fn multiple_input_operand() {
        let input = [
            CalculatorInput::Add,
            CalculatorInput::Subtract,
            CalculatorInput::Multiply,
            CalculatorInput::Divide
        ];
        assert_eq!(rpn_calculator(&input), None);
    }
}
