#The Bank Kata in Rust

During this kata, I found the challenge was in re-thinking the design when compared to completing this kata in more "traditionally OOP" languages such as C#, Java etc..

Solutions in these languages then to use a PrintService or a DateService in order to control dates and make assertions on the printed statement, without having to unnecessarily change production code to facilitate such testing strategies.

Ultimately, the design that made the most sense to me (without over-engineering the tests to get around this stateless design) was to rethink dependencies in Rust - even though these entities are classes in OOP languages, they can simply be closures in Rust.
In making this design change, testing became simple - as did the production code itself.

If anyone with more Rust experience would like to give feedback on this approach, I'd really appreciate any pointers.

Thanks!
