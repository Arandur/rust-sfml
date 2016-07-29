extern crate sfml;
use self::sfml::system::{Vector2i, Vector2u, Vector2f};

#[test]
fn vector2i_ops() {
    let vec = Vector2i::new(6, 20);

    assert_eq!(vec + 5, Vector2i::new(11, 25));
    assert_eq!(5 + vec, Vector2i::new(11, 25));
    assert_eq!(vec - 5, Vector2i::new(1, 15));
    assert_eq!(5 - vec, Vector2i::new(-1, -15));
    assert_eq!(vec * 2, Vector2i::new(12, 40));
    assert_eq!(2 * vec, Vector2i::new(12, 40));
    assert_eq!(vec / 2, Vector2i::new(3, 10));
    assert_eq!(60 / vec, Vector2i::new(10, 3));
}

#[test]
fn vector2u_ops() {
    let vec = Vector2u::new(6, 20);

    assert_eq!(vec + 5, Vector2u::new(11, 25));
    assert_eq!(5 + vec, Vector2u::new(11, 25));
    assert_eq!(vec - 5, Vector2u::new(1, 15));
    assert_eq!(20 - vec, Vector2u::new(14, 0));
    assert_eq!(vec * 2, Vector2u::new(12, 40));
    assert_eq!(2 * vec, Vector2u::new(12, 40));
    assert_eq!(vec / 2, Vector2u::new(3, 10));
    assert_eq!(60 / vec, Vector2u::new(10, 3));
}

#[test]
fn vector2f_ops() {
    let vec = Vector2f::new(6.0, 20.0);

    assert_eq!(vec + 5.0, Vector2f::new(11.0, 25.0));
    assert_eq!(5.0 + vec, Vector2f::new(11.0, 25.0));
    assert_eq!(vec - 5.0, Vector2f::new(1.0, 15.0));
    assert_eq!(5.0 - vec, Vector2f::new(-1.0, -15.0));
    assert_eq!(vec * 2.0, Vector2f::new(12.0, 40.0));
    assert_eq!(2.0 * vec, Vector2f::new(12.0, 40.0));
    assert_eq!(vec / 2.0, Vector2f::new(3.0, 10.0));
    assert_eq!(60.0 / vec, Vector2f::new(10.0, 3.0));
}

