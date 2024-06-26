use ocaml::{Error, FromValue, ToValue, Value};

fn test_basic_array() -> Result<(), Error> {
    ocaml::runtime::init();
    let gc = unsafe { ocaml::Runtime::recover_handle() };
    unsafe {
        let mut a: ocaml::Array<&str> = ocaml::Array::alloc(2);
        a.set(gc, 0, &"testing")?;
        a.set(gc, 1, &"123")?;
        let b: Vec<&str> = FromValue::from_value(a.to_value(gc));
        assert!(b.as_slice() == &["testing", "123"]);
        Ok(())
    }
}

fn test_basic_list() {
    ocaml::runtime::init();
    let gc = unsafe { ocaml::Runtime::recover_handle() };
    let mut list = ocaml::List::empty();
    let a = 3i64.to_value(gc);
    let b = 2i64.to_value(gc);
    let c = 1i64.to_value(gc);

    unsafe {
        list = list.add(gc, &a);
        list = list.add(gc, &b);
        list = list.add(gc, &c);
    }

    unsafe {
        assert!(list.len() == 3);
    }

    let ll: std::collections::LinkedList<i64> = FromValue::from_value(list.to_value(gc));

    for (i, x) in ll.into_iter().enumerate() {
        assert!((i + 1) as i64 == x);
    }
}

fn test_int() {
    ocaml::runtime::init_persistent();
    let gc = unsafe { ocaml::Runtime::recover_handle() };
    let a = (-123isize).to_value(gc);
    let b = (-1isize).to_value(gc);
    let c = 123isize.to_value(gc);
    let d = 1isize.to_value(gc);
    let e = 0isize.to_value(gc);

    let a_: isize = FromValue::from_value(a);
    let b_: isize = FromValue::from_value(b);
    let c_: isize = FromValue::from_value(c);
    let d_: isize = FromValue::from_value(d);
    let e_: isize = FromValue::from_value(e);
    assert_eq!(a_, -123);
    assert_eq!(b_, -1);
    assert_eq!(c_, 123);
    assert_eq!(d_, 1);
    assert_eq!(e_, 0);
}

#[ocaml::func]
pub fn make_tuple(a: Value, b: Value) -> (Value, Value) {
    (a, b)
}

fn test_tuple_of_tuples() {
    ocaml::runtime::init_persistent();
    let gc = unsafe { ocaml::Runtime::recover_handle() };

    let x = (1f64, 2f64, 3f64, 4f64, 5f64, 6f64, 7f64, 8f64, 9f64).to_value(gc);
    let y = (9f64, 8f64, 7f64, 6f64, 5f64, 4f64, 3f64, 2f64, 1f64).to_value(gc);
    let ((a, b, c, d, e, f, g, h, i), (j, k, l, m, n, o, p, q, r)): (
        (f64, f64, f64, f64, f64, f64, f64, f64, f64),
        (f64, f64, f64, f64, f64, f64, f64, f64, f64),
    ) = unsafe { FromValue::from_value(Value::new(make_tuple(x.raw(), y.raw()))) };

    println!("a: {}, r: {}", a, r);
    assert!(a == r);
    assert!(b == q);
    assert!(c == p);
    assert!(d == o);
    assert!(e == n);
    assert!(f == m);
    assert!(g == l);
    assert!(h == k);
    assert!(i == j);
}

fn main() {
    test_basic_array().unwrap();
    test_basic_list();
    test_int();
    test_tuple_of_tuples();
}
