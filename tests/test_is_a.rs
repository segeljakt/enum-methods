#![allow(non_snake_case)]
#[macro_use]
extern crate enum_methods;

#[test]
fn test_is_a() {
    #[derive(EnumIsA, Debug)]
    enum MyEnum {
        Foo,
        Bar(bool, i32),
        Baz(String),
        StructType {
            foo: i32,
        },
        BiggerStructType {
            foo: i32,
            bar: &'static str,
            baz: bool,
        },
    }

    let first = MyEnum::Foo;
    let second = MyEnum::Bar(false, -3);
    let third = MyEnum::Baz("it's gonna take some time to do the things we never had".to_string());
    let fourth = MyEnum::StructType { foo: 42 };
    let fifth = MyEnum::BiggerStructType {
        foo: 42,
        bar: "I hear the drums echoing tonight",
        baz: true,
    };

    assert!(first.is_Foo());
    assert!(second.is_Bar());
    assert!(third.is_Baz());
    assert!(fourth.is_StructType());
    assert!(fifth.is_BiggerStructType());
}

#[test]
fn test_is_a_names() {
    #[derive(EnumIsA, Debug)]
    enum MyEnum {
        FooBar,
        BarBaz(bool, i32),
        Baz(String),
    }

    let first = MyEnum::FooBar;
    let second = MyEnum::BarBaz(false, -3);
    let third = MyEnum::Baz("cheers only whispers of some quiet conversation".to_string());

    assert!(first.is_FooBar());
    assert!(second.is_BarBaz());
    assert!(third.is_Baz());
}
