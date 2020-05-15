// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, May 2020

/// A set of usage specifiers
///
/// A usage specifier acts as a hint for the underlying drawing algorithms to perform more
/// effectively.
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Usage {
    /// Constantly changing data.
    Static,
    /// Occasionally changing data.
    Dynamic,
    /// Rarely changing data.
    Stream
}
