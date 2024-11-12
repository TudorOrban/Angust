use pest::Parser;
use pest_derive::Parser;


#[derive(Parser)]
#[grammar = "src/parsing/expression/expression.pest"]
pub struct ExpressionParser;

pub fn parse_expression(input: &str) -> Result<pest::iterators::Pairs<Rule>, pest::error::Error<Rule>> {
    ExpressionParser::parse(Rule::expression, input)
}