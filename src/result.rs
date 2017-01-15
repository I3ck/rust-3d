/*
Copyright 2016 Martin Buck
This file is part of rust-3d.
rust-3d is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
rust-3d is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.
You should have received a copy of the GNU Lesser General Public License
along with rust-3d.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::result;
use std::fmt;

pub enum ErrorKind {
    MinMaxSwapped,
    MinMaxEqual,
    TooFewPoints,
    BoundingBoxMissing
}

impl ErrorKind {
    fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::MinMaxSwapped      => "Passed min/max values are swapped (min > max)",
            ErrorKind::MinMaxEqual        => "Passed min/max values are equal",
            ErrorKind::TooFewPoints       => "Container had too few points for the operation",
            ErrorKind::BoundingBoxMissing => "Bounding box is missing for the operation"
        }
    }
}

impl fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub type Result<T> = result::Result<T, ErrorKind>;
