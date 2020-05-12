// Tests  needed for all channels:
//
// - what size is the buffer really?
// - how does it relate to number of senders?
// - how does back pressure work, on flush? or on poll_ready?
// - wakeup one deadlock
// - thundering herd
// - See: https://github.com/rust-lang/futures-rs/issues/1312 and https://github.com/rust-lang/futures-rs/pull/984 for inspiration.


pub use super::*;

#[ derive( Debug, Clone, PartialEq, Eq ) ]
//
pub enum MessageCount<I>
{
	Ready( Result<usize, ChanErr<I>> ),
	Flush( Result<usize, ChanErr<I>> ),
	Send ( ChanErr<I>                ),
}



// pass in a tx and see how many messages we can pass into tx before it returns pending or error,
// without reading from rx.
//
pub fn sp_buffer_size( tx: &mut (impl Sink<u8, Error = ChanErr<u8>> + Unpin) ) -> MessageCount<u8>
{
	let     waker   = noop_waker()                  ;
	let mut cx      = Context::from_waker( &waker ) ;
	let mut tx      = Pin::new( tx )                ;
	let mut count   = 0                             ;

	loop
	{
		count += 1 ;

		match tx.as_mut().poll_ready( &mut cx )
		{
			Poll::Pending             => return MessageCount::Ready( Ok (count) ),
			Poll::Ready  ( Err(err) ) => return MessageCount::Ready( Err(err  ) ),
			Poll::Ready  ( Ok (() ) ) => {}
		}


		match tx.as_mut().start_send( 1 )
		{
			Err(err) => return MessageCount::Send( err ),
			Ok (() ) => {}
		}


		match tx.as_mut().poll_flush( &mut cx )
		{
			Poll::Pending             => return MessageCount::Flush( Ok (count) ),
			Poll::Ready  ( Err(err) ) => return MessageCount::Flush( Err(err  ) ),
			Poll::Ready  ( Ok (() ) ) => {}
		}
	}
}
