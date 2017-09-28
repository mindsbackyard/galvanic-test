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

#![cfg(feature = "galvanic_mock_integration")]

#![feature(proc_macro)]
#[macro_use] extern crate galvanic_test;
extern crate galvanic_mock;

test_suite! {
    name named_test_suite1;
    use galvanic_test::TestFixture;

    fixture test_fixture() -> i32 {
        setup(&mut self) {
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
        setup(&mut self) {
            self.x * self.y
        }
    }

    test inject_fixture(fixture_with_params(6, 7)) {
        assert_eq!(fixture_with_params.val, 42);
    }
}

test_suite! {
    name parameterised_test_suite;
    use galvanic_test::TestFixture;

    fixture fixture_with_params(x: i32, y: i32) -> i32 {
        params {
            vec![(1,2), (2,3), (3,4)].into_iter()
        }
        setup(&mut self) {
            self.x * self.y
        }
    }

    test inject_fixture(fixture_with_params) {
        let params = &fixture_with_params.params;
        assert_eq!(fixture_with_params.val, params.x*params.y);
    }
}
