use std::marker::PhantomData;

pub struct EventHandlers<F: Fn(&Args) -> R, Args, R> {
    _args: PhantomData<Args>,
    _r: PhantomData<R>,
    callbacks: Vec<F>,
}

impl<F, Args, R: Sized> EventHandlers<F, Args, R>
where
    F: Fn(&Args) -> R,
{
    pub fn invoke_callback(&self, args: Args) -> Vec<R> {
        self.callbacks
            .iter()
            .map(|callback| callback(&args))
            .collect()
    }

    pub fn register_callback(&mut self, callback: F) {
        self.callbacks.push(callback);
    }
}
