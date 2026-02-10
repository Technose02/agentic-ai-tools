pub mod model;
pub mod port;
pub mod service;

// TODO: Make Inports generic over Input and Output types as well as the error-type (all associated types)
// implementing error should not depend an AdkError, String is a better target
// make language a general marker type (e.g. "struct DE;") to keep it consistent through layers (add it to generic functions, traits etc.)

pub trait DomainService {
    type Params;
    type Result;
    type Error;

    fn invoke(
        &self,
        params: Self::Params,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = std::result::Result<Self::Result, Self::Error>> + Send,
        >,
    >;
}

pub type PinBoxedFuture<R, E> =
    std::pin::Pin<Box<dyn std::future::Future<Output = std::result::Result<R, E>> + Send>>;
