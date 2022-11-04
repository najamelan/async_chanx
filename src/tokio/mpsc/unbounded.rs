use
{
	crate::{ import::*, ChanErr, ChanErrKind },
	tokio_crate::sync::mpsc::{ UnboundedSender as TokioSender, error::{ SendError } },
};


/// A wrapper around [`tokio::sync::mpsc::Sender`] that implements [`Sink`].
/// It will also return [`ChanErr`] like all the other wrappers in this crate.
//
pub struct UnboundedSender<I>
{
	inner: TokioSender<I>,
}


impl<I> UnboundedSender<I>
{
	/// Create a wrapper around [`tokio::sync::mpsc::Sender`] that implements [`Sink`].
	/// It will also return [`ChanErr`] like all the other wrappers in this crate.
	//
	pub fn new( inner: TokioSender<I> ) -> UnboundedSender<I>
	{
		Self{ inner }
	}

	/// Access the inner [`tokio::sync::mpsc::Sender`].
	//
	pub fn inner( &self ) -> &TokioSender<I>
	{
		&self.inner
	}

	/// Access the inner [`tokio::sync::mpsc::Sender`] mutably.
	//
	pub fn inner_mut( &mut self ) -> &mut TokioSender<I>
	{
		&mut self.inner
	}
}


impl<I> Sink<I> for UnboundedSender<I>
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
			Err( SendError(t) ) => Err
			(
				ChanErr
				{
					kind: ChanErrKind::Closed,
					item: Some(t),
				}
			),

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


impl<I> Clone for UnboundedSender<I>
{
	fn clone(&self) -> Self
	{
		Self{ inner: self.inner.clone() }
	}
}



impl<I> fmt::Debug for UnboundedSender<I>
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		f.debug_struct( "UnboundedSender" )

			.field("inner", &self.inner )

		.finish()
	}
}




impl<I> From<TokioSender<I>> for UnboundedSender<I>
{
	fn from( from: TokioSender<I> ) -> Self
	{
		Self::new(from)
	}
}
