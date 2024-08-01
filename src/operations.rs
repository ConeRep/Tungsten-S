#[derive(Clone, PartialEq)]
pub enum Operations {
	// Characters/Numbers
    Push,
	Plus,
	Minus,
    Equal,
    GreaterThan,
	
    // Keywords
    If,
    Else,
    End,
    While,
    Do,
    Dupl,
    Mem,

    // Bools
    True,
    False,

    Dump,
}

pub const MEM_CAPACITY: i64 = 640000;