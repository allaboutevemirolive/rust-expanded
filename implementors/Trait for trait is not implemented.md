# Trait for trait is not implemented

```rust
use std::error::Error;

pub trait Backend: Send + Sync {}

pub struct BackendBuilder {
    pub buf: Vec<u8>,
}

pub trait BackendBuilderFor<'a, T: Backend + 'a>: Sized {
    fn build(self) -> Result<T, Box<dyn Error>>;
}

pub trait BoxedBackendBuilderFor<'a, T: Backend + 'a>: BackendBuilderFor<'a, T> {
    fn build_boxed(self) -> Result<Box<dyn Backend + 'a>, Box<dyn Error>> {
        Ok(Box::new(self.build()?))
    }
}

impl<'a, T: Backend + 'a, O: BackendBuilderFor<'a, T>> BoxedBackendBuilderFor<'a, T> for O {}

mod backend_a {
    pub struct A {}
    impl super::Backend for A {}
    impl<'a> super::BackendBuilderFor<'a, A> for super::BackendBuilder {
        fn build(self) -> Result<A, Box<dyn super::Error>> {
            Ok(A {})
        }
    }
}

mod backend_b {
    pub struct B {}
    impl super::Backend for B {}
    impl<'a> super::BackendBuilderFor<'a, B> for super::BackendBuilder {
        fn build(self) -> Result<B, Box<dyn super::Error>> {
            Ok(B {})
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let builder = BackendBuilder { buf: vec![1, 2, 3] };

    let _backend: Box<dyn Backend> = BoxedBackendBuilderFor::<backend_a::A>::build_boxed(builder)?;

    Ok(())
}
```