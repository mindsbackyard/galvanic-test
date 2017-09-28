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
use galvanic_test::TestFixture;

fixture!( with_members(x: i32, y: i32) -> () {
    members {
        data_int: Option<i32>,
        data_float: Option<f32>
    }
    setup(&mut self) {
        self.data_int = Some(self.x + self.y);
        self.data_float = Some((self.x + self.y) as f32 * 2.0);
    }
    tear_down(&self) {
        assert_eq!(self.data_int, Some(41));
        assert_eq!(self.data_float, Some(82.0));
    }
});

#[test]
fn should_create_fixture_with_members_and_assign_expected_values() {
    let mut fixture = with_members::new(&(40, 1));
    fixture.setup();
}

#[test]
#[should_panic]
fn should_create_fixture_with_members_and_assign_wrong_values() {
    let mut fixture = with_members::new(&(40, 2));
    fixture.setup();
}
