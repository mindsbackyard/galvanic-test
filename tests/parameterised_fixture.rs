#[macro_use] extern crate galvanic_test;
use galvanic_test::TestFixture;


fixture!( test_fixture(x: i32, y: i32) -> i32 {
    params {
        vec![(1,1), (2,4), (3,6)].into_iter()
    }
    setup(&self) {
        self.x * self.y
    }
});

#[test]
fn should_generate_multiple_fixtures() {
    let cases = test_fixture::parameterise()
                             .expect("A case iterator should be returned")
                             .map(|f| {
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
