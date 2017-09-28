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

#[macro_use] extern crate galvanic_test;

mod fixture_without_parameters {
    use galvanic_test::TestFixture;

    fixture!( test_fixture() -> i32 {
        setup(&mut self) {
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
        setup(&mut self) {
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
        setup(&mut self) {
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
        setup(&mut self) {
            42
        }
    });

    fixture!( test_fixture2() -> i32 {
        setup(&mut self) {
            23
        }
    });

    test!( inject_fixture | test_fixture1, test_fixture2 | {
        assert_eq!(test_fixture1.val, 42);
        assert_eq!(test_fixture2.val, 23);
    });
}
