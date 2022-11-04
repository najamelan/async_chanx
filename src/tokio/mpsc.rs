use tokio_stream::wrappers::{ ReceiverStream, UnboundedReceiverStream };

mod bounded;
mod unbounded;

pub use bounded::Sender;
pub use unbounded::UnboundedSender;

/// Get a bounded tokio mpsc channel that implements `futures::Sink` on `tx` and
/// `futures::Stream` on `rx`.
//
pub fn channel<T>(buffer: usize) -> (Sender<T>, ReceiverStream<T>)
{
	let (tx, rx) = tokio_crate::sync::mpsc::channel(buffer);

	(tx.into(), ReceiverStream::new(rx))
}

/// Get a bounded tokio unbounded channel that implements `futures::Sink` on `tx` and
/// `futures::Stream` on `rx`.
//
pub fn unbounded_channel<T>() -> (UnboundedSender<T>, UnboundedReceiverStream<T>)
{
	let (tx, rx) = tokio_crate::sync::mpsc::unbounded_channel();

	(tx.into(), UnboundedReceiverStream::new(rx))
}
