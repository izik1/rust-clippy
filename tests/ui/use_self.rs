// run-rustfix

#![warn(clippy::use_self)]
#![allow(dead_code)]
#![allow(clippy::should_implement_trait)]

fn main() {}

mod use_self {
    struct Foo {}

    impl Foo {
        fn new() -> Foo {
            Foo {}
        }
        fn test() -> Foo {
            Foo::new()
        }
    }

    impl Default for Foo {
        fn default() -> Foo {
            Foo::new()
        }
    }
}

mod better {
    struct Foo {}

    impl Foo {
        fn new() -> Self {
            Self {}
        }
        fn test() -> Self {
            Self::new()
        }
    }

    impl Default for Foo {
        fn default() -> Self {
            Self::new()
        }
    }
}

mod lifetimes {
    struct Foo<'a> {
        foo_str: &'a str,
    }

    impl<'a> Foo<'a> {
        // Cannot use `Self` as return type, because the function is actually `fn foo<'b>(s: &'b str) ->
        // Foo<'b>`
        fn foo(s: &str) -> Foo {
            Foo { foo_str: s }
        }
        // cannot replace with `Self`, because that's `Foo<'a>`
        fn bar() -> Foo<'static> {
            Foo { foo_str: "foo" }
        }

        // FIXME: the lint does not handle lifetimed struct
        // `Self` should be applicable here
        fn clone(&self) -> Foo<'a> {
            Foo { foo_str: self.foo_str }
        }
    }
}

#[allow(clippy::boxed_local)]
mod traits {

    use std::ops::Mul;

    trait SelfTrait {
        fn refs(p1: &Self) -> &Self;
        fn ref_refs<'a>(p1: &'a &'a Self) -> &'a &'a Self;
        fn mut_refs(p1: &mut Self) -> &mut Self;
        fn nested(p1: Box<Self>, p2: (&u8, &Self));
        fn vals(r: Self) -> Self;
    }

    #[derive(Default)]
    struct Bad;

    impl SelfTrait for Bad {
        fn refs(p1: &Bad) -> &Bad {
            p1
        }

        fn ref_refs<'a>(p1: &'a &'a Bad) -> &'a &'a Bad {
            p1
        }

        fn mut_refs(p1: &mut Bad) -> &mut Bad {
            p1
        }

        fn nested(_p1: Box<Bad>, _p2: (&u8, &Bad)) {}

        fn vals(_: Bad) -> Bad {
            Bad::default()
        }
    }

    impl Mul for Bad {
        type Output = Bad;

        fn mul(self, rhs: Bad) -> Bad {
            rhs
        }
    }

    #[derive(Default)]
    struct Good;

    impl SelfTrait for Good {
        fn refs(p1: &Self) -> &Self {
            p1
        }

        fn ref_refs<'a>(p1: &'a &'a Self) -> &'a &'a Self {
            p1
        }

        fn mut_refs(p1: &mut Self) -> &mut Self {
            p1
        }

        fn nested(_p1: Box<Self>, _p2: (&u8, &Self)) {}

        fn vals(_: Self) -> Self {
            Self::default()
        }
    }

    impl Mul for Good {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self {
            rhs
        }
    }

    trait NameTrait {
        fn refs(p1: &u8) -> &u8;
        fn ref_refs<'a>(p1: &'a &'a u8) -> &'a &'a u8;
        fn mut_refs(p1: &mut u8) -> &mut u8;
        fn nested(p1: Box<u8>, p2: (&u8, &u8));
        fn vals(p1: u8) -> u8;
    }

    // Using `Self` instead of the type name is OK
    impl NameTrait for u8 {
        fn refs(p1: &Self) -> &Self {
            p1
        }

        fn ref_refs<'a>(p1: &'a &'a Self) -> &'a &'a Self {
            p1
        }

        fn mut_refs(p1: &mut Self) -> &mut Self {
            p1
        }

        fn nested(_p1: Box<Self>, _p2: (&Self, &Self)) {}

        fn vals(_: Self) -> Self {
            Self::default()
        }
    }

    // Check that self arg isn't linted
    impl Clone for Good {
        fn clone(&self) -> Self {
            // Note: Not linted and it wouldn't be valid
            // because "can't use `Self` as a constructor`"
            Good
        }
    }
}

mod issue2894 {
    trait IntoBytes {
        fn into_bytes(&self) -> Vec<u8>;
    }

    // This should not be linted
    impl IntoBytes for u8 {
        fn into_bytes(&self) -> Vec<u8> {
            vec![*self]
        }
    }
}

mod existential {
    struct Foo;

    impl Foo {
        fn bad(foos: &[Self]) -> impl Iterator<Item = &Foo> {
            foos.iter()
        }

        fn good(foos: &[Self]) -> impl Iterator<Item = &Self> {
            foos.iter()
        }
    }
}

mod tuple_structs {
    pub struct TS(i32);

    impl TS {
        pub fn ts() -> Self {
            TS(0)
        }
    }
}

mod macros {
    macro_rules! use_self_expand {
        () => {
            fn new() -> Foo {
                Foo {}
            }
        };
    }

    struct Foo {}

    impl Foo {
        use_self_expand!(); // Should lint in local macros
    }
}

mod nesting {
    struct Foo {}
    impl Foo {
        fn foo() {
            #[allow(unused_imports)]
            use self::Foo; // Can't use Self here
            struct Bar {
                foo: Foo, // Foo != Self
            }

            impl Bar {
                fn bar() -> Bar {
                    Bar { foo: Foo {} }
                }
            }

            // Can't use Self here
            fn baz() -> Foo {
                Foo {}
            }
        }

        // Should lint here
        fn baz() -> Foo {
            Foo {}
        }
    }

    enum Enum {
        A,
        B(u64),
        C { field: bool }
    }
    impl Enum {
        fn method() {
            #[allow(unused_imports)]
            use self::Enum::*; // Issue 3425
            static STATIC: Enum = Enum::A; // Can't use Self as type
        }

        fn method2() {
            let _ = Enum::B(42);
            let _ = Enum::C { field: true };
            let _ = Enum::A;
        }
    }
}

mod issue3410 {

    struct A;
    struct B;

    trait Trait<T> {
        fn a(v: T);
    }

    impl Trait<Vec<A>> for Vec<B> {
        fn a(_: Vec<A>) {}
    }
}

#[allow(clippy::no_effect, path_statements)]
mod rustfix {
    mod nested {
        pub struct A {}
    }

    impl nested::A {
        const A: bool = true;

        fn fun_1() {}

        fn fun_2() {
            nested::A::fun_1();
            nested::A::A;

            nested::A {};
        }
    }
}
