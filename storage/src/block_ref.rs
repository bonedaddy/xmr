// Xmr, Monero node.
// Copyright (C) 2018  Jean Pierre Dudey
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use primitives::H256;

#[derive(Debug)]
pub enum BlockRef {
    Height(u64),
    Id(H256),
}

impl From<u64> for BlockRef {
    fn from(height: u64) -> BlockRef {
        BlockRef::Height(height)
    }
}


impl From<H256> for BlockRef {
    fn from(id: H256) -> BlockRef {
        BlockRef::Id(id)
    }
}
