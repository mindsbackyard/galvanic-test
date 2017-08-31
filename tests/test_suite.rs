#![cfg_attr(feature = "galvanic_mock_integration", feature(proc_macro))]
#[macro_use] extern crate galvanic_test;
#[cfg(feature = "galvanic_mock_integration")] extern crate galvanic_mock;
#[cfg(feature = "galvanic_mock_integration")] use galvanic_mock::*;

test_suite! {
    name named_test_suite1;
    use galvanic_test::TestFixture;

    fixture test_fixture() -> i32 {
        setup(&self) {
            42
        }
    }

    test inject_fixture(test_fixture) {
        assert_eq!(test_fixture.val, 42);
    }
}

test_suite! {
    name named_test_suite2;
    use galvanic_test::TestFixture;

    fixture fixture_with_params(x: i32, y: i32) -> i32 {
        setup(&self) {
            self.x * self.y
        }
    }

    test inject_fixture(fixture_with_params(6, 7)) {
        assert_eq!(fixture_with_params.val, 42);
    }
}
