This repository demonstrates a common error in Rust related to its borrowing rules. The `bug.rs` file contains code that attempts to create an immutable reference after a mutable reference has been established, leading to a compile-time error.  The solution, provided in `bugSolution.rs`, shows how to correctly manage borrowing to avoid this issue.