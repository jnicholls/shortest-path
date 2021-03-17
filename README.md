# shortest-path

## Usage
	shortest-path
	Find the shortest path, changing one character at a time, between two words of equal length.

	USAGE:
		shortest-path <START_WORD> <END_WORD>

	FLAGS:
		-h, --help       Prints help information
		-V, --version    Prints version information

	ARGS:
		<START_WORD>    The starting word.
		<END_WORD>      The ending word.

## Description
	// Problem statement:
	//  Write a function that returns the "shortest path" between two given words.

	// Examples:
	//  1. fn("cat", "dog")   returns [cat, cot, dot, dog]
	//  2. fn("van", "car")   returns [van, can, car]
	//  3. fn("name", "norm") returns [name, came, come, core, corm, norm]

	// Assumptions you can make:
	//  1. Assume that you have access to an immutable, global Set<String> which
	//     contains 200,000 valid English dictionary words of varying lengths.
	// 2. Assume that the provided Set<String> supports all standard set
	//     operations, including iteration.

	// Requirements for solution:
	//  1. Each word in the returned path must differ from the previous word by
	//     exactly one-character substitution. In other words, the Hamming
	//     Distance between adjacent words in the returned path must equal 1.
	//  2. All words in the returned path must be present in the provided
	//     Set<String> dictionary. In other words, all words in the returned path
	//     must be valid dictionary words.
	//  3. If multiple paths exist between the two given words, return the
	//     shortest path.

