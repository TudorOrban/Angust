use super::expression_parser::{parse_expression, Rule};
use itertools::Itertools;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    Number(f64),
    Identifier(String),
    FunctionCall(String, Vec<ASTNode>),
    BinaryOperation {
        operator: Operator,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    Comparison {
        operator: Operator,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    LogicalOperation {
        operator: Operator,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add, Subtract, Multiply, Divide,    // Arithmetic
    Equal, NotEqual, Less, Greater, LessEqual, GreaterEqual,  // Comparison
    And, Or,                            // Logical
}

fn str_to_operator(op_str: &str) -> Operator {
    match op_str {
        "+" => Operator::Add,
        "-" => Operator::Subtract,
        "*" => Operator::Multiply,
        "/" => Operator::Divide,
        "==" => Operator::Equal,
        "!=" => Operator::NotEqual,
        "<" => Operator::Less,
        ">" => Operator::Greater,
        "<=" => Operator::LessEqual,
        ">=" => Operator::GreaterEqual,
        "&&" => Operator::And,
        "||" => Operator::Or,
        _ => panic!("Unsupported operator: {}", op_str),
    }
}

pub fn parse_string_to_ast(input: String) -> Result<ASTNode, pest::error::Error<Rule>> {
    let mut pairs = parse_expression(&input)?;
    let root_pair = pairs.next().unwrap();  // Assuming there is at least one pair
    if let Some(second_pair) = pairs.peek() {
        println!("Second pair detected, indicating incomplete parsing at the top level: {:?}", second_pair);
    }
    let ast = parse_pair_to_ast(root_pair);
    Ok(ast)
}


fn parse_pair_to_ast(pair: Pair<Rule>) -> ASTNode {
    println!("Processing pair: {:?}, Content: '{}'", pair.as_rule(), pair.as_span().as_str());

    match pair.as_rule() {
        Rule::expression => {
            parse_expression_content(pair)
        },
        Rule::number => {
            println!("Number: {}", pair.as_str());
            ASTNode::Number(pair.as_str().parse::<f64>().unwrap())
        },
        Rule::identifier => {
            ASTNode::Identifier(pair.as_str().to_string())
        },
        Rule::function_call => {
            let mut inner_pairs = pair.into_inner();
            let name = inner_pairs.next().unwrap().as_str().to_string();
            let args = inner_pairs.map(parse_pair_to_ast).collect();
            ASTNode::FunctionCall(name, args)
        },
        Rule::logical_expression | Rule::comparison_expression | Rule::additive_expression | Rule::multiplicative_expression => {
            parse_operation(pair)
        },
        _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
    }
}

fn parse_expression_content(pair: Pair<Rule>) -> ASTNode {
    let mut inner_pairs = pair.into_inner();
    if let Some(first_pair) = inner_pairs.next() {
        parse_pair_to_ast(first_pair)
    } else {
        unreachable!("Expression rule must contain inner content")
    }
}

fn parse_operation(pair: Pair<Rule>) -> ASTNode {
    let mut inner_pairs = pair.clone().into_inner();
    let left = parse_pair_to_ast(inner_pairs.next().unwrap());

    let mut current = left;
    while let Some(op_pair) = inner_pairs.next() {
        let operator_str = op_pair.as_str();
        let operator = str_to_operator(operator_str);

        if let Some(right_pair) = inner_pairs.next() {
            let right = parse_pair_to_ast(right_pair);
            current = match pair.as_rule() {
                Rule::comparison_expression => ASTNode::Comparison {
                    operator,
                    left: Box::new(current),
                    right: Box::new(right),
                },
                Rule::logical_expression => ASTNode::LogicalOperation {
                    operator,
                    left: Box::new(current),
                    right: Box::new(right),
                },
                _ => ASTNode::BinaryOperation {
                    operator,
                    left: Box::new(current),
                    right: Box::new(right),
                },
            };
        } else {
            break;
        }
    }
    current
}
