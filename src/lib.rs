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
//! Features:
//!
//! tok - This feature includes the tokenizer which should be used when actually generating the headers
//! and for simplifing complex headers like an annex K compliant string.h
//!  This feature requires alloc.

#[cfg(feature = "tok")]
extern crate alloc;

#[cfg(feature = "tok")]
pub mod tok;

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize
};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
/// Header type
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
pub enum Variadic {
    #[default]
    /// function takes constant (n) number of parameters
    Nary,
    /// function takes variable number of parameters
    Variadic
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Hash, Debug, Default)]
/// Header
pub struct Header<'a> {
    path: Option<&'a str>,
    name: &'a str,
    guard: Option<HeaderGuard<'a>>,
    funcs_ptr: usize, // *const Func<'a>
    num_funcs: usize,
    macros_ptr: usize, // *const Macro<'a>
    num_macros: usize,
    types_ptr: usize, // *const Type<'a>
    num_types: usize,
    cxx: CXX,
    extra: Option<&'a str>,
    post_extra: Option<&'a str>
}

impl<'a> Header<'a> {
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
	path: Option<&'a str>,
	name: &'a str,
	guard: Option<HeaderGuard<'a>>,
	funcs: &'a [Func<'a>],
	macros: &'a [Macro<'a>],
	types: &'a [Type<'a>],
	cxx: CXX,
	extra: Option<&'a str>,
	post_extra: Option<&'a str>
    ) -> Self {
	let funcs_ptr = funcs.as_ptr() as usize;
	let macros_ptr=  macros.as_ptr() as usize;
	let types_ptr = types.as_ptr() as usize;
	Self {
	    path,
	    name,
	    guard,
	    funcs_ptr,
	    macros_ptr,
	    types_ptr,
	    extra,
	    post_extra,
	    cxx,
	    num_funcs: funcs.len(),
	    num_types: types.len(),
	    num_macros: macros.len()
	}
    }

    /// path after include/
    pub fn path(&self) -> Option<&'a str> {
	self.path
    }

    /// name of header
    pub fn name(&self) -> &'a str {
	self.name
    }

    /// header guard
    pub fn guard(&self) -> Option<HeaderGuard> {
	self.guard
    }

    /// functions
    pub fn funcs(&self) -> &'a [Func<'a>] {
	unsafe {
	    core::slice::from_raw_parts(self.funcs_ptr as *const Func<'a>, self.num_funcs)
	}
    }

    /// macros
    pub fn macros(&self) -> &'a [Macro<'a>] {
	unsafe {
	    core::slice::from_raw_parts(self.macros_ptr as *const Macro<'a>, self.num_macros)
	}
    }

    /// typedefs
    pub fn types(&self) -> &'a [Type<'a>] {
	unsafe {
	    core::slice::from_raw_parts(self.types_ptr as *const Type<'a>, self.num_types)
	}
    }

    /// language support
    pub fn cxx(&self) -> CXX {
	self.cxx
    }

    /// other symbols
    pub fn extra(&self) -> Option<&'a str> {
	self.extra
    }

    /// other symbols (after header guard)
    pub fn post_extra(&self) -> Option<&'a str> {
	self.post_extra
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Hash, Debug, Default)]
/// Typedef
pub struct Type<'a> {
    name: &'a str,
    r#type: &'a str
}

impl<'a> Type<'a> {
    /// Create new typedef
    ///
    /// name - name of typedef
    ///
    /// type - type of typedef
    pub fn new(
	name: &'a str,
	r#type: &'a str
    ) -> Self {
	Self {
	    name,
	    r#type
	}
    }

    /// name of typedef
    pub fn name(&self) -> &'a str {
	self.name
    }

    /// type of typedef
    pub fn r#type(&self) -> &'a str {
	self.r#type
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Hash, Debug, Default)]
/// Header guard
pub struct HeaderGuard<'a> {
    tok: &'a str,
    val: &'a str
}

impl<'a> HeaderGuard<'a> {
    /// Create new header guard
    ///
    /// tok - name of guard token
    ///
    /// val - value of guard token
    pub fn new(
	tok: &'a str,
	val: &'a str
    ) -> Self {
	Self {
	    tok,
	    val
	}
    }

    /// guard token
    pub fn tok(&self) -> &'a str {
	self.tok
    }

    /// value of guard token
    pub fn val(&self) -> &'a str {
	self.val
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Hash, Debug, Default)]
/// Function
pub struct Func<'a> {
    out: &'a str,
    name: &'a str,
    params_ptr: usize, // *const &'a str
    num_params: usize,
    va: Variadic
}

impl<'a> Func<'a> {
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
	out: &'a str,
	name: &'a str,
	params: &'a[&'a str],
	va: Variadic
    ) -> Self {
	let params_ptr = params.as_ptr() as usize;
	Self {
	    out,
	    name,
	    va,
	    params_ptr,
	    num_params: params.len()
	}
    }

    /// output of function
    pub fn out(&self) -> &'a str {
	self.out
    }

    /// name of function
    pub fn name(&self) -> &'a str {
	self.name
    }

    /// parameters of  function
    pub fn params(&self) -> &'a [&'a str] {
	unsafe {
	    core::slice::from_raw_parts(self.params_ptr as *const &'a str, self.num_params)
	}
    }

    /// arity of function
    pub fn va(&self) -> Variadic {
	self.va
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Hash, Debug, Default)]
/// Macro
pub struct Macro<'a> {
    tok: &'a str,
    val: &'a str,
}

impl<'a> Macro<'a> {
    /// Create new macro
    ///
    /// tok - macro token (contains parameters if function macro)
    /// 
    /// val - value of token
    pub fn new(
	tok: &'a str,
	val: &'a str
    ) -> Self {
	Self {
	    tok,
	    val
	}
    }

    /// tok string
    pub fn tok(&self) -> &'a str {
	self.tok
    }

    /// val string
    pub fn val(&self) -> &'a str {
	self.val
    }
}
