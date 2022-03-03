# Rusty-Fuzzer

A multi-threaded mutation fuzzer in Rust. Read the blog post [here](https://sophieboyle.github.io/2022/03/03/rusty-fuzzer-a-multi-threaded-mutation-fuzzer-in-rust.html)

I wanted to learn fuzzing, and have also been learning Rust at University. I figured I would combine the both into a project. I'm also using this post as practice for writing blogs in the future.

This is very heavily based off of the blog-post/tutorial written by [Hombre](https://twitter.com/h0mbre_): [Fuzzing Like a Caveman Part 1](https://h0mbre.github.io/Fuzzing-Like-A-Caveman/#). The key differences is that my fuzzer is written in Rust, but also that the implementation uses Rust multi-threading. I won't repeat everything that Hombre has already detailed very well in their own blog, but I will briefly summarise the key points, and then discuss the multi-threaded soup at the end.

Rusty_fuzzer gets it's name from not only being written in Rust, but because nobody has probably used a fuzzer this simple in the field in over a decade. However, it's a great learning opportunity.
