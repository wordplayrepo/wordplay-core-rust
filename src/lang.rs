/*
 * Copyright © 2024 Gregory P. Moyer
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::{fmt::{Debug, Display}, hash::{Hash, Hasher}};

use dyn_clone::{clone_trait_object, DynClone};

use crate::rust::{DynEq, DynHash};

/// A letter represents a single character that, when put together with other letters, creates a word that can be used in a placement.
pub trait Letter: Debug + Display + DynClone + DynEq + DynHash {
    /// Retrieve the character that represents this letter.
    fn character(&self) -> char;
}

clone_trait_object!(Letter);

impl Eq for dyn Letter {}

impl Hash for dyn Letter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dyn_hash(state)
    }
}

impl PartialEq<dyn Letter> for dyn Letter {
    fn eq(&self, other: &dyn Letter) -> bool {
        self.as_dyn_eq() == other.as_dyn_eq()
    }
}
