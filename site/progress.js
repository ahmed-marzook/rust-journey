// ─────────────────────────────────────────────────────────────
//  Your Rust learning progress. This is the ONLY file you need
//  to edit to update the site.
//
//  Section status: "done" | "doing" | "todo"
//  Add a journal entry whenever you finish something.
// ─────────────────────────────────────────────────────────────

const BOOK = "https://doc.rust-lang.org/book/";
const REPO = "https://github.com/ahmed-marzook/rust-journey";

const STARTED = "2026-10-16";

const JOURNAL = [
  // Newest first.
  { date: "2026-10-16", text: "Started the Rust Book. Installed Rust, wrote hello_world with rustc, now on Hello, Cargo!" },
];

const CHAPTERS = [
  { num: 1, title: "Getting Started", page: "ch01-00-getting-started.html", sections: [
    ["1.1", "Installation", "done", "ch01-01-installation.html"],
    ["1.2", "Hello, World!", "done", "ch01-02-hello-world.html"],
    ["1.3", "Hello, Cargo!", "doing", "ch01-03-hello-cargo.html"],
  ]},
  { num: 2, title: "Programming a Guessing Game", page: "ch02-00-guessing-game-tutorial.html", sections: [
    ["2", "Guessing game tutorial", "todo"],
  ]},
  { num: 3, title: "Common Programming Concepts", page: "ch03-00-common-programming-concepts.html", sections: [
    ["3.1", "Variables and Mutability", "todo"],
    ["3.2", "Data Types", "todo"],
    ["3.3", "Functions", "todo"],
    ["3.4", "Comments", "todo"],
    ["3.5", "Control Flow", "todo"],
  ]},
  { num: 4, title: "Understanding Ownership", page: "ch04-00-understanding-ownership.html", sections: [
    ["4.1", "What Is Ownership?", "todo"],
    ["4.2", "References and Borrowing", "todo"],
    ["4.3", "The Slice Type", "todo"],
  ]},
  { num: 5, title: "Using Structs to Structure Related Data", page: "ch05-00-structs.html", sections: [
    ["5.1", "Defining and Instantiating Structs", "todo"],
    ["5.2", "An Example Program Using Structs", "todo"],
    ["5.3", "Method Syntax", "todo"],
  ]},
  { num: 6, title: "Enums and Pattern Matching", page: "ch06-00-enums.html", sections: [
    ["6.1", "Defining an Enum", "todo"],
    ["6.2", "The match Control Flow Construct", "todo"],
    ["6.3", "Concise Control Flow with if let and let else", "todo"],
  ]},
  { num: 7, title: "Packages, Crates, and Modules", page: "ch07-00-managing-growing-projects-with-packages-crates-and-modules.html", sections: [
    ["7.1", "Packages and Crates", "todo"],
    ["7.2", "Defining Modules to Control Scope and Privacy", "todo"],
    ["7.3", "Paths for Referring to an Item in the Module Tree", "todo"],
    ["7.4", "Bringing Paths into Scope with the use Keyword", "todo"],
    ["7.5", "Separating Modules into Different Files", "todo"],
  ]},
  { num: 8, title: "Common Collections", page: "ch08-00-common-collections.html", sections: [
    ["8.1", "Storing Lists of Values with Vectors", "todo"],
    ["8.2", "Storing UTF-8 Encoded Text with Strings", "todo"],
    ["8.3", "Storing Keys with Associated Values in Hash Maps", "todo"],
  ]},
  { num: 9, title: "Error Handling", page: "ch09-00-error-handling.html", sections: [
    ["9.1", "Unrecoverable Errors with panic!", "todo"],
    ["9.2", "Recoverable Errors with Result", "todo"],
    ["9.3", "To panic! or Not to panic!", "todo"],
  ]},
  { num: 10, title: "Generic Types, Traits, and Lifetimes", page: "ch10-00-generics.html", sections: [
    ["10.1", "Generic Data Types", "todo"],
    ["10.2", "Traits: Defining Shared Behavior", "todo"],
    ["10.3", "Validating References with Lifetimes", "todo"],
  ]},
  { num: 11, title: "Writing Automated Tests", page: "ch11-00-testing.html", sections: [
    ["11.1", "How to Write Tests", "todo"],
    ["11.2", "Controlling How Tests Are Run", "todo"],
    ["11.3", "Test Organization", "todo"],
  ]},
  { num: 12, title: "An I/O Project: Building a Command Line Program", page: "ch12-00-an-io-project.html", sections: [
    ["12.1", "Accepting Command Line Arguments", "todo"],
    ["12.2", "Reading a File", "todo"],
    ["12.3", "Refactoring to Improve Modularity and Error Handling", "todo"],
    ["12.4", "Developing the Library's Functionality with TDD", "todo"],
    ["12.5", "Working with Environment Variables", "todo"],
    ["12.6", "Writing Error Messages to Standard Error", "todo"],
  ]},
  { num: 13, title: "Functional Language Features: Iterators and Closures", page: "ch13-00-functional-features.html", sections: [
    ["13.1", "Closures", "todo"],
    ["13.2", "Processing a Series of Items with Iterators", "todo"],
    ["13.3", "Improving Our I/O Project", "todo"],
    ["13.4", "Comparing Performance: Loops vs. Iterators", "todo"],
  ]},
  { num: 14, title: "More About Cargo and Crates.io", page: "ch14-00-more-about-cargo.html", sections: [
    ["14.1", "Customizing Builds with Release Profiles", "todo"],
    ["14.2", "Publishing a Crate to Crates.io", "todo"],
    ["14.3", "Cargo Workspaces", "todo"],
    ["14.4", "Installing Binaries with cargo install", "todo"],
    ["14.5", "Extending Cargo with Custom Commands", "todo"],
  ]},
  { num: 15, title: "Smart Pointers", page: "ch15-00-smart-pointers.html", sections: [
    ["15.1", "Using Box<T> to Point to Data on the Heap", "todo"],
    ["15.2", "Treating Smart Pointers Like Regular References", "todo"],
    ["15.3", "Running Code on Cleanup with the Drop Trait", "todo"],
    ["15.4", "Rc<T>, the Reference Counted Smart Pointer", "todo"],
    ["15.5", "RefCell<T> and the Interior Mutability Pattern", "todo"],
    ["15.6", "Reference Cycles Can Leak Memory", "todo"],
  ]},
  { num: 16, title: "Fearless Concurrency", page: "ch16-00-concurrency.html", sections: [
    ["16.1", "Using Threads to Run Code Simultaneously", "todo"],
    ["16.2", "Transfer Data Between Threads with Message Passing", "todo"],
    ["16.3", "Shared-State Concurrency", "todo"],
    ["16.4", "Extensible Concurrency with Send and Sync", "todo"],
  ]},
  { num: 17, title: "Async and Await", page: "ch17-00-async-await.html", sections: [
    ["17.1", "Futures and the Async Syntax", "todo"],
    ["17.2", "Applying Concurrency with Async", "todo"],
    ["17.3", "Working with Any Number of Futures", "todo"],
    ["17.4", "Streams: Futures in Sequence", "todo"],
    ["17.5", "A Closer Look at the Traits for Async", "todo"],
    ["17.6", "Futures, Tasks, and Threads", "todo"],
  ]},
  { num: 18, title: "Object-Oriented Programming Features", page: "ch18-00-oop.html", sections: [
    ["18.1", "Characteristics of Object-Oriented Languages", "todo"],
    ["18.2", "Using Trait Objects to Abstract over Shared Behavior", "todo"],
    ["18.3", "Implementing an Object-Oriented Design Pattern", "todo"],
  ]},
  { num: 19, title: "Patterns and Matching", page: "ch19-00-patterns.html", sections: [
    ["19.1", "All the Places Patterns Can Be Used", "todo"],
    ["19.2", "Refutability", "todo"],
    ["19.3", "Pattern Syntax", "todo"],
  ]},
  { num: 20, title: "Advanced Features", page: "ch20-00-advanced-features.html", sections: [
    ["20.1", "Unsafe Rust", "todo"],
    ["20.2", "Advanced Traits", "todo"],
    ["20.3", "Advanced Types", "todo"],
    ["20.4", "Advanced Functions and Closures", "todo"],
    ["20.5", "Macros", "todo"],
  ]},
  { num: 21, title: "Final Project: Building a Multithreaded Web Server", page: "ch21-00-final-project-a-web-server.html", sections: [
    ["21.1", "Building a Single-Threaded Web Server", "todo"],
    ["21.2", "From Single-Threaded to Multithreaded Server", "todo"],
    ["21.3", "Graceful Shutdown and Cleanup", "todo"],
  ]},
];
