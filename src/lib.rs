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
//! this crate is no_std so that no_std crates can use it
//! the use of alloc is for efficiency of storage and thus crates do not use alloc should
//! only conditionally include ccgen for header generation
//!
//! features:
//!
//! tok - this feature includes the tokenizer which should be used when actually generating the headers
//! and for simplifing complex headers like an annex K compliant string.h
//! this feature requires alloc

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
/// 
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
pub enum Variadic {
    #[default]
    /// function takes constant (n) number of parameters
    Nary,
    /// function takes variable number of parameters
    Variadic
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Hash, Debug, Default)]
pub struct Header {
    /// path after include/ 
    pub path: Option<&'static str>,
    /// file name
    pub name: &'static str,
    /// header guard if used
    pub guard: Option<HeaderGuard>,
    /// vector of functions
    pub funcs: Vec<Func>,
    /// vector of macros
    pub macros: Vec<Macro>,
    /// vector of types
    pub types: Vec<Type>,
    /// what languages header supports
    pub cxx: CXX,
    /// other symbols to be included
    pub extra: Option<&'static str>,
    /// other symbols to be include (after end of include guard)
    pub post_extra: Option<&'static str>
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Hash, Debug, Default)]
/// typedef
pub struct Type {
    /// name of type
    pub name: &'static str,
    /// type of  type
    pub r#type: &'static str
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Hash, Debug, Default)]
/// header guard
pub struct HeaderGuard {
    /// name of guard
    pub tok: &'static str,
    /// value of guard
    pub val: &'static str
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Hash, Debug, Default)]
/// function
pub struct Func {
    /// output
    pub out: &'static str,
    /// name
    pub name: &'static str,
    /// vector of prameters
    pub params: Vec<&'static str>,
    /// n-ary of variadic function
    pub va: Variadic
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Hash, Debug, Default)]
/// macro
pub struct Macro {
    /// name of macro (contains paramters if function macro)
    pub tok: &'static str,
    /// token value of macro
    pub val: &'static str,
}
