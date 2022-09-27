// unit test allow you to quickly test a part of your code in isolation from the rest of your code
//unit test files should be in the src directory of the code they're testing [tests.rs, annonated with cfg(test)]
//this annotation tells rust to compile test.rs only when you run <cargo test>
// note that the cfg means the item should only be included given a particular configuration option (test in this case)
// note that tests can test private functions too.