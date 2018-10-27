/* Copyright 2018 Alan Somers
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

//! As of Rustc 1.30.0 the `test_suite` macro should be importable with `use`,
//! rather than `macro_use`

#![cfg_attr(feature = "galvanic_mock_integration", feature(proc_macro_hygiene))]
#[cfg(feature = "galvanic_mock_integration")] extern crate galvanic_mock;
extern crate galvanic_test;

use galvanic_test::*;

test_suite! {
    test simple_test_in_unnamed_test_suite() {
        assert!(true);
    }
}
