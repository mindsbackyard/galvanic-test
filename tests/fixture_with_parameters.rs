#[macro_use] extern crate galvanic_test;
use galvanic_test::TestFixture;

static mut TEAR_DOWN_VALUE: i32 = 0;

fixture!( test_fixture(x: i32, y: i32) -> i32 {
    setup(&self) {
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
        let fixture = test_fixture::new(params);
        let binding = fixture.setup();
        assert_eq!(binding.val,
                   binding.params.x * binding.params.y);
    }
    assert_eq!(unsafe { TEAR_DOWN_VALUE }, 1);
}

#[test]
fn should_have_no_parameters() {
    assert!(test_fixture::parameters().is_none());
}
