pub mod parser;
pub mod lexer;
mod syn;

// Runs with default radix 10
// const RADIX: u32 = 10;

/* Calculator interpreter
 * ======================
 * The interpreter is a finite state machine
 * that uses tokens for transitioning.
 * The grammar must be such:
 * [number][operator][number][operator]....
 *
 * The lexer is also a finite state machine
 * I'm really tired of looking at that mess
 * It does not perform grammar checking
 */
