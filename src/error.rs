use crate::import::*;

/// Result which has a ChanErr as error type.
//
pub type ChanRes<T, I> = Result<T, ChanErr<I>>;

/// Errors that can happen in thespis_impl.
//
#[ derive( Clone, PartialEq, Eq ) ]
//
pub enum ChanErr<I>
{
	/// The channel Sender is already closed.
	//
	Closed,


	/// The channel Sender is already closed. The item you tried to send is returned in the error.
	//
	ClosedI
	(
		/// The item you tried to send.
		//
		I
	),
}


impl<I> std::error::Error for ChanErr<I>
{

}


impl<I> fmt::Debug for ChanErr<I>
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		match *self
		{
			ChanErr::Closed     => write!( f, "The channel Sender is already closed." ) ,
			ChanErr::ClosedI(_) => write!( f, "The channel Sender is already closed." ) ,
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
