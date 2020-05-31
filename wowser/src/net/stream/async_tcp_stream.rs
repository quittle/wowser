use super::super::NETWORK_BUFFER_SIZE;
use std::net;
use std::{io::Read, task};

pub struct AsyncTcpStream {
    stream: net::TcpStream,
}

impl AsyncTcpStream {
    pub fn from_tcp_stream(stream: net::TcpStream) -> AsyncTcpStream {
        AsyncTcpStream { stream }
    }
}

impl futures::stream::Stream for AsyncTcpStream {
    type Item = Result<Vec<u8>, std::io::Error>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        _context: &mut futures::task::Context,
    ) -> task::Poll<Option<Self::Item>> {
        let read_amt = self.stream.peek(&mut [0u8; 1]);
        match read_amt {
            Ok(0) => task::Poll::Pending,
            Ok(_) => {
                let mut ret = [0u8; NETWORK_BUFFER_SIZE];
                match self.get_mut().stream.read(&mut ret) {
                    Ok(amt) => task::Poll::Ready(Some(Ok(ret[..amt].to_vec()))),
                    Err(e) => task::Poll::Ready(Some(Err(e))),
                }
            }
            Err(e) => task::Poll::Ready(Some(Err(e))),
        }
    }
}
