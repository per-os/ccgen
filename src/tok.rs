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

//! 
//! 
//! 

use alloc::string::String;

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
    fn tok(&self) -> String;
}

trait EndToken {
    fn end_tok(&self) -> String;
}

impl Token for Header {
    fn tok(&self) -> String {
	let mut out = String::new();
	if let Some(guard) =  &self.guard {
	    out.push_str(&guard.tok());
	    out.push('\n');
	}
	match self.cxx {
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
	for i in 0..self.types.len() {
	    out.push_str(&self.types[i].tok());
	}
	out.push('\n');
	for i in 0..self.macros.len() {
	    out.push_str(&self.macros[i].tok());
	}
	out.push('\n');
	for i in 0..self.funcs.len() {
	    out.push_str(&self.funcs[i].tok());
	}
	out.push('\n');
	if let Some(extra) = self.extra {
	    out.push_str(extra);
	    out.push('\n');
	}
	match self.cxx {
	    CXX::CXX => {
		out.push_str("#ifdef __cplusplus\n}\n#endif\n");
	    },
	    _ => {}
	}
	if let Some(guard) = &self.guard {
	    out.push_str(&guard.end_tok());
	    out.push('\n');
	}
	if let Some(post_extra) = &self.post_extra {
	    out.push_str(post_extra);
	    out.push('\n');
	}
	out
    }
}

impl Token for HeaderGuard {
    fn tok(&self) -> String {
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

impl EndToken for HeaderGuard {
    fn end_tok(&self) -> String {
	String::from("#endif\n")
    }
}

impl Token for Func {
    fn tok(&self) -> String {
	let mut out = String::from(self.out);
	out.push(' ');
	out.push_str(self.name);
	out.push('(');
	for i in 0..self.params.len() {
	    if i != 0 {
		out.push_str(", ");
	    }
	    out.push_str(self.params[i]);
	}
	if let(Variadic::Variadic) = self.va {
	    if self.params.len() == 0 {
		out.push_str("...");
	    } else {
		out.push_str(", ...");
	    }
	}
	out.push_str(");\n");
	out
    }
}

impl Token for Macro {
    fn tok(&self) -> String {
	let mut out = String::from("#define ");
	out.push_str(self.tok);
	out.push(' ');
	out.push_str(self.val);
	out.push('\n');
	out
    }
}

impl Token for Type {
    fn tok(&self) -> String {
	let mut out = String::from("typedef ");
	out.push_str(self.r#type);
	out.push(' ');
	out.push_str(self.name);
	out.push('\n');
	out
    }
}
