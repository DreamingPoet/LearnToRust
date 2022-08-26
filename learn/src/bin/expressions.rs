
// 7. Expressions
//  A Rust program is (mostly) made up of a series of statements.
fn main() {

    let x = 5;
    // statements = expression + ;
    x;

    x + 1;

    15;

    // Blocks are expressions too,
    // so they can be used as values in assignments.
    // The last expression in the block will be assigned to the place expression such as a local variable.
    // However, if the last expression of the block ends with a semicolon, the retuen value wil be ().
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x
    };
    let z = { 2 * x};
}