fn main() {
    println!("Hello, world!");



    // Moves

    // In Rust, for most types, operations like assigning a value to a variable, passing it to a function, or returning it from a function don't copy the value, they move it. The source relinquishes ownership of the value to the destination, and becomes uninitialized. The destination now controls the value's lifetime. Rust programs build up and tear down complex structures one value at a time, one move at a time.

    // Consider the following Python code:
    // s = ['udon', 'ramen', 'soba']
    // t = s
    // u = s
    // Each Python object carries a reference count, tracking the number of values that are currently referring to it. To start, since only s is pointing to the list, the list's reference count is 1. Since the list is the only object pointing to the strings, each of their reference counts is also 1. So what happens with t and u? Python implements assignment simply by making the destination point to the same object as the source, and incrementing the object's reference count.
    // So this means that when it's just s, there is 1 reference count, with t and u, the reference count is now 3. So if we want to free a value, we must keep in mind how many references there are and where they are.

    // For C++ instead of reference counts, the language makes a copy of each list to t and u. So instead of 3 variables referencing a list with 3 values as in Python, C++ will have 3 lists, with 9 total values, tripling the memory usage. This is a large issue to contend with in C++.
    // See page 132 and 133 for diagrams to illustrate this.

    // In a sense, C++ and Pyton have chosen opposite trade-offs. Python makes assignment cheap, at the expense of requiring reference counting (and in the general case, garbage collection). C++ keeps the ownership of all the memory clear, at the expense of making assignment carry out a deep copy of the object. Deep copies can be bad as they can be expensive and there are more practical alternatives.

    // So how does the above work in Rust?
    let s = vect!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s;
    let u = s;

    // Like C and C++ Rust puts plain string literals like "udon" in read-only memory. So to make the comparison equal we call to_string here to get heap-allocated String values.
    // Like the C++, Rust will allocate s a stack frame which is allocated to the heap which contains the three strings. See page 135 for diagram.
    // Recall that in Rust, assignments of most types move the value from the source to the destination, leaving the source uninitialized. So looking at t, it takes on the vector's three header fields from s, t now owns the vector. The vector's elements stayed just where they were, and nothing happened to the strings either. Every value still has a single owner, although one has changed hands. No changes to reference counts and the compiler considers s uninitialized.
    // When we get to u, it would assign the uninitialized value s to u. Rust prohibits using uninitialized values, so the compiler rejects the code with a "ownership_double_move" error.

    // So like Python Rust's assignment is cheap, but it also is like C++ where ownership is always clear. A win win. The price of this however is that we must explicitly ask for copies when we want them. To do so, me must call the vector's clone method which perform a deep copy of the vector and its elements:
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s.clone()
    let u = s.clone();
    // We can also re-create Python's behaviour by using Rust's reference-counted pointer types which will be discussed shortly.



}
