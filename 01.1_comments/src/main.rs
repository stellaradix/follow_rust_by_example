fn main() {
	// Regular comments which are ignored by the compiler:

	// Line comments which go to the end of the line.
	/* Block comments which go to the closing delimiter. */

	/* Doc comments which are parsed into HTML library documentation:

	/// Generate library docs for the following item.
	//! Generate library docs for the enclosing item.

	*/

	let x = 5 + /* 90 + */ 5;

	println!("Is 'x' 10 or 100? x = {}", x);
}
