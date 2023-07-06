use std::fmt::Debug;

trait SomeTrait {
    type Item: ?Sized;
    fn process(&mut self) -> &Self::Item {
        self.result()
    }

    fn result(&self) -> &Self::Item;
}

#[derive(Debug)]
struct Custom {
    data: [u8; 10],
}

impl SomeTrait for Custom {
    type Item = [u8];

    fn result(&self) -> &Self::Item {
        &self.data
    }
}

fn main() {
    let mut arg = Custom { data: [0; 10] };
    func1(&mut arg);
    func2(arg);
}

fn func1<T>(arg: &mut T)
where
    T: SomeTrait,
    for<'a> &'a T::Item: Debug,
{
    println!("{:?}", arg.process());
}

fn func2<T>(mut arg: T)
where
    T: SomeTrait,
    for<'a> &'a T::Item: Debug,
{
    println!("{:?}", arg.process());
}
