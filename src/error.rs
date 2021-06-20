use crate::import::*;

/// Result which has a ChanErr as error type.
//
pub type ChanRes<T, I> = Result<T, ChanErr<I>>;


/// Errors.
//
#[ derive(Eq) ]
//
pub struct ChanErr<I>
{
	pub(crate) kind: ChanErrKind,
	pub(crate) item: Option<I>,
}

// We never allow access to the contained I by reference. The user can only
// move it out.
//
unsafe impl<I: Send> Sync for ChanErr<I> {}



impl<I: Send> ChanErr<I>
{
	/// Retrieve the item you tried to send if there is one.
	//
	pub fn item( &mut self ) -> Option<I>
	{
		self.item.take()
	}


	/// The kind or error that occurred.
	//
	pub fn kind( &mut self ) -> ChanErrKind
	{
		self.kind
	}


}



/// A unified error for async channels. When creating a trait object based on
/// `Sink`, you will also have to specify the error type. By creating a single
/// type here that raps the native errors from channels, we make this easier.
//
#[ derive( Copy, Clone, Debug, PartialEq, Eq ) ]
//
pub enum ChanErrKind
{
	/// The channel Sender is already closed.
	//
	Closed,
}





impl<I> std::error::Error for ChanErr<I>
{

}


impl<I> fmt::Debug for ChanErr<I>
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		match self.kind
		{
			ChanErrKind::Closed => write!( f, "The channel Sender is already closed." ) ,
		}
	}
}

impl<I> fmt::Display for ChanErr<I>
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		write!( f, "{}", self )
	}
}


impl<I> PartialEq for ChanErr<I>
{
	fn eq( &self, other: &ChanErr<I> ) -> bool
	{
		self.kind == other.kind
	}
}
