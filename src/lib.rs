use std::ops::Drop;
use std::fmt::Debug;

#[cfg(feature = "galvanic_mock_integration")]
pub fn testfunc() {
    println!("feature present");
}

pub trait TestFixture<P, R> : Drop
        where P: Debug + 'static {
    fn new(curried_params: P) -> Self;

    fn parameters() -> Option<Box<Iterator<Item=P>>> { None }

    fn parameterise() -> Option<Box<Iterator<Item=Self>>>
            where Self: std::marker::Sized {
        if let Some(iter) = Self::parameters() {
            Some(Box::new(iter.map(|params| Self::new(params))))
        } else {
            None
        }
    }

    fn setup(&self) -> FixtureBinding<Self, R>
            where Self: std::marker::Sized;

    fn tear_down(&self) { }
}

pub struct FixtureBinding<'fixture, F:'fixture, R> {
    pub val: R,
    pub params: &'fixture F
}

impl<'fixture, F:'fixture, R> FixtureBinding<'fixture, F, R> {
    pub fn decompose(self) -> (R, &'fixture F) {
        (self.val, self.params)
    }

    pub fn into_val(self) -> R {
        self.val
    }

    pub fn into_params(self) -> &'fixture F {
        self.params
    }
}

/// Creates a new `TestFixture` implementation.
///
/// A `fixture!` requires a name, parameters and a
#[macro_export]
macro_rules! fixture {
    ( @impl_drop $name:ident ) => {
        impl ::std::ops::Drop for $name {
            fn drop(&mut self) {
                use ::galvanic_test::TestFixture;
                self.tear_down();
            }
        }
    };

    ( @impl_struct $name:ident $($param:ident : $param_ty:ty),* ) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug)]
        pub struct $name {
            $(pub $param : $param_ty),*
        }
    };

    ( @new_method $param:ident : $param_ty:ty ) => {
        fn new($param : $param_ty) -> Self {
            Self { $param, }
        }
    };
    ( @new_method $($param:ident : $param_ty:ty),+ ) => {
        fn new(($($param),*) : ($($param_ty),*)) -> Self {
            Self { $($param),* }
        }
    };

    ( $name:ident ( ) -> $ret_ty:ty {
          setup(&$self_setup:ident) $setup_body:block
          $(tear_down(&$self_td:ident) $tear_down_body:block)*
      }
    ) => {
        fixture!(@impl_struct $name );

        impl ::galvanic_test::TestFixture<(), $ret_ty> for $name {
            fn new(_: ()) -> Self { Self {} }
            fn parameters() -> Option<Box<Iterator<Item=()>>> {
                panic!("Internal error: `parameters()` should never be called on a fixture without parameters")
            }
            fn parameterise() -> Option<Box<Iterator<Item=Self>>>
                    where Self: ::std::marker::Sized {
                Some(Box::new(Some(Self::new(())).into_iter()))
            }
            fn setup(&$self_setup) -> ::galvanic_test::FixtureBinding<Self, $ret_ty> {
                let value = $setup_body;
                ::galvanic_test::FixtureBinding {
                    val: value,
                    params: $self_setup
                }
            }
            $(fn tear_down(&$self_td) $tear_down_body)*
        }

        fixture!(@impl_drop $name);
    };

    ( $name:ident ($($param:ident : $param_ty:ty),+) -> $ret_ty:ty {
          $(params $params_body:block)*
          setup(&$self_setup:ident) $setup_body:block
          $(tear_down(&$self_td:ident) $tear_down_body:block)*
      }
    ) => {
        fixture!(@impl_struct $name $($param : $param_ty),*);

        impl ::galvanic_test::TestFixture<($($param_ty),*), $ret_ty> for $name {
            fixture!(@new_method $($param : $param_ty),*);
            fn parameters() -> Option<Box<Iterator<Item=($($param_ty),*)>>> {
                (None as Option<Box<Iterator<Item=($($param_ty),*)>>>)
                $(; Some(Box::new($params_body)))*
            }
            fn setup(&$self_setup) -> ::galvanic_test::FixtureBinding<Self, $ret_ty> {
                let value = $setup_body;
                ::galvanic_test::FixtureBinding {
                    val: value,
                    params: $self_setup
                }
            }
            $(fn tear_down(&$self_td) $tear_down_body)*
        }

        fixture!(@impl_drop $name);
    };
}


#[macro_export]
macro_rules! test {
    ( @parameters | $body:block $test_case_failed:ident ) => { $body };

    ( @parameters | $body:block $test_case_failed:ident $($fixture_obj:ident)+) => {
        let described_parameters = format!("{:?}", ($(&$fixture_obj),*));
        let result = ::std::panic::catch_unwind(|| $body);
        if result.is_err() {
            println!("The above error occured with the following parameterisation of the test case: {}\n",
                     described_parameters);
            $test_case_failed.set(true);
        }
    };

    ( @parameters , $($remainder:tt)+ ) => {
        test!(@parameters $($remainder)*);
    };

    ( @parameters $fixture:ident ( $($expr:expr),* ) $($remainder:tt)+ ) => {
        let fixture_obj = $fixture::new(($($expr),*));
        let $fixture = fixture_obj.setup();
        test!(@parameters $($remainder)* fixture_obj);
    };

    ( @parameters $fixture:ident $($remainder:tt)+ ) => {
        match $fixture::parameterise() {
            Some(iterator) => {
                for fixture_obj in iterator {
                    let $fixture = fixture_obj.setup();
                    test!(@parameters $($remainder)* fixture_obj);
                }
            },
            None => panic!(concat!(
                "If a test fixture should be injected without supplying parameters, ",
                "it either needs to have no arguments ",
                "or a `params` block returning an iterator of parameter tuples ",
                "must be given for the fixture."))
        }
    };

    ( $(#[$attr:meta])* $name:ident | $($args_and_body:tt)* ) => {
        #[test]
        $(#[$attr])*
        fn $name() {
            // Cell is a workaround for #![allow(unused_mut)] which would affect the whole fn
            let test_case_failed = ::std::cell::Cell::new(false);
            test!(@parameters $($args_and_body)* test_case_failed);
            if test_case_failed.get() {
                panic!("Some parameterised test cases failed");
            }
        }
    };

    ( $(#[$attr:meta])* $name:ident $body:block ) => {
        #[test]
        $(#[$attr])*
        fn $name() {
            $body
        }
    };
}

#[macro_export]
#[cfg(not(feature = "galvanic_mock_integration"))]
macro_rules! test_suite {
    // named test suite
    ( name $name:ident ; $($remainder:tt)* ) => {
        #[cfg(test)]
        mod $name {
            __test_suite_int!(@int $($remainder)*);
        }
    };

    // anonymous test suite
    ( $($remainder:tt)* ) => {
        #[cfg(test)]
        mod __test {
            __test_suite_int!(@int $($remainder)*);
        }
    };
}

#[macro_export]
#[cfg(feature = "galvanic_mock_integration")]
macro_rules! test_suite {
    // named test suite
    ( name $name:ident ; $($remainder:tt)* ) => {
        #[cfg(test)]
        #[use_mocks]
        mod $name {
            __test_suite_int!(@int $($remainder)*);
        }
    };

    // anonymous test suite
    ( $($remainder:tt)* ) => {
        #[cfg(test)]
        #[use_mocks]
        mod __test {
            __test_suite_int!(@int $($remainder)*);
        }
    };
}

#[macro_export]
macro_rules! __test_suite_int {
    // internal: fixture in test_suite
    ( @int $(#[$attr:meta])* fixture $name:ident ($($param:ident : $param_ty:ty),*) -> $ret_ty:ty {
          $(setup(&$self_setup:ident) $setup_body:block)*
          $(tear_down(&$self_td:ident) $tear_down_body:block)*
      } $($remainder:tt)*
    ) => {
        fixture!( $(#[$attr])* $name ($($param : $param_ty),*) -> $ret_ty {
              $(setup(&$self_setup) $setup_body)*
              $(tear_down(&$self_td) $tear_down_body)*
        });
        __test_suite_int!(@int $($remainder)*);
    };

    // internal: test in test_suite
    ( @int $(#[$attr:meta])* test $name:ident ( $($fixture:ident $(($($expr:expr),*))*),* )
            $body:block
            $($remainder:tt)*
    ) => {
        test!( $(#[$attr])* $name | $($fixture $(($($expr),*))* ),* | $body);
        __test_suite_int!(@int $($remainder)*);
    };

    // internal: arbitrary item in test suite
    ( @int $item:item
            $($remainder:tt)*
    ) => {
        $item
        __test_suite_int!(@int $($remainder)*);
    };

    // internal: empty test suite
    ( @int ) => { };
}
