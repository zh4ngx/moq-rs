use crate::coding::*;
use crate::Path;

#[derive(Clone, Debug)]
pub struct Fetch {
	pub path: Path,
	pub priority: i8,
	pub group: u64,
	pub offset: usize,
}

impl Encode for Fetch {
	fn encode<W: bytes::BufMut>(&self, w: &mut W) {
		self.path.encode(w);
		self.priority.encode(w);
		self.group.encode(w);
		self.offset.encode(w);
	}
}

impl Decode for Fetch {
	fn decode<R: bytes::Buf>(r: &mut R) -> Result<Self, DecodeError> {
		let path = Path::decode_more(r, 3)?;
		let priority = i8::decode_more(r, 2)?;
		let group = u64::decode_more(r, 1)?;
		let offset = usize::decode(r)?;

		Ok(Self {
			path,
			priority,
			group,
			offset,
		})
	}
}
