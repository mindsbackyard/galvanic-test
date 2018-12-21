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

mod setup_only {
    use galvanic_test::{fixture, TestFixture};

    fixture!( setup_only() -> i32 {
        setup(&mut self) {
            42
        }
    });

    #[test]
    fn should_create_i32() {
        let params = &();
        let mut fixture = setup_only::new(params);
        assert_eq!(fixture.setup().val, 42);
    }

    #[test]
    fn should_get_single_unit_parameter() {
        assert_eq!(
            setup_only::parameters().unwrap().collect::<Vec<_>>(),
            vec![()]
        );
    }
}

mod with_tear_down {
    use galvanic_test::{fixture, TestFixture};

    static mut TEAR_DOWN_FLAG: bool = false;

    fixture!( setup_only() -> i32 {
        setup(&mut self) {
            42
        }

        tear_down(&self) {
            unsafe { TEAR_DOWN_FLAG = true; }
        }
    });

    #[test]
    fn should_tear_down_fixture() {
        {
            let params = &();
            setup_only::new(params);
        }
        assert!(unsafe { TEAR_DOWN_FLAG });
    }
}
