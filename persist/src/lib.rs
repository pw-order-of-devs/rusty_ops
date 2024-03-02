pub mod mongo;

#[allow(opaque_hidden_inferred_bound)]
pub trait Persistence {

    fn build() -> impl std::future::Future<Output=Self> + Send
        where Self: Send;
}
