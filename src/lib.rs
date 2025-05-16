#![no_std]

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

//! ccgen
//!
//! Store information in order to fully generate a C header
//!
//! This crate is no_std so that no_std crates can use it
//! the use of alloc is for efficiency of storage and thus crates
//! that do not use alloc should
//! only conditionally include ccgen for header generation
//!
//! Features:
//!
//! tok - This feature includes the tokenizer which should be used when actually generating the headers
//! and for simplifing complex headers like an annex K compliant string.h
//!  This feature requires alloc.

extern crate alloc;

#[cfg(feature = "tok")]
pub mod tok;

use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize
};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
/// Header type
/// 
pub enum CXX {
    /// Header only supports C
    C,
    #[default]
    /// Header supports both C and C++
    CXX,
    /// Header only supports C++ (library is responsible for making sure C++ name mangling is done correctly)
    CXXOnly
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
/// Variadic function
///
pub enum Variadic {
    #[default]
    /// function takes constant (n) number of parameters
    Nary,
    /// function takes variable number of parameters
    Variadic
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Hash, Debug, Default)]
/// Header
///
pub struct Header {
    path: Option<&'static str>,
    name: &'static str,
    guard: Option<HeaderGuard>,
    funcs: Vec<Func>,
    macros: Vec<Macro>,
    types: Vec<Type>,
    cxx: CXX,
    extra: Option<&'static str>,
    post_extra: Option<&'static str>
}

impl Header {
    /// Create new header
    ///
    /// path - path after include/
    ///
    /// name - name of header
    ///
    /// guard - header guard
    ///
    /// funcs - functions
    ///
    /// macros - macros
    ///
    /// types - typedefs
    ///
    /// cxx - language support
    ///
    /// extra - other symbols
    ///
    /// post_extra - other symbols (after end of include guard)
    pub fn new(
	path: Option<&'static str>,
	name: &'static str,
	guard: Option<HeaderGuard>,
	funcs: &[Func],
	macros: &[Macro],
	types: &[Type],
	cxx: CXX,
	extra: Option<&'static str>,
	post_extra: Option<&'static str>
    ) -> Self {
	let mut f = Vec::new();
	let mut m = Vec::new();
	let mut t = Vec::new();
	f.extend_from_slice(funcs);
	m.extend_from_slice(macros);
	t.extend_from_slice(types);
	Self {
	    path,
	    name,
	    guard,
	    funcs: f,
	    macros: m,
	    types: t,
	    extra,
	    post_extra,
	    cxx
	}
    }

    /// path after include/
    pub fn path(&self) -> Option<&'static str> {
	self.path
    }

    /// name of header
    pub fn name(&self) -> &'static str {
	self.name
    }

    /// header guard
    pub fn guard(&self) -> Option<HeaderGuard> {
	self.guard
    }

    /// functions
    pub fn funcs(&self) -> &[Func] {
	self.funcs.as_slice()
    }

    /// macros
    pub fn macros(&self) -> &[Macro] {
	self.macros.as_slice()
    }

    /// typedefs
    pub fn types(&self) -> &[Type] {
	self.types.as_slice()
    }

    /// language support
    pub fn cxx(&self) -> CXX {
	self.cxx
    }

    /// other symbols
    pub fn extra(&self) -> Option<&'static str> {
	self.extra
    }

    /// other symbols (after header guard)
    pub fn post_extra(&self) -> Option<&'static str> {
	self.post_extra
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Hash, Debug, Default)]
/// Typedef
pub struct Type {
    name: &'static str,
    r#type: &'static str
}

impl Type {
    /// Create new typedef
    ///
    /// name - name of typedef
    ///
    /// type - type of typedef
    pub fn new(
	name: &'static str,
	r#type: &'static str
    ) -> Self {
	Self {
	    name,
	    r#type
	}
    }

    /// name of typedef
    pub fn name(&self) -> &'static str {
	self.name
    }

    /// type of typedef
    pub fn r#type(&self) -> &'static str {
	self.r#type
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Hash, Debug, Default)]
/// Header guard
pub struct HeaderGuard {
    tok: &'static str,
    val: &'static str
}

impl HeaderGuard {
    /// Create new header guard
    ///
    /// tok - name of guard token
    ///
    /// val - value of guard token
    pub fn new(
	tok: &'static str,
	val: &'static str
    ) -> Self {
	Self {
	    tok,
	    val
	}
    }

    /// guard token
    pub fn tok(&self) -> &'static str {
	self.tok
    }

    /// value of guard token
    pub fn val(&self) -> &'static str {
	self.val
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Hash, Debug, Default)]
/// Function
pub struct Func {
    out: &'static str,
    name: &'static str,
    params: Vec<&'static str>,
    va: Variadic
}

impl Func {
    /// Create new  function
    ///
    /// out - output type
    ///
    /// name - name of function
    ///
    /// params - parameters of function
    ///
    /// va - arity of function
    pub fn new(
	out: &'static str,
	name: &'static str,
	params: &[&'static str],
	va: Variadic
    ) -> Self {
	let mut p = Vec::new();
	p.extend_from_slice(params);
	Self {
	    out,
	    name,
	    va,
	    params: p
	}
    }

    /// output of function
    pub fn out(&self) -> &'static str {
	self.out
    }

    /// name of function
    pub fn name(&self) -> &'static str {
	self.name
    }

    /// parameters of  function
    pub fn params(&self) -> &[&'static str] {
	self.params.as_slice()
    }

    /// arity of function
    pub fn va(&self) -> Variadic {
	self.va
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Hash, Debug, Default)]
/// Macro
pub struct Macro {
    tok: &'static str,
    val: &'static str,
}

impl Macro {
    /// Create new macro
    ///
    /// tok - macro token (contains parameters if function macro)
    /// 
    /// val - value of token
    pub fn new(
	tok: &'static str,
	val: &'static str
    ) -> Self {
	Self {
	    tok,
	    val
	}
    }

    /// tok string
    pub fn tok(&self) -> &'static str {
	self.tok
    }

    /// val string
    pub fn val(&self) -> &'static str {
	self.val
    }
}
