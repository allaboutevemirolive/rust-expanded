struct Inner<'a> {
    data: &'a str,
}

struct Outer<'a> {
    inner: Inner<'a>,
}

fn process<'a, 'b>(outer: &'a Outer<'b>) -> &'b str
where
    'a: 'b,
{
    &outer.inner.data[..]
}

fn main() {

    let result;
    {
        let another_data = "Another data";
        let another_inner = Inner { data: &another_data };
        let another_outer = Outer { inner: another_inner };
        result = process(&another_outer);
    }

    println!("Result: {}", result);
}
