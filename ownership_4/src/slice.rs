//iter : returns each element in a collection
// enumerate : wrap the result of iter and return an element as part of a tuple instead

// take a look at this function 
pub fn first_word (s: &String) -> usize {
    let bytes = s.as_bytes(); //converts string to an array of bytes
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    } 
    s.len()
}

pub fn first_word2 (s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..1];
        }
    } 
    &s[..]
}

pub fn main() {
    let mut t = String::from("hello world");
    let pos = first_word(&t); // because the return is not of &Str, ownership of t is dropped after the funtion runs
    println!("{}", pos);
    //now clear the string t
    t.clear(); //this works because there's currently no immutable reference to t

    //notice that pos still has the index of 5, but is now completely useless, since the intitial string has been cleared. 
    //hence, the index could easily get out of sync with the data in the
    // hence, the variable index 'pos' and the string 't' aren't connected in any way. How do we keep them in sync? 
    //we do so by using String slices
    //take this string below for example
    let u = String::from("how are you");
    let first = &u[0..3]; //takes the reference of the string from index zero to three (excluding three)
    //note that the slice DS stores the starting position and length of the slice. hence 'first above' contains a pointer to the first
    // byte of u, with a length of 3
    //to start at the first index, drop the 0. Also, if your slice includes the last byte, you can drop the trailing number. 
    //you can also drop both first and 
    //last numbers to take slices of the entire string.
    //also note that if we write let s = "fifty". this string literal is actually a slice that points to specific locations in binary. 
    //hence it's of the &str type. that's why it's immutable

    // now use a slice in the function above. notice that if you clear t in this case, the first_word function still works. this is because
    // the value we get this time points to the desired location of the string, and hence is tied to it.
    let mut v = String::from("how far");
    let pos2 = first_word2(&v); //because the return is &Str, this ownership is not dropped. pos2 now has an immutable reference to v
    println!("{}", pos2);
    
    println!("{}", first);

    v.clear(); // this prints an error. this is because v.clear() tries to make a mutable reference in order to truncate the string. 
    //but we passed an immutable reference of v to pos2 via the first_word2 function already
    // and recall that we can't have both a mutable and immutable reference to something in the same scope.
    //println!("{}", pos2);

}

