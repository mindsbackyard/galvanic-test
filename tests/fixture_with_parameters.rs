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

// #[macro_use] extern crate galvanic_test;
use galvanic_test::{fixture, TestFixture};

static mut TEAR_DOWN_VALUE: i32 = 0;

fixture!( test_fixture(x: i32, y: i32) -> i32 {
    setup(&mut self) {
        self.x * self.y
    }

    tear_down(&self) {
        unsafe {
            TEAR_DOWN_VALUE = self.y - self.x;
        }
    }
});

#[test]
fn should_create_binding_access_parameters_and_tear_down() {
    {
        let params = &(2, 3);
        let mut fixture = test_fixture::new(params);
        let binding = fixture.setup();
        assert_eq!(binding.val, binding.params.x * binding.params.y);
    }
    assert_eq!(unsafe { TEAR_DOWN_VALUE }, 1);
}

#[test]
fn should_have_no_parameters() {
    assert!(test_fixture::parameters().is_none());
}
