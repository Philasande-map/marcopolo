// Copyright 2019 MarcoPolo Protocol Authors.
// This file is part of MarcoPolo Protocol.

// MarcoPolo Protocol is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// MarcoPolo Protocol is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with MarcoPolo Protocol.  If not, see <http://www.gnu.org/licenses/>.

use serde::{Serialize, Deserialize};
use crate::hash::{Hash};

/// Block header
#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
	pub height: u64,
	pub parent_hash: Hash,
    pub time: u64,
}

impl Default for Header {
	fn default() -> Self {
		Header {
			height: 0,
			parent_hash: Hash([0; 32]),
			time: 0,
		}
	}
}

pub struct Block {
    pub header: Header,
}

impl Default for Block {
    fn default() -> Self {
        Block {
            header: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::bincode;

    #[test]
    fn test_header_hash() {
        let head: Header = Default::default();
        let encoded: Vec<u8> = bincode::serialize(&head).unwrap();
        assert_eq!(encoded, vec![0; 48]);
    }
}