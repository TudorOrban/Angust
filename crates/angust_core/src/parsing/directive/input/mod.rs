
/*
 * Flow of input parsing: for each parent component:
 * 1. Create component with component factory registry
 * 2. Use parsing context: parent component state, functions and scanned inputs ASTs
 * 3. For each scanned input, evaluate the AST and pass it to parent component initialization function (input_evaluator)
 * 4. In parent component initialization:
 *   a. scan for inputs of depth 1 children components (input_scanner)
 *   b. trigger registered input setters with input values (input_setter)
 *   c. map DOM to elements, only after state is updated with input values
 */

pub mod input_scanner;
pub mod input_setter;
pub mod input_evaluator;

mod input_parser;