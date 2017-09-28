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
        setup(&mut self) {
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
