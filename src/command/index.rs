use std::cmp;

#[derive(Debug, PartialEq)]
pub enum MinMax {
	Nth(u8),
	Between((u8, u8)),
}

impl MinMax {
	pub fn parse(s: &str) -> Option<Self> {
		let vals: Vec<_> = s.split(':').collect();
		match vals.len() {
			1 => vals[0].parse::<u8>().map(|n| Self::Nth(n)).ok(),
			2 => {
				let (x, y) = (vals[0], vals[1]);
				let left = if x.is_empty() {
					0u8
				} else if let Ok(n) = x.parse::<u8>() {
					n
				} else {
					return None;
				};
				let right= if y.is_empty() {
					u8::MAX
				} else if let Ok(n) = y.parse::<u8>() {
					n
				} else {
					return None;
				};
				Some(Self::Between(left, right))
			}
			_ => None,
		}
	}

	pub fn in_range(&self, n: u8) -> bool {
		match self {
			Self::Nth(x) => x == n,
			Self::Between((min, max)) => n >= min && n <= max,
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum Index {
	Nth(isize),
	Between((isize, isize)),
}

impl Default for Index {
	fn default() -> Self {
		Self::Between(0, isize::MAX)
	}
}

impl Index {
	pub fn parse(s: &str) -> Option<Self> {
		let vals: Vec<_> = s.split(':').collect();
		match vals.len() {
			1 => vals[0].parse::<isize>().map(|n| Self::Nth(n)).ok(),
			2 => {
				let (x, y) = (vals[0], vals[1]);
				let left= if x.is_empty() {
					0isize
				} else if let Ok(n) = x.parse::<isize>() {
					n
				} else {
					return None;
				};
				let right= if y.is_empty() {
					isize::MAX
				} else if let Ok(n) = y.parse::<isize>() {
					n
				} else {
					return None;
				};

				Some(Self::Between(left, right))
			}
			_ => None,
		}
	}

	pub fn slice<'a, T>(&self, sl: &'a [T]) -> Option<&'a [T]> {
		match self {
			Self::Nth(&n) if n < sl.len() => {
				let x = if n < 0 { sl.len() - n } else { n };
				if x < 0 {
					None
				} else {
					let x = usize::try_from(x).unwrap();
					Some(&sl[x..x + 1])
				}
			}
			Self::Nth(_) => None,
			Self::Between((start, end)) => {
				let len = isize::try_from(sl.len()).unwrap();
				let start = if *start < 0 { len + *start } else { *start };
				let end = if *end < 0 { *end + len } else { *end };

				if start == end {
					return None;
				}
				let mut min = cmp::min(start, end);
				let mut max = cmp::max(start, end);
				if min < 0 {
					min = 0;
				} else if min >= len {
					return None;
				}
				if max > len {
					max = len;
				} else if max <= 0 {
					return None;
				}
				Some(&sl[usize::try_from(min).unwrap()..usize::try_from(max).unwrap()])
			}
		}
	}
	
	pub fn in_range(&self, n: isize) -> bool{
		match self{
			Self::Nth(&x)=> x == n,
			Self::Between((&x, &y)) => {
				let (min, max)= (cmp::min(x, y), cmp::max(x, y));
				min <= n && max> n
			}
		}
	}
	
	pub fn calibrate(&mut self, len: usize) {
		let len= isize::try_from(len).unwrap_or(isize::MAX);
		match self{
			Self::Nth(n) if n < 0 => {
					self = if n + len < 0 {
						Self::Nth(isize::MAX)
					}else{
						Self::Nth(n + len)
					};
				
			}
			Self::Between((left, right)) => {
				let left= if left < 0 {
					left + len
				}else if left > len{
					len
				}else{
					left
				};
				if left < 0 {
					self= Self::Nth(isize::MAX);
					return;
				}
				let right = if right < 0 {
					right + len
				}else if right > len{
					len
				}else{
					right
				};
				self = if right < 0 {
					Self::Nth(isize::MAX)
				}else{
					Self::Between((left, right))
				};
			}
			_=> (),
		};
	}
	
	pub fn is_reversed(&self) -> bool{
		match self{
			Self::Nth(_)=> false,
			Self::Between((&left, &right)) => left < right,
		}
	}
}
