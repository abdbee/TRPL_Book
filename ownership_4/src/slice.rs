//iter : returns each element in a collection
// enumerate : wrap the result of iter and return an element as part of a tuple instead

// take a look at this function 
pub fn first_word (s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    } 
    s.len()
}

pub fn main() {
    let mut t = String::from("hello world");
    let pos = first_word(&t);
    println!("{}", pos);
    //now clear the string t
    t.clear();
    //notice that pos still has the index of 5, but is now completely useless, since the intitial string has been cleared. hence, the index could easily get out of sync with the data in the
    // hence, the variable index 'pos' and the string 't' aren't connected in any way. How do we keep them in sync? we do so by using String slices
    //take this string below for example
    let u = String::from("how are you");
    let first = &u[0..3]; //takes the reference of the string from index zero to three (excluding three)
    //note that the slice DS stores the starting position and length of the slice. hence 'first above' contains a pointer to the first byte of u, with a length of 3
    
    println!("{}", first);

}

