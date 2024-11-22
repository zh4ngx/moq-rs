use super::*;

use std::str::FromStr;

use derive_more::{Display, From};

use crate::catalog::Error;

#[derive(Debug, Clone, PartialEq, Eq, Display, From)]
pub enum VideoCodec {
	H264(H264),
	H265(H265),
	VP9(VP9),
	AV1(AV1),

	#[display("vp8")]
	VP8,

	#[display("{_0}")]
	Unknown(String),
}

impl FromStr for VideoCodec {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.starts_with("avc1.") {
			return H264::from_str(s).map(Into::into);
		} else if s.starts_with("hvc1.") {
			return H265::from_str(s).map(Into::into);
		} else if s == "vp8" {
			return Ok(Self::VP8);
		} else if s.starts_with("vp09.") {
			return VP9::from_str(s).map(Into::into);
		} else if s.starts_with("av01.") {
			return AV1::from_str(s).map(Into::into);
		}

		Ok(Self::Unknown(s.to_string()))
	}
}