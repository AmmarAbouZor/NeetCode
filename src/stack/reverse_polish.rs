//! Reverse Polish Notation.
//! Similar to Stack functional programming interpretation

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut values = Vec::new();

    for t in &tokens {
        match t.as_str() {
            "+" => apply_operation(Operation::Add, &mut values),
            "-" => apply_operation(Operation::Subtract, &mut values),
            "*" => apply_operation(Operation::Multiply, &mut values),
            "/" => apply_operation(Operation::Divide, &mut values),
            num => {
                let num: i32 = num.parse().expect("We have only valid numbers");
                values.push(num);
            }
        }
    }

    assert_eq!(values.len(), 1, "We must have one value only at the end");

    values.pop().unwrap()
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn apply_operation(op: Operation, values: &mut Vec<i32>) {
    let right = values.pop().unwrap();
    let left = values.pop().unwrap();

    let val = match op {
        Operation::Add => left + right,
        Operation::Subtract => left - right,
        Operation::Multiply => left * right,
        Operation::Divide => left / right,
    };

    values.push(val);
}
