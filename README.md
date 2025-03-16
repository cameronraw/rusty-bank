# The Bank Kata in Rust

During this kata, I found the challenge was in **re-thinking the design** when compared to completing this kata in more "traditionally OOP" languages such as C# or Java.  

In those languages, solutions often introduce a `PrintService` or `DateService` to control dates and make assertions on printed statements, avoiding unnecessary changes to production code for testing purposes. However, Rust's ownership model and preference for **explicit dependencies** encouraged a different approach.

## Rethinking Dependencies in Rust

Instead of treating these services as **classes**, I realized they could simply be **closures** in Rust. This shift simplified both production code and testing, without over-engineering solutions just to fit an OOP mindset.

For example:
- **Instead of a `PrintService` class**, we can inject a `print_fn: Fn(&[Transaction])` closure.
- **Instead of a `DateService` class**, we can pass a `now_fn: Fn() -> String` closure.

This keeps the code lightweight, avoids unnecessary traits, and makes dependencies explicit. As a result, testing became straightforward—simply passing a mock closure was enough.