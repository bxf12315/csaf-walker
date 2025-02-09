//! Walking through CSAF documents
//!
//! ## Idea
//!
//! The basic idea is to provide a mechanism to walk over documents from differences sources
//! ([`csaf_walker::source::HttpSource`] or [`csaf_walker::source::FileSource`]). Then
//! chaining visitors in a layered fashion depending on your use case, extending the information
//! known about a CSAF document. That doesn't mean to actually parse the document, but the ensure
//! things like integrity, by digests and signatures.
//!
//! The stack allows one to customize the walking process, like skipping existing documents, or
//! processing only changed documents.
//!
//! The last step, most likely, is to do something with a discovered document (like storing,
//! uploading, evaluating). This is up to user to implement this. However, for some common use
//! cases, the [`csaf_cli`](https://crates.io/crates/csaf-cli) crate might have some
//! out-of-the-box tooling for the command line.
//!
//! ## Example
//!
//! A simple example for iterating over a source of CSAF documents:
//!
//! ```rust
//! use anyhow::Result;
//! use url::Url;
//! use csaf_walker::source::HttpSource;
//! use csaf_walker::walker::Walker;
//! use csaf_walker::retrieve::RetrievingVisitor;
//! use csaf_walker::validation::{ValidatedAdvisory, ValidationError, ValidationVisitor};
//! use walker_common::fetcher::Fetcher;
//!
//! async fn walk() -> Result<()> {
//!   let fetcher = Fetcher::new(Default::default()).await?;
//!   let source = HttpSource {
//!     url: Url::parse("https://www.redhat.com/.well-known/csaf/provider-metadata.json")?,
//!     options: Default::default(),
//!     fetcher,
//!   };
//!
//!   Walker::new(source.clone())
//!     .walk(RetrievingVisitor::new(
//!         source.clone(),
//!         ValidationVisitor::new(
//!             move |advisory: Result<ValidatedAdvisory, ValidationError>| async move {
//!                 log::info!("Found advisory: {advisory:?}");
//!                 Ok::<_, anyhow::Error>(())
//!             },
//!         )
//!     ))
//!     .await?;
//!
//!   Ok(())
//! }
//! ```

pub mod discover;
pub mod model;
pub mod retrieve;
pub mod source;
pub mod validation;
pub mod visitors;
pub mod walker;

#[cfg(feature = "csaf")]
pub mod verification;
