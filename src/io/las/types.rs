/*
Copyright 2020 Martin Buck

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"),
to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall
be included all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

//! Module for types for the las file format

use std::io::Error as ioError;

//------------------------------------------------------------------------------

//@todo I used u8 as unsigned char, which is incorrect, must fix this

pub struct Header {
    //COUNT_BYTES_THIS COUNT_BYTES_TOTAL_HERE
    pub file_signature: [char; 4],           //4 4
    pub file_source_id: u16,                 //2 6
    pub global_encoding: u16,                //2 8
    pub guid1: u32,                          //4 12
    pub guid2: u16,                          //2 14
    pub guid3: u16,                          //2 16
    pub guid4: [u8; 8],                      //8 24
    pub version_major: u8,                   //1 25
    pub version_minor: u8,                   //1 26
    pub system_identifier: [char; 32],       //32 58
    pub generating_software: [char; 32],     //32 90
    pub file_creation_day: u16,              //2 92
    pub file_creation_year: u16,             //2 94
    pub header_size: u16,                    //2 96
    pub offset_point_data: u32,              //4 100
    pub n_variable_length_records: u32,      //4 104
    pub point_record_format: u8,             //1 105
    pub point_record_length: u16,            //2 107
    pub legacy_n_point_records: u32,         //4 111
    pub legacy_n_point_return: [u32; 5],     //20 131
    pub scale_factor_x: f64,                 //8 139
    pub scale_factor_y: f64,                 //8 147
    pub scale_factor_z: f64,                 //8 155
    pub offset_x: f64,                       //8 163
    pub offset_y: f64,                       //8 171
    pub offset_z: f64,                       //8 179
    pub max_x: f64,                          //8 187
    pub min_x: f64,                          //8 195
    pub max_y: f64,                          //8 203
    pub min_y: f64,                          //8 211
    pub max_z: f64,                          //8 219
    pub min_z: f64,                          //8 227
    pub start_wavefront_data: u64,           //8 235
    pub start_extended_variable_length: u64, //8 243
    pub n_extended_variable_length: u32,     //4 247
    pub n_point_records: u64,                //8 255
    pub n_points_return: [u64; 15],          //120 375
}

//------------------------------------------------------------------------------

/// Error type for .las file operation
pub enum LasError {
    AccessFile,
    BinaryData,
}

/// Result type for .las file operation
pub type LasResult<T> = std::result::Result<T, LasError>;

impl From<ioError> for LasError {
    fn from(_error: ioError) -> Self {
        LasError::AccessFile
    }
}

impl From<std::array::TryFromSliceError> for LasError {
    fn from(_error: std::array::TryFromSliceError) -> Self {
        LasError::BinaryData
    }
}
