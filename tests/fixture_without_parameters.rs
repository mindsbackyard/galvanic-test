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
        let fixture = setup_only::new(());
        assert_eq!(fixture.setup().val, 42);
    }

    #[test]
    #[should_panic]
    fn should_fail_to_get_parameters() {
        setup_only::parameters();
    }

    #[test]
    fn should_have_a_single_parameterisation() {
        let mut iter = setup_only::parameterise().expect("No fixtures generated");
        assert_eq!(iter.next().expect("No parameterised fixture returned").setup().val, 42);
        assert!(iter.next().is_none());
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
            setup_only::new(());
        }
        assert!(unsafe { TEAR_DOWN_FLAG });
    }
}
