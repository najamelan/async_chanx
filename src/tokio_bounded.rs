use
{
	crate::{ import::*, ChanErr },
	tokio::sync::mpsc::{ Sender, error::{ TrySendError } },
};


#[ derive( Clone ) ]
//
pub struct TokioSender<I>
{
	inner: Sender<I>,
}


impl<I> Sink<I> for TokioSender<I>
{
	type Error = ChanErr<I>;


	fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>
	{
		Sender::poll_ready( &mut self.get_mut().inner, cx ).map_err(|_| ChanErr::Closed )
	}


	fn start_send( mut self: Pin<&mut Self>, msg: I ) -> Result<(), Self::Error>
	{
		match self.as_mut().inner.try_send( msg )
		{
			Err( TrySendError::Full  (_) ) => panic!( "call `poll_ready` before start_send" ),
			Err( TrySendError::Closed(t) ) => Err( ChanErr::ClosedI(t) ),
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





impl<I> fmt::Debug for TokioSender<I>
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		f.debug_struct("Sender")

			.field("inner", &self.inner )

		.finish()
	}
}
