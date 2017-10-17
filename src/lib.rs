/* Copyright 2017 Christopher Bacher
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::ops::Drop;
use std::fmt::Debug;

pub trait TestFixture<'param, P, R> : Drop
        where P: Debug + 'static {
    fn new(curried_params: &'param P) -> Self;

    fn parameters() -> Option<Box<Iterator<Item=P>>>;

    fn setup(&mut self) -> FixtureBinding<Self, R>
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
        impl<'param> ::std::ops::Drop for $name<'param> {
            fn drop(&mut self) {
                use ::galvanic_test::TestFixture;
                self.tear_down();
            }
        }
    };

    ( @impl_struct $name:ident Params[$($param:ident : $param_ty:ty),*] Members[$($member:ident : $member_ty:ty),*] ) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug)]
        pub struct $name<'param> {
            $(pub $param : &'param $param_ty,)*
            $($member : Option<$member_ty>,)*
        }
    };

    ( @new_method Params[$param:ident : $param_ty:ty] Members[$($member:ident),*] ) => {
        fn new($param : &'param $param_ty) -> Self {
            Self {
                $param,
                $($member: None,)*
            }
        }
    };
    ( @new_method Params[$($param:ident : $param_ty:ty),+] Members[$($member:ident),*] ) => {
        fn new(&($(ref $param),*) : &'param ($($param_ty),*)) -> Self {
            Self {
                $($param,)*
                $($member: None,)*
            }
        }
    };

    ( $name:ident ( ) -> $ret_ty:ty {
          $(members { $($member:ident : Option<$member_ty:ty>),* })*
          setup(& mut $self_setup:ident) $setup_body:block
          $(tear_down(&$self_td:ident) $tear_down_body:block)*
      }
    ) => {
        fixture!(@impl_struct $name Params[_phantom : ()] Members[$($($member : $member_ty),*),*]);

        impl<'param> ::galvanic_test::TestFixture<'param, (), $ret_ty> for $name<'param> {
            fn new(_phantom: &'param ()) -> Self {
                Self {
                    _phantom,
                    $($($member: None),*),*
                }
            }
            fn parameters() -> Option<Box<Iterator<Item=()>>> {
                Some(Box::new(Some(()).into_iter()))
            }
            fn setup(&mut $self_setup) -> ::galvanic_test::FixtureBinding<Self, $ret_ty> {
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
          $(members { $($member:ident : Option<$member_ty:ty>),* })*
          $(params $params_body:block)*
          setup(& mut $self_setup:ident) $setup_body:block
          $(tear_down(&$self_td:ident) $tear_down_body:block)*
      }
    ) => {
        fixture!(@impl_struct $name Params[$($param : $param_ty),*] Members[$($($member : $member_ty),*),*]);

        impl<'param> ::galvanic_test::TestFixture<'param, ($($param_ty),*), $ret_ty> for $name<'param> {
            fixture!(@new_method Params[$($param : $param_ty),*] Members[$($($member),*),*]);
            fn parameters() -> Option<Box<Iterator<Item=($($param_ty),*)>>> {
                (None as Option<Box<Iterator<Item=($($param_ty),*)>>>)
                $(; Some(Box::new($params_body)))*
            }
            fn setup(&mut $self_setup) -> ::galvanic_test::FixtureBinding<Self, $ret_ty> {
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

    ( @parameters | $body:block $test_case_failed:ident $(($fixture_obj:ident, $params:expr, $fixture:ident))+) => {
        let mut described_parameters = String::from("Test panicked before all fixtures have been assigned.");
        let result = ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| {
            let mut described_params = Vec::new();
            $(
                let params = &$params;
                let mut $fixture_obj = $fixture::new(params);
                described_params.push(format!("{:?}", $fixture_obj));
                let mut $fixture = $fixture_obj.setup();
                noop(&mut $fixture);
            )*
            described_parameters = described_params.join(", ");
            $body
        }));
        if result.is_err() {
            println!("The above error occured with the following parameterisation of the test case:\n    {}\n",
                     described_parameters);
            $test_case_failed.set(true);
        }
    };

    ( @parameters , $($remainder:tt)+ ) => {
        test!(@parameters $($remainder)*);
    };

    ( @parameters $fixture:ident ( $($expr:expr),* ) $($remainder:tt)+ ) => {
        test!(@parameters $($remainder)* (fixture_obj, ($($expr),*), $fixture));
    };

    ( @parameters $fixture:ident $($remainder:tt)+ ) => {
        match $fixture::parameters() {
            Some(iterator) => {
                for params in iterator {
                    test!(@parameters $($remainder)* (fixture_obj, params, $fixture));
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
            fn noop<F, R>(_: &::galvanic_test::FixtureBinding<F,R>) { }
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
            #[allow(unused_imports)] use ::galvanic_test::TestFixture;
            __test_suite_int!(@int $($remainder)*);
        }
    };

    // anonymous test suite
    ( $($remainder:tt)* ) => {
        #[cfg(test)]
        mod __galvanic_test {
            #[allow(unused_imports)] use ::galvanic_test::TestFixture;
            __test_suite_int!(@int $($remainder)*);
        }
    };
}

#[macro_export]
#[cfg(feature = "galvanic_mock_integration")]
macro_rules! test_suite {
    // named test suite
    ( name $name:ident ; $($remainder:tt)* ) => {
        #[allow(unused_imports)] use ::galvanic_mock::use_mocks;

        #[cfg(test)]
        #[use_mocks]
        mod $name {
            #[allow(unused_imports)] use ::galvanic_test::TestFixture;
            __test_suite_int!(@int $($remainder)*);
        }
    };

    // anonymous test suite
    ( $($remainder:tt)* ) => {
        #[allow(unused_imports)] use ::galvanic_mock::use_mocks;
        #[cfg(test)]
        #[use_mocks]
        mod __galvanic_test {
            #[allow(unused_imports)] use ::galvanic_test::TestFixture;
            __test_suite_int!(@int $($remainder)*);
        }
    };
}

#[macro_export]
macro_rules! __test_suite_int {
    // internal: fixture in test_suite
    ( @int $(#[$attr:meta])* fixture $name:ident ($($param:ident : $param_ty:ty),*) -> $ret_ty:ty {
          $(members { $($member:ident : Option<$member_ty:ty>),* })*
          $(params $params_body:block)*
          setup(& mut $self_setup:ident) $setup_body:block
          $(tear_down(&$self_td:ident) $tear_down_body:block)*
      } $($remainder:tt)*
    ) => {
        fixture!( $(#[$attr])* $name ($($param : $param_ty),*) -> $ret_ty {
              $(members { $($member : Option<$member_ty>),* })*
              $(params $params_body)*
              setup(& mut $self_setup) $setup_body
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
