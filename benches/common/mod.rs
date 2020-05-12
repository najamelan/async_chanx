pub use
{
	// async_std :: { } ,

	std             :: { task::{ Context, Poll }, pin::Pin, fmt, future::Future } ,
	futures         :: { Stream, Sink, task::{ noop_waker, LocalSpawnExt }, sink::SinkExt, stream::StreamExt, future::join, executor::{ LocalPool, block_on }, channel::oneshot } ,
	async_chanx     :: { * } ,
	assert_matches  :: { * } ,
	async_executors :: { * } ,
	tokio           :: { sync::mpsc } ,
	criterion       :: { Criterion, criterion_group, criterion_main, BatchSize, Bencher } ,
	influmenza      :: { * },
};

const MSG_LEN: usize = 1;


#[derive(Clone, Copy)]
//
pub struct Msg( pub [usize; MSG_LEN] );

impl Msg
{
	pub fn new( num: usize ) -> Msg
	{
		Msg( [num; MSG_LEN] )
	}
}


impl fmt::Debug for Msg
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		f.pad( "Msg" )
	}
}





pub fn get_exec<T: Send + 'static>() -> impl SpawnHandle<T>
{
	AsyncStd::default()
}




