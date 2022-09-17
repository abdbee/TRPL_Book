// to make a reference mutable, add 'mut' after the &
// to be able to mutate the reference to a variable, the variable itself must be mutable.

pub fn mutable() {
    let mut s = String::from("hell");

    //note that you can have only one mutable reference to a data in a particular scope. this is to prevent data race
    //let r1 = &mut s; //first mutable borrow.  uncomment this and try printing r1. you'll get an error telling you that you can't borrow s more than once
    //let r2 = &mut s; // second mutable borrow. uncomment this and try printing r2. you'll get an error telling you that you can't borrow s more than once
    change(&mut s); // third mutable borrow.
    println!("{}", s);

}

pub fn change(y: &mut String) {
    y.push_str("o, world");
}

// but you can always use curly braces to create a new scope for another mutable reference of the same data. 

pub fn mutable_and_immutable() {
    let mut s = String::from("hello");
    
    //creating multiple immutable references is not a problem. the data isn't being changed anyway.
    let r1 = &s;
    let r2 = &s; 
    //however, we cannot have an immutable reference together with a mutable reference in the same scope.
    // this is because those using the immutable reference might not be expecting that the value will change.
    //let r3 = &mut s; //uncommenting this and printing r1, r2 and r3 will print an error.

    // println!("{}, {}, {}", r1, r2, r3);
}
