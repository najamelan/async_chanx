use
{
	crate   :: { import::*, ChanErr                             } ,
	piper   :: { Sender                                         } ,
	std     :: { task::{ Waker, Poll, Context }, future::Future } ,
	futures :: { FutureExt, ready                               } ,
};


/// A wrapper around piper::Sender that implements Sink.
//
pub struct PiperSender<I>
{
	sender: Option< Sender<I>                                         > ,
	buffer: Option< I                                                 > ,
	waker : Option< Waker                                             > ,
	send  : Option< Pin<Box< dyn Future<Output = Sender<I>> + Send >> > ,
}


impl<I: fmt::Debug> fmt::Debug for PiperSender<I>
{
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		fmt.debug_struct( "PiperSender" )

			.field( "sender", &self.sender )
			.field( "buffer", &self.buffer )
			.field( "waker" , &self.waker )
			.field( "send"  , &self.send.as_ref().map( |_| "future processing send" ) )

		.finish()
	}
}



impl<I> PiperSender<I>
{
	/// Constructor
	//
	pub fn new( sender: Sender<I> ) -> Self
	{
		Self
		{
			sender: Some(sender) ,
			buffer: None         ,
			waker : None         ,
			send  : None         ,
		}
	}
}



impl<I: 'static +  Send + Unpin> Sink<I> for PiperSender<I>
{
	type Error = ChanErr<I>;


	fn poll_ready( mut self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll< Result<(), Self::Error> >
	{
		if self.buffer.is_some()
		{
			self.waker = Some( cx.waker().clone() );
			return Poll::Pending;
		}

		Poll::Ready( Ok(()) )
	}


	fn start_send( mut self: Pin<&mut Self>, msg: I ) -> Result<(), Self::Error>
	{
		if self.buffer.is_some()
		{
			panic!( "call `poll_ready` before start_send" )
		}

		self.buffer = Some(msg);

		Ok(())
	}


	fn poll_flush( mut self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{
		if let Some(future) = self.send.as_mut()
		{
			self.sender = Some( ready!(future.as_mut().poll(cx)) );
			self.send   = None;
		}

		match self.buffer.take()
		{
			None => Poll::Ready( Ok(()) ),

			Some(msg) =>
			{
				// take the sender
				//
				let mut sender = self.as_mut().sender.take().unwrap();

				let send = async move
				{
					sender.send( msg ).await;
					sender

				}.boxed();

				// store it in the future
				//
				self.send = Some( send );

				// leave it that way until it's ready
				//
				sender = ready!( self.as_mut().send.as_mut().unwrap().as_mut().poll(cx) );

				// Put the sender back and drop the future.
				//
				self.sender = Some(sender);
				self.send   = None;

				Poll::Ready( Ok(()) )
			}
		}
	}


	fn poll_close( self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{
		self.poll_flush( cx )
	}
}



impl<I> Clone for PiperSender<I>
{
	fn clone(&self) -> Self
	{
		Self
		{
			sender: self.sender.clone() ,
			buffer: None                ,
			waker : None                ,
			send  : None                ,
		}
	}
}
