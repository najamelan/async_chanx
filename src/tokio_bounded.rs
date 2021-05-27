use
{
	crate::{ import::*, ChanErr },
	tokio::sync::mpsc::{ Sender, error::{ SendError }, OwnedPermit,  },
};


enum Inner<M>
{
	Future(Pin<Box<dyn Future<Output = Result<OwnedPermit<M>, SendError<()>>> + Send >> ),
	Permit(OwnedPermit<M>),
}


/// A wrapper around [`tokio::sync::mpsc::Sender`] that implements [`Sink`].
/// It will also return [`ChanErr`] like all the other wrappers in this crate.
//
pub struct TokioSender<I>
{
	inner : Option< Inner<I> >,
	sender: Sender<I>         ,
}


impl<I> TokioSender<I>
{
	/// Create a wrapper around [`tokio::sync::mpsc::Sender`] that implements [`Sink`].
	/// It will also return [`ChanErr`] like all the other wrappers in this crate.
	//
	pub fn new( sender: Sender<I> ) -> TokioSender<I>
	{
		Self
		{
			inner : None,
			sender
		}
	}

	/// Access the inner [`tokio::sync::mpsc::Sender`].
	//
	pub fn inner( &self ) -> &Sender<I>
	{
		&self.sender
	}

	/// Access the inner [`tokio::sync::mpsc::Sender`] mutably.
	//
	pub fn inner_mut( &mut self ) -> &mut Sender<I>
	{
		&mut self.sender
	}
}


impl<I: 'static + Send> Sink<I> for TokioSender<I>
{
	type Error = ChanErr<I>;


	/// When calling reserve_owned, we will store the future in the option so we can poll it
	/// to completion.
	//
	fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>
	{
		let inner = self.inner.take();

		let mut fut = match inner
		{
			None                       => Box::pin( self.sender.clone().reserve_owned() ) ,
			Some( Inner::Future(fut) ) => fut                                             ,
			Some( Inner::Permit(_)   ) => return Poll::Ready( Ok(()) )                    ,
		};


		match Pin::new(&mut fut).poll(cx)
		{
			Poll::Pending =>
			{
				self.inner = Some( Inner::Future(fut) );
				return Poll::Pending;
			}

			Poll::Ready(res) =>
			{
				match res
				{
					Ok(owned_permit) =>
					{
						self.inner = Some( Inner::Permit(owned_permit) );
						return Poll::Ready( Ok(()) );
					}

					Err(_) =>
					{
						return Poll::Ready( Err(ChanErr::Closed) )
					}
				}
			}
		}
	}


	fn start_send( mut self: Pin<&mut Self>, msg: I ) -> Result<(), Self::Error>
	{
		// We need to remove the permit.
		//
		let inner = self.inner.take();

		match inner
		{
			None                     |
			Some( Inner::Future(_) ) => panic!( "call `poll_ready` before start_send" ),

			Some( Inner::Permit(p) ) =>
			{
				let _ = p.send( msg );
				self.inner = None;
				Ok(())
			}
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


impl<I> Clone for TokioSender<I>
{
	fn clone(&self) -> Self
	{
		Self
		{
			inner: None,
			sender: self.sender.clone()
		}
	}
}





impl<I> fmt::Debug for TokioSender<I>
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		f.debug_struct("Sender")

			.field("sender", &self.sender )

		.finish()
	}
}
