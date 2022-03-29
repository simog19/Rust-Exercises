## Exercise 1 (warm-up)

Implement a function for capitalizing first char of each word present in a text s, ignoring other upper chars present in it.

- fn capitalize (s: &str) -> String { }

Words are separated by one or more space.
Then, read a sequence of words as argument from command line, call the function and print the outcome.

 - Example: cargo run -- "this is   à sentence"

Also write the test cases needed to verify the code correctness. Cover at least the following cases:
- a string with more than one word;
- a string with one word and without;
- a string with accented letter as first word characters;
- an empty string;
- a string with more than one space.


