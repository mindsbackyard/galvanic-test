# Galvanic-test: easier test setup for Rust
[![Build Status](https://travis-ci.org/mindsbackyard/galvanic-test.svg?branch=master)](https://travis-ci.org/mindsbackyard/galvanic-test)
[![Crates.io](https://img.shields.io/crates/v/galvanic-test.svg)](https://crates.io/crates/galvanic-test)

This crate provides a framework for creating test suites, managing their shared dependencies, and for writing parameterised tests.

 * supports test fixtures to handle setup/tear-down of test dependencies
 * test fixtures with parameters provide more flexibility in test setups
 * parameterised tests---write your test once and verify it in different scenarios
 * automatic dependency injection of test fixtures into your tests
 * richer test design with **[galvanic-assert](https://www.github.com/mindsbackyard/galvanic-assert)**---express more specific properties of parameterised tests
 * test suites integrate with **[galvanic-mock](https://www.github.com/mindsbackyard/galvanic-mock)**---no manual `#[use_mocks]` is needed
 * other panic-based assertion and mocking frameworks of your choice can be used too

 The crate is part of **galvanic**---a complete test framework for **Rust**.
 The framework is shipped in three parts, so you can choose to use only the parts you need.

## A short introduction to galvanic-test

**Galvanic-test** simplifies the setup and tear-down of your test environments and helps you to organise your tests.
Everything you already know about Rust testing should still apply.

Tests are organised in test suites which are either named or anonymous.
```Rust
#[macro_use] extern crate galvanic_test;
use galvanic_test::*;

// test suites are only built when a test is executed, e.g., with `cargo test`
test_suite! {
    // for anonymous test suites remove the name directive
    name my_test_suite;

    // suites act as modules and may contain any item
    fn calc(a: i32, b: i32) -> i32 { a*b }

    // instead of `fn`, `test` defines a test item.
    test simple_first_test() {
        assert_eq!(3*2, 6);
    }

    // attributes can still be applied as for functions
    #[should_panic]
    test another_test() {
        assert_eq!(calc(3,2), 7);
    }
}
```

The most powerful part of **galvanic-test** are test fixtures to manage your test environments.
A test fixture is a piece of code which setups *one* specific part of a test and makes sure that it's torn down after the test executed (even if it failed).
If you know [pytest](https://docs.pytest.org/en/latest/) you should feel at home.
If you have experience with XUnit-style frameworks, e.g., JUnit, CPPUnit, ...; then you can think about fixtures as different `before`/`after` blocks which belong together.
```Rust
#[macro_use] extern crate galvanic_test;

test_suite! {
    use std::fs::{File, remove_file};
    use std::io::prelude::*;

    fixture bogus_number() -> i32 {
        setup(&mut self) {
            42
        }
    }

    fixture input_file(file_name: String, content: String) -> File {
        members {
            file_path: Option<String>
        }
        setup(&mut self) {
            let file_path = format!("/tmp/{}.txt", self.file_name);
            self.file_path = Some(file_path.clone());
            {
                let mut file = File::create(&file_path).expect("Could not create file.");
                file.write_all(self.content.as_bytes()).expect("Could not write input.");
            }
            File::open(&file_path).expect("Could not open file.")
        }
        // tear_down is optional
        tear_down(&self) {
            remove_file(self.file_path.as_ref().unwrap()).expect("Could not delete file.")
        }
    }

    // fixtures are arguments to the tests
    test a_test_using_a_fixture(bogus_number) {
        assert_eq!(21*2, bogus_number.val);
    }

    // fixtures with arguments must receive the required values
    test another_test_using_fixtures(input_file(String::from("my_file"), String::from("The stored number is: 42"))) {
        let mut read_content = String::new();
        input_file.val.read_to_string(&mut read_content).expect("Couldn't read 'my_file'");

        assert_eq!(&read_content, input_file.params.content);
    }
}
```

Test fixtures enable us also to run the same test code with different parameterisations.
This can significantly reduce our work required for testing complex code with multiple execution paths.
```Rust
test_suite! {
    fixture product(x: u32, y: u32) -> u32 {
        params {
            vec![(2,3), (2,4), (1,6), (1,5), (0,100)].into_iter()
        }
        setup(&mut self) {
            self.x * self.y
        }
    }

    test a_parameterised_test_case(product) {
        let wrong_product = (0 .. *product.params.y).fold(0, |p,_| p + product.params.x) - product.params.y%2;
        // fails for (2,3) & (1,5)
        assert_eq!(wrong_product, product.val)
    }
}
```

## Documentation

**Galvanic-test** simplifies the setup of shared test environments, i.e., it helps us to create and reset the resources needed by our tests work properly.

It is recommended that you add `galvanic-test` as a dev-dependency in your `Cargo.toml`.
Make sure to use an appropriate version specification.
The crate follows semantic versioning.
```toml
[dev-dependencies]
galvanic-test = "*" // insert the appropriate version instead of "*"
```
After specifying the dependency we include the library with enabled macros in our `main.rs`,`lib.rs`, and/or our integration tests in `tests/`.
```Rust
#[macro_use] extern crate galvanic_test;
```

### Creating test suites for grouping tests

Before we start writing tests we have a look at how to group them.
Tests are organized in test suites.
A test suite takes care of several things:
* They create a private module to group test cases and test fixtures.
* They are only built if tests are built, e.g., using `cargo test`.
* Test fixtures defined in the suite can be injected into its test cases.
* If the `galvanic_mock_integration` feature is enabled then the test suite uses an implicit `#[use_mocks]` directive. (*nightly*)

They come in two varieties: *anonymous* and *named*.
To create a *anonymous* test suite we use the `test_suite!` macro.
```Rust
test_suite! {
    // ...
}
```
For easier location of a failing test case it is recommended to name a test suite.
```Rust
test_suite! {
    name some_identifer_naming_the_suite;
    // ...
}
```
Note that the `name` directive must occur as the first element of the suite.

### Writing tests in test suites

Now that we have defined a test suite, we can fill it with test cases.
A test case is defined as a `test` item.
```Rust
test_suite! {
    test my_first_test_case() {
        // ... some assertions
        assert_eq!(1+1, 2);
    }
}
```
If we want to define a test which is expected to panic we can simply use the `#[should_panic]` attribute or if we need more fine grained control we may use `galvanic-assert`'s `assert_that!(..., panics);` macro.
```Rust
test_suite! {
    #[should_panic]
    test a_panicking_test_case() {
        // ... some failing assertion
        assert_eq!(1+1, 4);
    }
    test a_panicking_test_case_using_galvanic_assert() {
        assert_that(panic("No towels!"), panics);
    }
}
```
So far test cases behave similar to functions annotated with `#[test]` as in simple Rust unit tests.
Though, test cases defined as a `test` item support automatic injection of test fixtures and parametrisation, as we will see later.

### Adding test fixtures for test resource management

Tests often depend on some resources of the test environment, e.g., objects used by the test, files with input, etc.
All those things must be created at the beginning of the test and torn down at the end of the test.
If we forget or mess up one of those tasks we introduce errors in test code, which is actually not central to the test.
Further if many parts of these environments are the same or similar for several test cases then the problem gets even worse.

To keep our tests clean we do not want to code setup and tear down tasks multiple times.
Therefore we write a *test fixture* for each resource.
```Rust
fixture a_number() -> i32 {
    setup(&mut self) {
        42
    }
    tear_down(&self) {
        println!("Cleaning up ...");
    }
}
```
Every fixture definition consists of the following parts:
* the `fixture` keyword
* a *name*: `a_number` in our example
* a list of typed arguments: *none* in our example
* the *type* of the *resource* managed by the fixture: `i32` here
* a required `setup` block which receives the fixture (`self`) as a mutable borrow and must return a resource of the type specified by the fixture
* an *optional* `tear_down` block which receives the fixture (`self`) as an immutable borrow

To use our new fixture in a test it must be defined in the same `test_suite!`.
The fixtures required by a test are given as parameters for test case by name.
Before the test is executed, `setup` method is invoked.
Its return value is then wrapped in a `FixtureBinding` and the binding is injected into test case.
The return value can then be accessed by the binding's `val` member.
```Rust
test a_test_using_a_fixture(a_number) {
    assert_eq!(a_number.val, 42);
}
```

#### Test fixtures with arguments

Often setting up exactly the same resource for several tests is not enough and we'd like to parameterise the `setup`/`tear_down` code.
We can do so by specifying arguments for the fixture.
```Rust
fixture offset_number(offset: i32) -> i32 {
    setup(&mut self) {
        self.offset + 42
    }
    tear_down(&self) {
        println!("Cleaning up a number with offset {} ...", self.offset);
    }
}
```
The arguments are then accessible as members of the fixture.
A test can then specify the required arguments when requesting the fixture.
The arguments passed to the fixture are accessible in the test case through the `FixtureBinding`'s `params` member by the names used in the fixture definition.
```Rust
test a_test_using_a_fixture(offset_number(8)) {
    assert_eq!(offset_number.val, 42 + offset_number.params.offset);
}
```

#### Sharing data between setup() and tear_down()

We've seen that fixture arguments are available both in the `setup` and `tear_down` blocks via `self`.
However there are situations where we depend on some external input, e.g., the system time, a random number, both in our setup code and tear-down code to, e.g., create unique file names or other identifiers.
With the facilities shown so far we have no (non-hacky) way to transfer the information.

To get around this issue we can define member variables for our fixtures.
A member variable is accessible via `self` and is always an `Option` which is initialised with `None`.
The `setup` block may then overwrite the members' values (therefore its `&mut self`).

To declare member variables we need to place a `members` block before the `setup` block and list our variable declarations ase we would in a `struct`.
```Rust
fixture offset_number() -> i32 {
    members {
        some_identifier: Option<i32>
    }
    setup(&mut self) {
        self.some_identifier = Some(12)
        42
    }
    tear_down(&self) {
        println!("Cleaning up a fixture with identifier {} ...", self.some_identifier.as_ref().unwrap();
    }
}
```

### Writing parameterised tests and fixtures

A very powerful feature of `galvanic-test` is the ability to parameterise tests.
A parameterised test case is run with several different initialisations of its fixtures.

First we need a test case which accepts one or multiple fixtures.
Let's write a test which calculates the product of two numbers `(x,y)` by summing `x`, `y`-times.
```Rust
test parameterised_test(product) {
    let sum: u32 = (0..product.params.y).fold(0, |a,b| a + product.params.x);
    assert_eq!(sum, product.val);
}
```

We want to test this code snippet with different values to test the border cases and equivalence classes.
For we create a `product` fixture with arguments `x` and `y` and let the `setup` block calculate the product of the two numbers.
To make the fixture parameterised we add a `params` block at the beginning.
The block must return an `Iterator<R>` where `R` is the type of the fixture's return value.
```Rust
fixture product(x: u32, y: u32) -> u32 {
    params {
        vec![(2,3), (1,4), (0,100)].into_iter()
    }
    setup(&mut self) {
        self.x * self.y
    }
}
```

Now if we run our tests each test case with takes the `product` fixture as an argument without supplying parameters to the fixture will take the values from the `params` block instead.
The `setup` and `tear_down` block will be executed before/after each parameterisation.

If the test case takes multiple parameterised fixtures then all possible combinations (the cross-product) will be evaluated.
Again before/after each parameterisation **all** `setup`/`tear_down` blocks of the parameterised fixtures will be executed.

If on the other hand you provide parameters to a parameterised fixture, as shown below, then only that parameterisation will be considered for the fixture.
```Rust
test parameterised_test(product(3,8)) {
    let sum: u32 = (0..product.params.y).fold(0, |a,b| a + product.params.x);
    assert_eq!(sum, 24);
}
```

#### Errors, `#[should_panic]`, and parameterised fixtures

Let's see what happens if a test fails.
```Rust
test failing_parameterised_test(product) {
    let sum: i32 = (0..*product.params.y).fold(0, |a,b| a + product.params.x);
    assert_eq!(sum, product.val - product.params.x%2)
}
```

The framework will show you all parameterisations which triggered an error so debugging will be easier.
```
...
running 1 test
thread 'test::parameterised_test' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `3`', src/main.rs:17:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.
The above error occured with the following parameterisation of the test case:
    product { x: 1, y: 4 }

thread 'test::parameterised_test' panicked at 'Some parameterised test cases failed', src/main.rs:3:0
test test::parameterised_test ... FAILED
...
```

Be careful when applying `#[should_panic]` to a parameterised test case.
In that case the test will succeed if **any** parameterisation fails.
To assert that all parameterisation fail it's recommended to use `assert_that!(..., panics)` from the `galvanic-assert` crate to treat panicking like a regular behaviour.

### Enabling Galvanic-mock integration
If you want to use **galvanic-mock** integration (only available on nightly) then add
```Rust
#[macro_use] extern crate galvanic_test;
#![feature(proc_macro)]
extern crate galvanic_mock;
```
and enable the `galvanic_mock_integration` feature in your `Cargo.toml`
```toml
[dev-dependencies]
galvanic-test = { version = "*", features = ["galvanic_mock_integration"] }
galvanic-mock = "*" // replace with the correct version
```

Afterwards each test suite will automatically apply the `#[use_mocks]` attribute so you can use fixtures to return actual mock objects.
