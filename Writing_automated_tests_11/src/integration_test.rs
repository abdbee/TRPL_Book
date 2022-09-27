/*
these tests are
- external to your library
- only call functions that are part of your library's public API
- purpose is to test whether various parts of your library work properly together.

## to create an integration test:
- create a test directory at the top level of project directory, next to src
- create as many test files as you want in this directory
- note that since each test in the directory is a crate, we need to bring the library into scope of each test 
- also, no need to annotate with cfg(test), since files in this directory are only compiled when we run cargo test.
- you can also run individual functions, just like in unit tests
- you can also run all the functions in an integration test file by running: cargo --test test_name


*/