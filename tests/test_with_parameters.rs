#[macro_use] extern crate galvanic_test;

mod fixture_without_parameters {
    use galvanic_test::TestFixture;

    fixture!( test_fixture() -> i32 {
        setup(&self) {
            42
        }
    });

    test!( inject_fixture | test_fixture | {
        assert_eq!(test_fixture.val, 42);
    });
}

mod fixture_with_single_parameter {
    use galvanic_test::TestFixture;

    fixture!( fixture_with_single_param(x: i32) -> i32 {
        setup(&self) {
            self.x * 2
        }
    });

    test!( inject_fixture | fixture_with_single_param(21) | {
        assert_eq!(fixture_with_single_param.val, 42);
    });
}

mod fixture_with_parameters {
    use galvanic_test::TestFixture;

    fixture!( fixture_with_params(x: i32, y: i32) -> i32 {
        setup(&self) {
            self.x * self.y
        }
    });

    test!( inject_fixture | fixture_with_params(6, 7) | {
        assert_eq!(fixture_with_params.val, 42);
    });
}

mod multiple_fixtures_without_parameters {
    use galvanic_test::TestFixture;

    fixture!( test_fixture1() -> i32 {
        setup(&self) {
            42
        }
    });

    fixture!( test_fixture2() -> i32 {
        setup(&self) {
            23
        }
    });

    test!( inject_fixture | test_fixture1, test_fixture2 | {
        assert_eq!(test_fixture1.val, 42);
        assert_eq!(test_fixture2.val, 23);
    });
}
