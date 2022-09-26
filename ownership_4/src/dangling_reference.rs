// if you have a reference to some data, the compiler will ensure that the data doesn't go out of scope before the reference of the data does
fn runner() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { //dangle returns a reference to a string
    let s = String::from("hello"); // a new string s is created

    &s // we return reference to string s
} // s goes out of scope. hence, nothing can be returned because it's not stored anymore. s is deallocated  when the code of "dangle"
// finishes running, since s was created inside the dangle function. but if you simply return s, the ownership of s is moved out to
//another variable, and nothing is deallocated.