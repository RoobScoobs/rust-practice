/***
 * 
 * 
 * 
    USING THE CUSTOM DERIVE

    Specified the builder crate as dependency in the manifest
    and it's referenced by a simple relative path
  
    RUNNING CARGO EXPAND TO UNDERSTAND THE GENERATED CODE

    impl<T, U> Item<T, U>
    where
        T: Default,
    {
        fn builder() -> ItemBuilder<T, U> {
            ItemBuilder::new()
        }
    }

    impl<T, U> Default for ItemBuilder<T,U>
    where
        T: Default,
    {
        fn default() -> Self {
            ItemBuilder {
                a: None,
                b: None,
                c: None,
                d: None,
                e: None,
                f: None,
            }
        }
    }

    struct ItemBuilder<T, U>
    where
        T: Default,
    {
        a: Option<u32>,
        b: Option<Option<&'static str>>,
        c: Option<String>,
        d: Option<X>,
        e: Option<T>,
        f: Option<U>,
    }

    impl<T, U> ItemBuilder<T, U>
    where
        T: Default,
    {
        fn new() -> Self {
            Default::default()
        }

        fn a<__Builder_T: Into<u32>>(mut self, val: __Builder_T) -> Self {
            self.a = Some(val.into());
            self
        }

        fn b<__Builder_T: Into<Option<&'static str>>(mut self, val: __Builder_T) -> Self {
            self.b = Some(val.into());
            self
        }

        fn c<__Builder_T: Into<String>>(mut self, val: __Builder_T) -> Self {
            self.c = Some(val.into());
            self
        }

        fn d<__Builder_T: Into<X>>(mut self, val: __Builder_T) -> Self {
            self.d = Some(val.into());
            self
        }

        fn e<__Builder_T: Into<T>>(mut self, val: __Builder_T) -> Self {
            self.e = Some(val.into());
            self
        }

        fn f<__Builder_T: Into<U>>(mut self, val: __Builder_T) -> Self {
            self.f = Some(val.into());
            self
        }

        fn build(self) -> Item<T, U> {
            Item {
                a: self.a.unwrap_or_else(Default::default),
                b: self.b.unwrap_or_else(Default::default),
                c: self.c.unwrap_or_else(Default::default),
                d: self.d.unwrap(),
                e: self.e.unwrap_or_else(Default::default),
                f: self.f.unwrap(),
            }
        }
    }
***/

use builder::Builder;

#[derive(Debug)]
struct X {}

#[derive(Debug, Builder)]
struct Item<T, U>
where
    T: Default
{
    a: u32,
    b: Option<&'static str>,
    c: String,
    #[builder(required)]
    d: X,
    e: T,
    #[builder(required)]
    f: U,
}

fn main() {
    let item: Item<i32, &str> = Item::builder()
        .a(42u32)
        .b("hello")
        .c("boom".to_owned())
        .d(X {})
        .e(42i32)
        .f("hello")
        .build();

    println!("{:#?}", item);

    let item2 = Item::<u32, u64>::builder().b(None).d(X {}).f(99u64).build();

    println!("{:#?}", item2);
}
