use crate::{Decoder, Encoder};
use bytes::{Bytes, BytesMut};
use std::io::Error;

/// A simple codec that ships bytes around
///
/// # Example
///
///  ```
/// #![feature(async_await, await_macro)]
/// use bytes::Bytes;
/// use futures::{SinkExt, TryStreamExt};
/// use std::io::Cursor;
/// use futures_codec::{BytesCodec, Framed};
///
/// async move {
///     let mut buf = vec![];
///     // Cursor implements AsyncRead and AsyncWrite
///     let cur = Cursor::new(&mut buf);
///     let mut framed = Framed::new(cur, BytesCodec {});
///
///     let msg = Bytes::from("Hello World!");
///     await!(framed.send(msg)).unwrap();
///
///     while let Some(msg) = await!(framed.try_next()).unwrap() {
///         println!("{:?}", msg);
///     }
/// };
/// ```
pub struct BytesCodec {}

impl Decoder for BytesCodec {
    type Item = Bytes;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let len = src.len();
        if len > 0 {
            Ok(Some(src.split_to(len).freeze()))
        } else {
            Ok(None)
        }
    }
}

impl Encoder for BytesCodec {
    type Item = Bytes;
    type Error = Error;

    fn encode(&mut self, src: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.extend_from_slice(&src);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::BytesCodec;
    use crate::Framed;

    use futures::{executor, TryStreamExt};
    use std::io::Cursor;
    #[test]
    fn decodes() {
        let mut buf = [0u8; 32];
        let expected = buf.clone();
        let cur = Cursor::new(&mut buf);
        let mut framed = Framed::new(cur, BytesCodec {});

        let read = executor::block_on(framed.try_next()).unwrap().unwrap();
        assert_eq!(&read[..], &expected[..]);

        assert!(executor::block_on(framed.try_next()).unwrap().is_none());
    }
}