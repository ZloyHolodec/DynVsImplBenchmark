use std::time;

const ENTITIES_AMOUNT: usize = 1_000_000_000;

trait Foo {
    fn do_bar(&self, x: i32, y: i32) -> i32;
}

struct SummarizeFoo;
struct MultiplyAndAddConstant {
    val: i32,
}

#[derive(Default)]
struct ResultContainer {
    result: i32,
}

impl MultiplyAndAddConstant {
    fn new(val: i32) -> Self {
        return Self { val: val };
    }
}

impl Foo for SummarizeFoo {
    fn do_bar(&self, x: i32, y: i32) -> i32 {
        return x + y;
    }
}

impl Foo for MultiplyAndAddConstant {
    fn do_bar(&self, x: i32, y: i32) -> i32 {
        return (x * y) + self.val;
    }
}

impl ResultContainer {
    fn do_dyn(&mut self, foo: &dyn Foo, x: i32, y: i32) {
        self.result += foo.do_bar(x, y);
    }

    fn do_impl(&mut self, foo: &impl Foo, x: i32, y: i32) {
        self.result += foo.do_bar(x, y);
    }
}

fn do_dyn() -> i32 {
    let mut result = ResultContainer::default();

    for i in 0..ENTITIES_AMOUNT {
        let i: i32 = i.try_into().unwrap();
        let x = i;
        let y = i + 100;
        match i % 2 {
            0 => result.do_dyn(&SummarizeFoo {}, x, y),
            1 => result.do_dyn(&MultiplyAndAddConstant::new(x / 10), x, y),
            _ => {}
        }
    }

    return result.result;
}

fn do_impl() -> i32 {
    let mut result = ResultContainer::default();

    for i in 0..ENTITIES_AMOUNT {
        let i: i32 = i.try_into().unwrap();
        let x = i;
        let y = i + 100;
        match i % 2 {
            0 => result.do_impl(&SummarizeFoo {}, x, y),
            1 => result.do_impl(&MultiplyAndAddConstant::new(x / 10), x, y),
            _ => {}
        }
    }

    return result.result;
}

fn main() {
    let checkpoint = time::Instant::now();
    let dyn_result = do_dyn();
    let dyn_time = checkpoint.elapsed().as_millis();

    let checkpoint = time::Instant::now();
    let impl_result = do_impl();
    let impl_time = checkpoint.elapsed().as_millis();

    println!("Impl time: {}", impl_time);
    println!("Impl result: {}", impl_result);
    println!("Dyn time: {}", dyn_time);
    println!("Dyn result: {}", dyn_result);
}
