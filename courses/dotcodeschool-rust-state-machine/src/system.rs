/* TODO: You might need to update your imports. */
use std::collections::BTreeMap;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
pub struct Pallet {
	/// The current block number.
	/* field `block_number` that stores a `u32`. */
	block_number: u32,

	/// A map from an account to their nonce.
	/* field `nonce` that is a `BTreeMap` from `String` to `u32`. */
	nounce: BTreeMap<String, u32>,
}

impl Pallet {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		/* Return a new instance of the `Pallet` struct. */
		Self { block_number: 0, nounce: BTreeMap::new() }
	}
}
