#[macro_use] extern crate galvanic_test;

mod setup_only {
    use galvanic_test::TestFixture;

    fixture!( setup_only() -> i32 {
        setup(&self) {
            42
        }
    });

    #[test]
    fn should_create_i32() {
        let params = &();
        let fixture = setup_only::new(params);
        assert_eq!(fixture.setup().val, 42);
    }

    #[test]
    fn should_get_single_unit_parameter() {
        assert_eq!(setup_only::parameters().unwrap().collect::<Vec<_>>(),
                   vec![()]
        );
    }
}

mod with_tear_down {
    use galvanic_test::TestFixture;

    static mut TEAR_DOWN_FLAG: bool = false;

    fixture!( setup_only() -> i32 {
        setup(&self) {
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
