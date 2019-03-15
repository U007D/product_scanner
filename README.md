## Product Scanner
### Downloading Source
```bash
git clone https://github.com/u007d/product_scanner.git
```

### Executing the Code
```bash
cargo test
```

Note: A command-line interface to construct a price and purchase list is not currently implemented.
Please run `product_scanner --help` from the command line for more information.

### Viewing the Documentation
```bash
cargo doc --open
```

### Notes
In implementing this code, careful attention was paid to the given constraints:

#### Include test cases with your code
I have largely used the Test-Driven Development (TDD) process, including red-green-refactor for this work.  I have
included unit tests at least touching any logic I have written of even moderate signficance.  It is worth point out that
many engineers feel tests are used to prove their code works, and that, of course is true, but I feel the main value of
tests actually lies elsewhere.

Tests provide the ability to verify that *future changes* to the code base do not break
required contracts/invariants.  Should someone make a change which invalidates an existing contract, a test should go
red, clearly indicating something has gone wrong.

Combined with loose coupling, it is much easier to work on large codebases, since one does not have to know everything
about the entire codebase (which becomes impossible fairly quickly) in order to be productive and effective in it.

#### Include a readme to describe how to execute your code and test cases
This README.md contains this information plus more.

#### the code should be shared on your GitHub (give us the GitHub link)
If you are reading this, you have found the repository! :)  If you happen to be reading this file and it is not part of
a code repository, please find the repository this file came from [here](https://github.com/u007d/product_scanner.git).

#### Don't use any mutable `vars`, nulls or throw Exceptions
Translated to Rust, this was taken to mean don't use `mut`able bindings.

To that end, there are no `mut`able bindings in this implementation.  To achieve this, I implemented the solution using 
a functional programming paradigm.  Further, I chose the Builder Pattern to isolate the construction of the `PriceList` 
to one type, namely, `PriceListBuilder`.  Once built, the `PriceList` has no way of being mutated.

#### Don't use nulls
Fortunately, in Safe Rust, there are no nulls.

#### Don't throw Exceptions
Translated to Rust, this was taken to mean don't panic.

Without rewriting the Rust standard library (e.g. to elegantly handle `push`ing to a `Vec` already containing `usize` 
elements) *and* running with infinite memory, it is not possible to guarantee no panics.  But with those (extreme)
situations excepted, I believe I believe this implementation to be panic-free*.

*The main remaining potential source of panics is from arithmetic operations--some people consider overflow panics to be
an acceptable risk, while others do not.  To that end, I created two solutions.  Of course the implementation using
checked arithmetic is much more robust, but at a considerable cost to complexity.  For the purposes of this exercise,
I wanted to write a solution which did, in fact, meet all of the constraints, even under the strictest of 
interpretations.  The branch containing the checked arithmetic solution can be found
[here](https://github.com/U007D/product_scanner/tree/checked_arithmetic).

#### Bonus points: demonstrate use of a type parameter
I have made use of type parameters in a few places, but I will call out just one:
A type parameter is used in the a blanket implementation of the `AsRefStr` extension method such that the 
`as_product_list()` convenience function (this fn converts a convenient product string such as "ABCD" to a strongly 
typed slice of `Products` in the form of `[Product::A, Product::B, Product::C, Product::D]`) is available on all 
relevant `std` types, such as `str` and `String`, for example.

#### Bonus points: demonstrate use of a higher order function
The `Terminal` struct makes heavy use of closures, which when passed as parameters to functions (the functional 
combinators in this specific case) is use of higher order functions.

#### Correctness and IEEE754 floating point values
Floating point math is notoriously difficult to get right.  Comparing values is frought with pitfalls, even 
`assert_eq!(0.1 + 0.2, 0.3)` will assert `false` (this is not Rust's fault, it's the IEEE754 standard for floating point
numbers).  For a "financial application" to ensure correctness, all non-integer calculations performed by this program
use `Decimal` representation as opposed to floating point values to avoid these issues.  Please see the included 
documentation on `OrdDecimal` for more information (`cargo doc --open`).  
