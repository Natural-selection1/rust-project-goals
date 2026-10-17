fn callback_print_any_lifetime_str<'a, 'b, 'c, F>(
    hello: &'a str,
    world: &'b str,
    rust: &'c str,
    f: F,
) where
    for<'any> F: Fn(&'any str),
{
    f(hello);
    f(world);
    f(rust);
}

fn callback_print_concrete_lifetime_str<'a, 'b, 'c, F>(
    hello: &'a str,
    world: &'b str,
    rust: &'c str,
    f: F,
) where
    'b: 'a,
    F: Fn(&'a str),
{
    f(hello);
    f(world);
    // f(rust);
}

fn main() {}

struct Foo;
struct Bar;

fn u(bar: &Bar, foo: &Foo) {}

trait TraitWithAsync {
    fn fn_return_send_future(&self, b: &Foo) -> impl Future<Output = ()> + Send;
}

struct Baz;

impl Baz {
    async fn b<F>(&self, mut f: F)
    where
        F: AsyncFnMut(&mut Bar),
    {
        let mut c = Bar;
        f(&mut c).await;
        f(&mut c).await;
    }
}

// impl TraitWithAsync for Baz {
//     async fn fn_return_send_future(&self, b: &Foo) {
//         self.b(async |c| u(c, b)).await;
//     }
// }
