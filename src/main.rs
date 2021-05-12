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

    // In a sense, C++ and Python have chosen opposite trade-offs. Python makes assignment cheap, at the expense of requiring reference counting (and in the general case, garbage collection). C++ keeps the ownership of all the memory clear, at the expense of making assignment carry out a deep copy of the object. Deep copies can be bad as they can be expensive and there are more practical alternatives.

    // So how does the above work in Rust?
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s;
    let u = s;

    // Like C and C++ Rust puts plain string literals like "udon" in read-only memory. So to make the comparison equal we call to_string here to get heap-allocated String values.
    // Like the C++, Rust will allocate s a stack frame which is allocated to the heap which contains the three strings. See page 135 for diagram.
    // Recall that in Rust, assignments of most types move the value from the source to the destination, leaving the source uninitialized. So looking at t, it takes on the vector's three header fields from s, t now owns the vector. The vector's elements stayed just where they were, and nothing happened to the strings either. Every value still has a single owner, although one has changed hands. No changes to reference counts and the compiler considers s uninitialized.
    // When we get to u, it would assign the uninitialized value s to u. Rust prohibits using uninitialized values, so the compiler rejects the code with a "ownership_double_move" error.

    // So like Python Rust's assignment is cheap, but it also is like C++ where ownership is always clear. A win win. The price of this however is that we must explicitly ask for copies when we want them. To do so, me must call the vector's clone method which perform a deep copy of the vector and its elements:
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s.clone();
    let u = s.clone();
    // We can also re-create Python's behaviour by using Rust's reference-counted pointer types which will be discussed shortly.



    // More Operations That Move

    // In the examples this far, we've shown initializations, providing values for variables as they come into scope in a let statement. Assigning to a variable is slightly different, in that if you move a value into a variable that was already initialized, Rust drops the variable's prior value. For example:
    let mut s = "Govinda".to_string();
    s = "Siddhartha".to_string(); // value "Govinda" dropped here

    // In this code, when the program assigns the string "siddhartha" to s, its prior value "Govinda" gets dropped first. But consider the following:
    let mut s ="Govinda".to_string();
    let t = s;
    s = "Siddhartha".to_string(); // nothing is dropped here

    // This time, t has taken ownership of the original string from s, so that by the time we assign to s, it is uninitialized. In this scenario, no string is dropped.

    // Rust applies move semantics to almost any use of a value. Passing arguments to functions moves ownership to the function's parameters; returning a value from a function moves ownership to the caller. Building a tuple moves the values into the tuple, and so on.
    // For example using a previous example:
    struct Person { name: String, birth: i32 }

    let mut composers = Vec::new();
    composers.push(Person { name: "Palestrina".to_string(), birth: 1525 });
    // This code shows several places at which moves occur, beyong initialization and assignment:
    // 1. Returning values from a function
        // The call Vec::new() constructs a new vector, and returns, not a pointer to the vector, but the vector itself. Its ownership moves from Vec::new to the variable composers. Similarly, the to_string call returns a fresh String instance.
    // 2. Constructing new values
        // The name field of the new Person structure is initialized with the return value of to_string. The structure takes ownership of the string.
    // 3. Passing values to a function
        // The entire Person structure, not just a pointer, is passed to the vector's push method, which moves it onto the end of the structure. The vector takes ownership of the Person, and thus becomes the indirect owner of the name String as well.

    // Moving values around like this may sound inefficient, but there are two things to keep in mind. First, the moves always apply to the value proper, not the heap storage they own. For vectors and strings, the value proper is a three-word header alone; the potentially large element arrays and text buffers sit where they are in the heap. Second, the Rust compiler's code generation is good at "seeing through" all these moves. In practice, the machine code often stores the value directly where it belongs.



    // Moves and Control Flow

    // The previous examples have very simple control flow. How do moves interact with more complicated code? The general principle is that, if it's possible for a variable to have had its value moved away, and it hasn't definitely been given a new value since, it's considered uninitialized. For example, if a variable still has a value after evaluating an if expression's condition, then we can use it in both branches:
    let x = vec![10, 20, 30];
    if c {
        f(x); //... ok to move from x here
    } else {
        g(x); // ... and ok to also move from x here
    }
    h(x) // bad: x is uninitialized here if either path uses it

    // For similar reasons, moving from a variable in a loop is forbidden:
    let x = vec![10, 20, 30];
    while f() {
        g(x); // bad: x would be moved in the first iteration,
              // uninitialized in second
    }

    // That is, unless we've definitely given it a new value by the next iteration:
    let mut x = vec![10, 20, 30];
    while f() {
        g(x); // move from x
        x = h(); // give x a fresh value
    }
    e(x);



    // Moves and Indexed Content

    // We've mentioned a move leaves the source uninitialized. But not every kind of value owner is prepared to become uninitialized. For example:
    // Build a vector of the strings "101", "102", ... "105"
    let mut v = Vec::new();
    for i in 101 .. 106 {
        v.push(i.to_string());
    }

    // Pull out random elements from the vector.
    let third = v[2];
    let fifth = v[4];

    // For this to work, Rust would somehow need to remember that the third and fifth elements of the vector have become uninitialized, and track that information until the vector is dropped. In the most general case, vectors would need to carry around extra info with them to indicate which elements are live and which have become uninitialized. That is clearly not the right behaviour for a systems programming language. A vector should be nothing but a vector. In fact, Rust rejects the preceding code with the following error:
    // ownership_move_out_of_vectors.rs...
    // let third = v[2];
    //              help: consider using a reference instead `&v[2]`

    // It also makes a similar complaint about the move to fifth. Rust in the error recommends using a reference, but what if we really want to move an element out of a vector? We'd need to find a method that does so in a way that respects the limitations of the type.

    // Here are three possibilities:
    // Build a vector of the strings "101", "102", ... "105"
    let mut v = Vec::new();
    for i in 101 .. 106 {
        v.push(i.to_string());
    }

    // 1. Pop a value off the end of the vector:
    // .unwrap() will give us the embedded T (Result<T, E> or Option<T>) if there is one. If instead not a T, but an E or None, it will panic.
    let fifth = v.pop().unwrap();
    assert_eq!(fifth, "105");

    // 2. Move a value out of the middle of the vector, and move the last element into its spot:
    let second = v.swap_remove(1);
    assert_eq!(second, "102")

    // 3. Swap in another value for the one we're taking out:
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    // Let's see what's left of our vector.
    assert_eq!(v, vec!["101", "104", "substitute"]);

    // Each of the above methods moves an element out of the vector, but does so in a way that leaves the vector in a state that is fully populated, if perhaps smaller.

    // Collection types like Vec also generally offer methods to consume all their elements in a loop:
    let v = vec!["liberte".to_string(),
        "egalite".to_string(),
        "fraternity".to_string()];

    for mut s in v {
        s.push("!");
        println!("{}", s);
    }

    // When we pass the vector to the loop directly, as in for ... in v, this moves the vector out of v, leaving v uninitialized. The for loop's internal machinery takes ownership of the vector, and dissects it into its elements. At each iteration, the loop moves another element to the variable s. Since s now owns the string, we're able to modify it in the loop body before printing it. Since the vector itself is no longer visible to the code, nothing can observe it mid-loop in some partially emptied state.

    // If we do find ourselves needing to move a value out of an owner that the compiler can't track, we might consider changing the owner's type to something that can dynamically track whether it has a value or not. For example, here's a variant on the earlier example:
    struct Person { name: Option<String>, birth: i32 }

    let mut composers = Vec::new();
    composers.push(Person { name: Some("Palestrina".to_string()), birth: 1525 });

    // We can't do this:
    let first_name = composers[0].name;

    // That will just elicit the same "cannot move out of indexed content" error shown earlier. But because we've changed the type of the name field from String to Option<String>, that means that None is a legitimate value for the field to hold, so this works:
    let first_name = std::mem::replace(&mut composers[0].name, None);
    assert_eq!(first_name, Some("Palestrina".to_string()));
    assert_eq!(composers[0].name, None);

    // The replace call moves out the value of composer[0].name, leaving None in its place, and passes ownership of the original value to its caller. In fact, using Option this way is common enough that the type provides a take method for this very purpose. We could write the preceding manipulation more legibly as follows:
    let first_name = composers[0].name.take();

    // This call to take has the same effect as the earlier call to replace.



    
    

}
