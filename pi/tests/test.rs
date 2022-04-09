use pi::{Params, Term};

#[test]
fn term_test() {
    let p = Params::new((1, 2),(1, 2), (1, 4));
    let mut t = Term::new(5, &p);
    t.print();
    assert!(false);
}
