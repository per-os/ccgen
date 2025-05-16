/*

BSD 3-Clause License

Copyright (c) 2025, Isaac Budzik

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

*/

use alloc::string::String;
use core::fmt::Write;

use crate::{
    Header,
    Macro,
    Type,
    Func,
    HeaderGuard,
    CXX,
    Variadic
};

/// Create C tokens from item
pub trait Token {
    fn token(&self) -> String;
}

/// Certain items (header guards, language support) require tokens at both the begenning and end of a header
/// EndToken is for such cases
pub trait EndToken {
    fn end_token(&self) -> String;
}

impl Token for String {
    fn token(&self) -> String {
	self.clone()
    }
}

impl Token for &str {
    fn token(&self) -> String {
	String::from(*self)
    }
}

impl Token for u64 {
    fn token(&self) -> String {
	let mut s = String::new();
	let _ = core::write!(s, "{}UL", *self);
	s
    }
}

impl Token for u32 {
    fn token(&self) -> String {
	let mut s = String::new();
	let _ = core::write!(s, "{}", *self);
	s
    }
}

impl Token for u16 {
    fn token(&self) -> String {
	let mut s = String::new();
	let _ = core::write!(s, "{}", *self);
	s
    }
}

impl Token for u8 {
    fn token(&self) -> String {
	let mut s = String::new();
	let _ = core::write!(s, "{}", *self);
	s
    }
}

impl Token for i64 {
    fn token(&self) -> String {
	let mut s = String::new();
	let _ = core::write!(s, "{}L", *self);
	s
    }
}

impl Token for i32 {
    fn token(&self) -> String {
	let mut s = String::new();
	let _ = core::write!(s, "{}", *self);
	s
    }
}

impl Token for i16 {
    fn token(&self) -> String {
	let mut s = String::new();
	let _ = core::write!(s, "{}", *self);
	s
    }
}

impl Token for i8 {
    fn token(&self) -> String {
	let mut s = String::new();
	let _ = core::write!(s, "{}", *self);
	s
    }
}

impl Token for f32 {
    fn token(&self) -> String {
	let mut s = String::new();
	let _ = core::write!(s, "{}", *self);
	if !s.contains(".") {
	    s.push('.');
	}
	s.push('F');
	s
    }
}

impl Token for f64 {
    fn token(&self) -> String {
	let mut s = String::new();
	let _ = core::write!(s, "{}", *self);
	if !s.contains(".") {
	    s.push('.');
	}
	s
    }
}

impl Token for usize {
    fn token(&self) -> String {
	let mut s = String::new();
	let _ = core::write!(s, "0x{:x}UL", *self);
	s
    }
}

impl Token for char {
    fn token(&self) -> String {
	String::from(*self)
    }
}

impl Token for CXX {
    fn token(&self) -> String {
	let mut out = String::new();
	match self {
	    CXX::C => {
		out.push_str("#ifdef __cplusplus\n#error \"This header can only be used by C\"\n#endif\n");
	    },
	    CXX::CXX => {
		out.push_str("#ifdef __cplusplus\nextern \"C\" {\n#endif\n");
	    },
	    CXX::CXXOnly => {
		out.push_str("#ifndef __cplusplus\n#error \"This header can only be used by C++\"\n#endif\n");
	    }
	}
	out
    }
}

impl EndToken for CXX {
    fn end_token(&self) -> String {
	let mut out = String::new();
	match self {
	    CXX::CXX => {
		out.push_str("#ifdef __cplusplus\n}\n#endif\n");
	    },
	    _ => {}
	}
	out
    }
}

impl Token for Header<'_> {
    fn token(&self) -> String {
	let mut out = String::new();
	if let Some(guard) =  &self.guard {
	    out.push_str(&guard.token());
	    out.push('\n');
	}
	out.push_str(&self.cxx.token());
	out.push('\n');
	for i in 0..self.num_types {
	    out.push_str(&self.types()[i].token());
	}
	out.push('\n');
	for i in 0..self.num_macros {
	    out.push_str(&self.macros()[i].token());
	}
	out.push('\n');
	for i in 0..self.num_funcs {
	    out.push_str(&self.funcs()[i].token());
	}
	out.push('\n');
	if let Some(extra) = self.extra {
	    out.push_str(extra);
	    out.push('\n');
	}
	out.push_str(&self.cxx.end_token());
	out.push('\n');
	if let Some(guard) = &self.guard {
	    out.push_str(&guard.end_token());
	    out.push('\n');
	}
	if let Some(post_extra) = &self.post_extra {
	    out.push_str(post_extra);
	    out.push('\n');
	}
	out
    }
}

impl Token for HeaderGuard<'_> {
    fn token(&self) -> String {
	let mut out = String::from("#ifndef ");
	out.push_str(self.tok);
	out.push_str("\n#define ");
	out.push_str(self.tok);
	out.push(' ');
	out.push_str(self.val);
	out.push('\n');
	out
    }
}

impl EndToken for HeaderGuard<'_> {
    fn end_token(&self) -> String {
	String::from("#endif\n")
    }
}

impl Token for Func<'_> {
    fn token(&self) -> String {
	let mut out = String::from(self.out);
	out.push(' ');
	out.push_str(self.name);
	out.push('(');
	for i in 0..self.num_params {
	    if i != 0 {
		out.push_str(", ");
	    }
	    out.push_str(self.params()[i]);
	}
	if let Variadic::Variadic = self.va {
	    if self.num_params == 0 {
		out.push_str("...");
	    } else {
		out.push_str(", ...");
	    }
	}
	out.push_str(");\n");
	out
    }
}

impl Token for Macro<'_> {
    fn token(&self) -> String {
	let mut out = String::from("#define ");
	out.push_str(self.tok);
	out.push(' ');
	out.push_str(self.val);
	out.push('\n');
	out
    }
}

impl Token for Type<'_> {
    fn token(&self) -> String {
	let mut out = String::from("typedef ");
	out.push_str(self.r#type);
	out.push(' ');
	out.push_str(self.name);
	out.push_str(";\n");
	out
    }
}

#[cfg(test)]
mod test {
    use super::{
	super::{
	    Macro,
	    Type,
	    Func,
	    Variadic,
	    HeaderGuard,
	    CXX,
	    Header
	},
	Token,
	EndToken
    };
    #[test]
    fn r#macro() {
	let m = Macro::new(
	    "H",
	    "1"
	).token();
	assert_eq!(&m, "#define H 1\n");
    }

    #[test]
    fn r#type() {
	let t = Type::new(
	    "size_t",
	    "unsigned long"
	).token();
	assert_eq!(&t, "typedef unsigned long size_t;\n");
    }

    #[test]
    fn func() {
	let f1=  Func::new(
	    "int",
	    "printf",
	    &["const char*"],
	    Variadic::Variadic
	).token();
	let f2 = Func::new(
	    "void",
	    "q",
	    &[],
	    Variadic::Variadic
	).token();
	let f3 = Func::new(
	    "void",
	    "q",
	    &[],
	    Variadic::Nary
	).token();
	let f4 = Func::new(
	    "void",
	    "q",
	    &[
		"a",
		"b",
		"c",
		"d",
		"e",
		"f",
		"g",
		"h",
		"i",
		"j",
		"k",
		"l",
		"m",
		"n",
		"o",
		"p",
		"q",
		"r",
		"s",
		"t",
		"u",
		"v",
		"w",
		"x",
		"y",
		"z"
	    ],
	    Variadic::Nary
	).token();
	assert_eq!(&f1, "int printf(const char*, ...);\n");
	assert_eq!(&f2, "void q(...);\n");
	assert_eq!(&f3, "void q();\n");
	assert_eq!(&f4, "void q(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z);\n");
    }

    #[test]
    fn header_guard() {
	let h1 = HeaderGuard::new("a", "1").token();
	let h2 = HeaderGuard::new("a", "").token();
	let e = HeaderGuard::new("b", "").end_token();
	assert_eq!(&h1, "#ifndef a\n#define a 1\n");
	assert_eq!(&h2, "#ifndef a\n#define a \n");
	assert_eq!(&e, "#endif\n");
    }

    #[test]
    fn cxx() {
	let c = CXX::C;
	let cxx = CXX::CXX;
	let cxx_only = CXX::CXXOnly;
	assert_eq!(&c.token(), "#ifdef __cplusplus\n#error \"This header can only be used by C\"\n#endif\n");
	assert_eq!(&cxx.token(), "#ifdef __cplusplus\nextern \"C\" {\n#endif\n");
	assert_eq!(&cxx_only.token(), "#ifndef __cplusplus\n#error \"This header can only be used by C++\"\n#endif\n");
	assert_eq!(&c.end_token(), "");
	assert_eq!(&cxx.end_token(), "#ifdef __cplusplus\n}\n#endif\n");
	assert_eq!(&cxx_only.end_token(), "");
    }

    #[test]
    fn header() {
	let f1=  Func::new(
	    "int",
	    "printf",
	    &["const char*"],
	    Variadic::Variadic
	);
	let t = Type::new(
	    "size_t",
	    "unsigned long"
	);
	let m = Macro::new(
	    "H",
	    "1"
	);
	let h = Header::new(
	    None,
	    "test.h",
	    None,
	    &[f1],
	    &[m],
	    &[t],
	    CXX::CXX,
	    None,
	    None
	).token();
	assert_eq!(&h, "#ifdef __cplusplus\nextern \"C\" {\n#endif\n\ntypedef unsigned long size_t;\n\n#define H 1\n\nint printf(const char*, ...);\n\n#ifdef __cplusplus\n}\n#endif\n\n");
    }

    #[test]
    fn nums() {
	let a: u64 = 12;
	let b: u32 = 11;
	let c: u16 = 10;
	let d: u8 = 9;
	let e: i64 = -12;
	let f: i32 = -11;
	let g: i16 = -10;
	let h: i8 = -9;
	let i: f32 = 1.3;
	let j: f64 = -1.3;
	let k: f64 = -1.;
	let l: f32 = 1002.;
	let m: usize = 0x1234abcd;
	assert_eq!(&a.token(), "12UL");
	assert_eq!(&b.token(), "11");
	assert_eq!(&c.token(), "10");
	assert_eq!(&d.token(), "9");
	assert_eq!(&e.token(), "-12L");
	assert_eq!(&f.token(), "-11");
	assert_eq!(&g.token(), "-10");
	assert_eq!(&h.token(), "-9");
	assert_eq!(&i.token(), "1.3F");
	assert_eq!(&j.token(), "-1.3");
	assert_eq!(&k.token(), "-1.");
	assert_eq!(&l.token(), "1002.F");
	assert_eq!(&m.token(), "0x1234abcdUL");
    }

    #[test]
    fn r#char() {
	let a: char = '𐘊';
	let b: char = '';
	let c: char = ' ';
	let d: char = '?';
	assert_eq!(&a.token(), "𐘊");
	assert_eq!(&b.token(), "");
	assert_eq!(&c.token(), " ");
	assert_eq!(&d.token(), "?");
    }
}
