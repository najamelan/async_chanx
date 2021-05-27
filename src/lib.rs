// See: https://github.com/rust-lang/rust/issues/44732#issuecomment-488766871
//
#![cfg_attr( nightly, feature(doc_cfg, external_doc) )]
#![cfg_attr( nightly, doc(include = "../README.md")  )]
#![doc = ""] // empty doc line to handle missing doc warning when the feature is missing.

#![ doc    ( html_root_url = "https://docs.rs/async_chanx" ) ]
#![ forbid ( unsafe_code                                      ) ]
#![ allow  ( clippy::suspicious_else_formatting               ) ]

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
mod tokio_bounded;
mod tokio_unbounded;
mod futures_unbounded;
mod error;

pub use
{
	// crate::async_channel::*,
	// crate::crossfire::*,
	tokio_bounded::*,
	tokio_unbounded::*,
	futures_unbounded::*,
	error::*,
};


// External dependencies
//
mod import
{
	pub(crate) use
	{
		std :: { fmt, task::{ Context, Poll }, pin::Pin, future::Future } ,
		futures_sink:: { Sink } ,
	};
}


