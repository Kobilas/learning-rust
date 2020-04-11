fn main() {
    println!("Hello, world!");
}

// cargo new <project>: creates new Rust project
    // creates structured directory, with src folder and Cargo.toml file
    // also initializes Git repository
// cargo build: builds and compiles Rust project
    // creates Cargo.lock file that keeps track of exact dependencies in project
        // managed automatically by Cargo
    // saves result in /target/build rather than in same directory as source or root of project
    // when compiling code for release, use cargo build ==release
        // utilizes optimizations which takes longer to run
        // saves result to /target/release
        // can benchmark code by utilizing executable in /target/release
    // cargo run: compiles and runs executable
// cargo check: checks to make sure that code compiles but does not create executable
