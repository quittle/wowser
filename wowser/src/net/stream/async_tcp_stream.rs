use super::super::NETWORK_BUFFER_SIZE;
use std::io::ErrorKind;
use std::net;
use std::{io::Read, task};

pub struct AsyncTcpStream {
    stream: net::TcpStream,
}

impl AsyncTcpStream {
    pub fn from_tcp_stream(stream: net::TcpStream) -> AsyncTcpStream {
        stream.set_nonblocking(true).unwrap();
        AsyncTcpStream { stream }
    }
}

impl futures::stream::Stream for AsyncTcpStream {
    type Item = Result<Vec<u8>, std::io::Error>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        context: &mut futures::task::Context,
    ) -> task::Poll<Option<Self::Item>> {
        let mut ret = [0_u8; NETWORK_BUFFER_SIZE];

        // Note that this stream should have been configured as non-blocking during construction.
        let read_result = self.get_mut().stream.read(&mut ret);
        match read_result {
            // Returning 0 indicates the stream is closed
            Ok(0) => task::Poll::Ready(None),
            // Returned the amount of bytes read
            Ok(amt) => task::Poll::Ready(Some(Ok(ret[..amt].to_vec()))),
            Err(e) => match e.kind() {
                // This error is okay and expected in non-blocking mode
                ErrorKind::WouldBlock => {
                    // Given there is no callback, ensure the future can be polled immediately.
                    context.waker().wake_by_ref();

                    task::Poll::Pending
                }
                // An unexpected error indicates a transport error and assumed to be unrecoverable.
                _ => task::Poll::Ready(Some(Err(e))),
            },
        }
    }
}
