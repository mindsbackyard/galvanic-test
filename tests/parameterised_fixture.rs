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


fixture!( test_fixture(x: i32, y: i32) -> i32 {
    params {
        vec![(1,1), (2,4), (3,6)].into_iter()
    }
    setup(&mut self) {
        self.x * self.y
    }
});

#[test]
fn should_generate_multiple_fixtures() {
    let cases = test_fixture::parameters()
                             .expect("A case iterator should be returned")
                             .map(|p| {
                                 let mut f = test_fixture::new(&p);
                                 let b = f.setup();
                                 (b.val, b.params.y - b.params.x)
                             })
                             .collect::<Vec<_>>();



    assert_eq!(cases.len(), 3);

    assert_eq!(cases[0].0, 1);
    assert_eq!(cases[0].1, 0);

    assert_eq!(cases[1].0, 8);
    assert_eq!(cases[1].1, 2);

    assert_eq!(cases[2].0, 18);
    assert_eq!(cases[2].1, 3);
}

#[test]
fn should_get_fixture_parameters() {
    assert_eq!(test_fixture::parameters().unwrap().collect::<Vec<_>>(),
               vec![(1,1), (2,4), (3,6)]);
}
