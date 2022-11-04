#![ cfg_attr( nightly, feature(doc_cfg) ) ]
#![ doc = include_str!("../README.md") ]

#![ doc    ( html_root_url = "https://docs.rs/async_chanx" ) ]
#![ allow  ( clippy::suspicious_else_formatting            ) ]

#![ warn
(
	anonymous_parameters          ,
	missing_copy_implementations  ,
	missing_debug_implementations ,
	missing_docs                  ,
	nonstandard_style             ,
	rust_2018_idioms              ,
	single_use_lifetimes          ,
	trivial_casts                 ,
	trivial_numeric_casts         ,
	unreachable_pub               ,
	unused_extern_crates          ,
	unused_qualifications         ,
	variant_size_differences      ,
)]


// mod async_channel;
// mod crossfire;
// mod futures_unbounded;
mod error;

pub use
{
	// crate::async_channel::*,
	// crate::crossfire::*,
	// futures_unbounded::*,
	error::*,
};


// External dependencies
//
mod import
{
	pub(crate) use
	{
		std ::fmt
	};

	#[cfg(feature="tokio")]
	pub(crate) use
	{
		std :: { task::{ Context, Poll }, pin::Pin, future::Future } ,
		futures_sink:: { Sink } ,
	};
}


/// Wrappers around `tokio::sync` channels.
//
#[cfg( feature="tokio" )]
//
pub mod tokio
{
	/// Wrappers around `tokio::sync::mpsc` channels.
	//
	pub mod mpsc;
}

