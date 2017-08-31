#![cfg(feature = "galvanic_mock_integration")]

#![feature(proc_macro)]
#[macro_use] extern crate galvanic_test;
extern crate galvanic_mock;use galvanic_mock::*;

#[mockable]
trait TestTrait {
    fn func(&self) -> i32;
}

test_suite! {
    name inject_mock;
    use galvanic_test::TestFixture;
    use super::*;

    fixture test_mock() -> mock::TestMock {
        setup(&self) {
            let mock = new_mock!(TestTrait for TestMock);
            given! {
                <mock as TestTrait>::func |_| true then_return 42 always;
            }
            mock
        }
    }

    test inject_mock(test_mock) {
        let mock = test_mock.into_val();
        assert_eq!(mock.func(), 42);
    }
}
