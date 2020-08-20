use
{
	crate::{ import::*, ChanErr },
	futures::channel::mpsc::{ UnboundedSender },
};


/// A wrapper around [`futures::channel::UnboundedSender`]. This type already
/// implements `Sink`, but we unify the error type to [`ChanErr`] like the other
/// wrappers in this crate.
//
#[ derive( Debug )]
//
pub struct FuturesUnboundedSender<I>
{
	inner: UnboundedSender<I>,
}


impl<I> FuturesUnboundedSender<I>
{
	/// Create a wrapper around [`tokio::sync::mpsc::Sender`] that implements [`Sink`].
	/// It will also return [`ChanErr`] like all the other wrappers in this crate.
	//
	pub fn new( inner: UnboundedSender<I> ) -> FuturesUnboundedSender<I>
	{
		Self{ inner }
	}

	/// Access the inner [`tokio::sync::mpsc::Sender`].
	//
	pub fn inner( &self ) -> &UnboundedSender<I>
	{
		&self.inner
	}

	/// Access the inner [`tokio::sync::mpsc::Sender`] mutably.
	//
	pub fn inner_mut( &mut self ) -> &mut UnboundedSender<I>
	{
		&mut self.inner
	}
}


impl<I> Sink<I> for FuturesUnboundedSender<I>
{
	type Error = ChanErr<I>;


	fn poll_ready( mut self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{
		Pin::new( &mut self.inner ).poll_ready( cx ).map_err( |_| ChanErr::Closed )
	}


	fn start_send( mut self: Pin<&mut Self>, msg: I ) -> Result<(), Self::Error>
	{
		self.as_mut().inner.start_send( msg ).map_err( |_| ChanErr::Closed )
	}


	fn poll_flush( mut self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{
		Pin::new( &mut self.inner ).poll_ready( cx ).map_err( |_| ChanErr::Closed )
	}


	fn poll_close( mut self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{
		Pin::new( &mut self.inner ).poll_ready( cx ).map_err( |_| ChanErr::Closed )
	}
}


impl<I> Clone for FuturesUnboundedSender<I>
{
	fn clone(&self) -> Self
	{
		Self{ inner: self.inner.clone() }
	}
}


impl<I> From<UnboundedSender<I>> for FuturesUnboundedSender<I>
{
	fn from( s: UnboundedSender<I> ) -> Self
	{
		Self{ inner: s }
	}
}
