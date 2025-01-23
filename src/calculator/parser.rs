use pest::Parser;
use pest_derive::Parser;
use std::fmt;

#[derive(Debug)]
pub enum CalculatorError {
    ParseError(String),
    DivisionByZero,
    UnknownOperator(String),
    MissingOperand,
    MissingOperator,
    UnexpectedRule(String),
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculatorError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CalculatorError::DivisionByZero => write!(f, "Division by zero"),
            CalculatorError::UnknownOperator(op) => write!(f, "Unknown operator: {}", op),
            CalculatorError::MissingOperand => write!(f, "Missing operand"),
            CalculatorError::MissingOperator => write!(f, "Missing operator"),
            CalculatorError::UnexpectedRule(rule) => write!(f, "Unexpected rule: {:?}", rule),
        }
    }
}

#[derive(Parser)]
#[grammar = "src/calculator/calculator.pest"] // 定义语法规则的文件
pub struct CalculatorParser;

/// 对外提供的计算函数
/// 输入：一个字符串表达式，例如 "(+ 1 (+ 2 3))"
/// 输出：计算结果，例如 6
pub fn calculate(input: &str) -> Result<f64, CalculatorError> {
    let pairs = CalculatorParser::parse(Rule::expression, input)
        .map_err(|e| CalculatorError::ParseError(e.to_string()))?;

    let mut result = 0.0;
    for pair in pairs {
        result = eval_expression(pair)?;
    }
    Ok(result)
}

fn eval_expression(pair: pest::iterators::Pair<Rule>) -> Result<f64, CalculatorError> {
    let mut inner = pair.into_inner();

    // Extract the operator
    let operator = inner.next().ok_or(CalculatorError::MissingOperator)?;

    // Extract the operands
    let operand1 = inner.next().ok_or(CalculatorError::MissingOperand)?;
    let operand2 = inner.next().ok_or(CalculatorError::MissingOperand)?;

    // Evaluate the operands
    let val1 = eval_operand(operand1)?;
    let val2 = eval_operand(operand2)?;

    // Perform the operation
    match operator.as_str() {
        "+" => Ok(val1 + val2),
        "-" => Ok(val1 - val2),
        "*" => Ok(val1 * val2),
        "/" => {
            if val2 == 0.0 {
                Err(CalculatorError::DivisionByZero)
            } else {
                Ok(val1 / val2)
            }
        }
        op => Err(CalculatorError::UnknownOperator(op.to_string())),
    }
}

fn eval_operand(pair: pest::iterators::Pair<Rule>) -> Result<f64, CalculatorError> {
    match pair.as_rule() {
        Rule::number => pair
            .as_str()
            .parse()
            .map_err(|_| CalculatorError::ParseError("Failed to parse number".to_string())),
        Rule::expression => eval_expression(pair),
        Rule::operand => {
            // If the operand is itself a nested expression, we need to evaluate it
            let inner = pair
                .into_inner()
                .next()
                .ok_or(CalculatorError::MissingOperand)?;
            eval_operand(inner)
        }
        rule => Err(CalculatorError::UnexpectedRule(format!("{:?}", rule))),
    }
}

#[test]
fn test_calculate() {
    assert_eq!(calculate("(+ 1 2)").unwrap(), 3.0);
    assert_eq!(calculate("(+ 1.5 2.5)").unwrap(), 4.0);
    assert_eq!(calculate("(+ 1 (+ 2 3))").unwrap(), 6.0);
    assert_eq!(calculate("(* 2.5 (+ 3 4))").unwrap(), 17.5);
    assert!(calculate("(/ 10 0)").is_err()); // 测试除以零错误
    assert!(calculate("(+ 1)").is_err()); // 测试缺少操作数错误
    assert!(calculate("(& 1 2)").is_err()); // 测试未知操作符错误
}
