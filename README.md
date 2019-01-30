# Leetcode Rust

My Leetcode solutions in Rust.

When coding the solution, make sure to write the unit tests as well. Run the test using

```
cargo test
```

You can either create the file for each problem manually or run this command below to generate a template for that file.

```
cargo run -- --medium file_name
```

For example, the problem __Find Anagram Mappings__ is in category __Easy__, so to generate a file using the template, run

```
cargo run -- --easy find_anagram_mappings
```

The command above will create the file `src/easy/find_anagram_mappings.rs`.

Don't forget to add the newly created file in its respective mod, e.g.

```
// src/easy.rs
pub mod find_anagram_mappings;
```
