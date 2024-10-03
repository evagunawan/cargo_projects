# Hello, world
In rust add a comment with two /s
```
\\ This wouldn't run as a function
```

This main function will write hello world to the terminal when ./main is run in the terminal
```
fn main() {
    println!("Hello, world!");
}
```

This is how to start a function

```
fn main() {}
```

If you want to create a function that takes an argument, add the argument variables into the ()
```
fn main(arg1, arg2) {}
```

The action that is done in the function is smooshed inbetween the {} brackets
```
fn main(arg1, arg2) {
    function
}
```

To call a macro that is innate to Rust, use !. Macros do not always follow the sample rules as a traditional function.
```
fn main() {
    println!("Hello, world!");
}
```

Remove the ! if you do not want to run a macro. 
```
fn main() {
    println("Hello, world!");
}
```

To indent a line use 4 spaces NOT TAB
```
fn main() {
    println!("Hello, world!");
}
```

Hello world is a string that is passed with println

Almost all rust expressions end with a ; to indicate the next expression can begin.

To run a rust program, you must compile it first, similar to c and c++.
```
rustc main.rs
```

Once compiled, Rust will output a binary executable that can be run.
```
ls
chapter_1.rs    main    main.rs
```

To run that program, use ./main. If it does not run add ./main.exe to the command line
```
./main
Hello, world!
```

Rust is an "ahead of time compiled" language. You can compile your program and give someone else the executable to run without them having rust.

For simple tools, rustc is a good enough compiler. If you have a more intensive program, there is another answer!
