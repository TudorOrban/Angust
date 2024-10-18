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

#[allow(dead_code)]
pub fn parse_string_to_ast(input: &str) -> Result<ASTNode, pest::error::Error<Rule>> {
    let mut pairs = parse_expression(input)?;
    let ast = parse_pair_to_ast(pairs.next().unwrap());
    Ok(ast)
}

fn parse_pair_to_ast(pair: Pair<Rule>) -> ASTNode {
    match pair.as_rule() {
        Rule::number => {
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
        _ => unreachable!(),
    }
}

fn parse_operation(pair: Pair<Rule>) -> ASTNode {
    let mut inner_pairs = pair.clone().into_inner();
    let mut left = parse_pair_to_ast(inner_pairs.next().unwrap());

    for (op_pair, right_pair) in inner_pairs.tuples() {
        let op_str = op_pair.as_str();
        let op = str_to_operator(op_str);
        let right = parse_pair_to_ast(right_pair);

        left = match pair.as_rule() {
            Rule::logical_expression => ASTNode::LogicalOperation { operator: op, left: Box::new(left), right: Box::new(right) },
            Rule::comparison_expression => ASTNode::Comparison { operator: op, left: Box::new(left), right: Box::new(right) },
            _ => ASTNode::BinaryOperation { operator: op, left: Box::new(left), right: Box::new(right) },
        };
    }
    left
}