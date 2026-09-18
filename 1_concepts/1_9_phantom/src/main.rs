use std::marker::PhantomData;

struct Fact<T> (PhantomData<T>);

trait Facts {
    fn fact() -> &'static str;
}

impl<T> Facts for Vec<T> {
    fn fact() -> &'static str {
        "Vec is heap-allocated."
    }
}

impl<T> Fact<T>
where T: Facts
{
    fn new() -> Self {
        Self(PhantomData)
    }

    fn fact(&self) -> &'static str {
        T::fact()
    }
}

fn main() {
    let f: Fact<Vec<u8>> = Fact::new();
    println!("Fact about Vec: {}", f.fact());
    println!("Fact about Vec: {}", f.fact());
}
