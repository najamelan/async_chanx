// Tested:
// - Backpressure occurs at exactly bound.
// - Receiver get woken up if all senders are dropped.

use
{
	async_chanx::tokio::mpsc,
};


mod common;

pub use common::*;


/// Backpressure occurs at exactly bound.
//
#[ test ]
//
fn tokio_bounded_sp_buffer_size()
{
	let (mut tx, _rx) = mpsc::channel( 3 );

	crate::assert_matches!( sp_buffer_size( &mut tx ), MessageCount::Ready( Ok(4) ) );
}


/// Receiver get woken up if all senders are dropped.
//
#[ test ]
//
fn tokio_bounded_receiver_woken_when_senders_dropped()
{
	let (start_tx, start_rx) = oneshot::channel();
	let (end_tx  , end_rx  ) = oneshot::channel();

	let (mut tx, mut rx) = mpsc::channel::<()>( 3 );

	let sender_thread = std::thread::spawn( move ||
	{
		block_on( async move
		{
			start_rx.await.expect( "wait start_rx" );

			tx.close().await.expect( "close tx" );
			drop(tx);

			end_tx.send(()).expect( "oneshot send" );
		});
	});


	block_on( async move
	{
		let (waker, count) = futures_test::task::new_count_waker();
		let mut cx         = Context::from_waker( &waker );

		crate::assert_matches!( Pin::new( &mut rx ).poll_next( &mut cx ), Poll::Pending );

		assert_eq!( count, 0 );

		start_tx.send(()).expect( "oneshot send" );
		end_rx.await.expect( "wait start_rx" );

		assert_eq!( count, 1 );
	});

	sender_thread.join().expect( "join thread" );
}





