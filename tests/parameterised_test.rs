#[macro_use] extern crate galvanic_test;
use galvanic_test::TestFixture;
#[cfg(feature = "galvanic_mock_integration")] extern crate galvanic_mock;

fixture!( parameterised_fixture(x: i32, y: i32) -> i32 {
    params {
        vec![(1,2), (2,3), (3,4)].into_iter()
    }
    setup(&self) {
        self.x * self.y
    }
});

test!( inject_parameterised_fixtures | parameterised_fixture | {
    let (value, params) = parameterised_fixture.decompose();
    assert_eq!(value, params.x * params.y);
});

test!( override_parameterised_fixture | parameterised_fixture(6,7) | {
    assert_eq!(parameterised_fixture.val, 42);
});
