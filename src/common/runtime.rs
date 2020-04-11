use async_trait::async_trait;
use futures::{AsyncRead, AsyncWrite};
use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::time::Duration;

/// Common trait that various runtimes should implement.
#[async_trait]
pub trait Runtime {
    /// Runtime's TCP stream type.
    type TcpStream: AsyncRead + AsyncWrite + Unpin + Send + 'static;

    fn spawn<F: Future<Output = ()> + Send + 'static>(&self, fut: F);
    async fn sleep(&self, duration: Duration);
    async fn connect_tcp(&self, target: String) -> io::Result<Self::TcpStream>;
    async fn resolve(&self, target: String) -> io::Result<Vec<SocketAddr>>;
}

#[cfg(feature = "async-std-support")]
pub mod async_std_support {
    use super::*;

    /// async-std runtime shim
    pub struct AsyncStdRuntime;

    #[async_trait]
    impl Runtime for AsyncStdRuntime {
        type TcpStream = async_std::net::TcpStream;

        fn spawn<F: Future<Output = ()> + Send + 'static>(&self, fut: F) {
            async_std::task::spawn(fut);
        }
        async fn sleep(&self, duration: Duration) {
            async_std::task::sleep(duration).await
        }
        async fn connect_tcp(&self, target: String) -> io::Result<Self::TcpStream> {
            Ok(async_std::net::TcpStream::connect(target).await?)
        }
        async fn resolve(&self, target: String) -> io::Result<Vec<SocketAddr>> {
            Ok(async_std::net::ToSocketAddrs::to_socket_addrs(&target)
                .await?
                .collect())
        }
    }
}

#[cfg(feature = "tokio-support")]
pub mod tokio_support {
    use super::*;
    use tokio_util::compat::*;

    /// Tokio runtime shim.
    pub struct TokioRuntime;

    #[async_trait]
    impl Runtime for TokioRuntime {
        type TcpStream = tokio_util::compat::Compat<tokio::net::TcpStream>;

        fn spawn<F: Future<Output = ()> + Send + 'static>(&self, fut: F) {
            tokio::spawn(fut);
        }
        async fn sleep(&self, duration: Duration) {
            tokio::time::delay_for(duration).await
        }
        async fn connect_tcp(&self, target: String) -> io::Result<Self::TcpStream> {
            Ok(tokio::net::TcpStream::connect(target).await?.compat_write())
        }
        async fn resolve(&self, target: String) -> io::Result<Vec<SocketAddr>> {
            Ok(tokio::net::lookup_host(target).await?.collect())
        }
    }
}
