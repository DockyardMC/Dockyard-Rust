use std::marker::PhantomData;

pub struct AdvancedJavaExecutor<'a, T> {
    pub marker: PhantomData<&'a T>,
}

impl<'a, T> AdvancedJavaExecutor<'a, T> {
    pub(crate) fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}