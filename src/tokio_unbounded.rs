use
{
	crate::{ import::*, ChanErr },
	tokio::sync::mpsc::{ UnboundedSender, error::{ SendError } },
};


/// A wrapper around [`tokio::sync::mpsc::Sender`] that implements [`Sink`].
/// It will also return [`ChanErr`] like all the other wrappers in this crate.
//
pub struct TokioUnboundedSender<I>
{
	inner: UnboundedSender<I>,
}


impl<I> TokioUnboundedSender<I>
{
	/// Create a wrapper around [`tokio::sync::mpsc::Sender`] that implements [`Sink`].
	/// It will also return [`ChanErr`] like all the other wrappers in this crate.
	//
	pub fn new( inner: UnboundedSender<I> ) -> TokioUnboundedSender<I>
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


impl<I> Sink<I> for TokioUnboundedSender<I>
{
	type Error = ChanErr<I>;


	fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>
	{
		Poll::Ready( Ok(()) )
	}


	fn start_send( mut self: Pin<&mut Self>, msg: I ) -> Result<(), Self::Error>
	{
		match self.as_mut().inner.send( msg )
		{
			Err( SendError(t) ) => Err( ChanErr::ClosedI(t) ),
			Ok(_) => Ok(()),
		}
	}


	fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>
	{
		Poll::Ready(Ok(()))
	}


	fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>
	{
		Poll::Ready(Ok(()))
	}
}


impl<I> Clone for TokioUnboundedSender<I>
{
	fn clone(&self) -> Self
	{
		Self{ inner: self.inner.clone() }
	}
}





impl<I> fmt::Debug for TokioUnboundedSender<I>
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		f.debug_struct( "TokioUnboundedSender" )

			.field("inner", &self.inner )

		.finish()
	}
}
