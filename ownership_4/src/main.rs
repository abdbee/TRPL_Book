mod mutable_reference;
mod slice;

fn main() {
    mutable_reference::mutable();
    mutable_reference::mutable_and_immutable();
    //dangling_reference::runner();
    slice::main();


    

}