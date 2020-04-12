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

use super::super::from_bytes::FromBytesError;

//------------------------------------------------------------------------------

#[derive(Debug)]
// This header reflects the 1.4 spec
pub struct Header {
    //COUNT_BYTES_THIS COUNT_BYTES_TOTAL_HERE
    pub file_signature: [u8; 4],       //4 4 //@todo 'signed' char in spec
    pub file_source_id: u16,           //2 6
    pub global_encoding: u16,          //2 8
    pub guid1: u32,                    //4 12
    pub guid2: u16,                    //2 14
    pub guid3: u16,                    //2 16
    pub guid4: [u8; 8],                //8 24
    pub version_major: u8,             //1 25
    pub version_minor: u8,             //1 26
    pub system_identifier: [u8; 32],   //32 58 //@todo 'signed' char in spec
    pub generating_software: [u8; 32], //32 90 //@todo 'signed' char in spec
    pub file_creation_day: u16,        //2 92
    pub file_creation_year: u16,       //2 94
    pub header_size: u16,              //2 96
    pub offset_point_data: u32,        //4 100
    pub n_variable_length_records: u32, //4 104
    pub point_record_format: u8,       //1 105
    pub point_record_length: u16,      //2 107
    pub legacy_n_point_records: u32,   //4 111
    pub legacy_n_point_return: [u32; 5], //20 131
    pub scale_factor_x: f64,           //8 139
    pub scale_factor_y: f64,           //8 147
    pub scale_factor_z: f64,           //8 155
    pub offset_x: f64,                 //8 163
    pub offset_y: f64,                 //8 171
    pub offset_z: f64,                 //8 179
    pub max_x: f64,                    //8 187
    pub min_x: f64,                    //8 195
    pub max_y: f64,                    //8 203
    pub min_y: f64,                    //8 211
    pub max_z: f64,                    //8 219
    pub min_z: f64,                    //8 227
    pub start_wavefront_data: u64,     //8 235
    pub start_extended_variable_length: u64, //8 243
    pub n_extended_variable_length: u32, //4 247
    pub n_point_records: u64,          //8 255
    pub n_points_return: [u64; 15],    //120 375
}

pub struct ExtendedVariableLengthHeader {
    pub reserved: u16,                   //2 2
    pub user_id: [u8; 16],               //16 18 //@todo 'signed' char in spec
    pub record_id: u16,                  //2 20
    pub record_length_after_header: u64, //8 28
    pub description: [u8; 32],           //32 60 //@todo 'signed' char in spec
}

//------------------------------------------------------------------------------

pub struct PointData {
    pub x: i32, //4 4
    pub y: i32, //4 8
    pub z: i32, //4 12
}

pub struct ColorData {
    pub red: u16,   //2 2
    pub green: u16, //2 4
    pub blue: u16,  //2 6
}

pub struct WaveData {
    pub wave_descriptor_index: u8,      //1 1
    pub offset_waveform_data: u64,      //8 9
    pub waveform_packet_size: u32,      //4 13
    pub return_point_waveform_loc: f32, //4 17
    pub dx: f32,                        //4 21
    pub dy: f32,                        //4 25
    pub dz: f32,                        //4 29
}

pub struct Format0 {
    pub point_data: PointData, //12 12
    pub intensity: u16,        //2 14
    pub bitdata: u8, //1 15 ReturnNumber 3 bits, NumberOfReturns 3 bits, ScanDirFlag 1 bit, EdgeOfFlightLine 1 bit
    pub classification: u8, //1 16
    pub scan_angle_rank: u8, //1 17 actually signed char
    pub user_data: u8, // 1 18
    pub point_source_id: u16, //2 20
}

pub struct Format1 {
    pub format_0: Format0, //20 20
    pub gps_time: f64,     //8 28
}

pub struct Format2 {
    pub format_0: Format0,     //20 20
    pub color_data: ColorData, //6 26
}

pub struct Format3 {
    pub format_1: Format1,     //28 28
    pub color_data: ColorData, //6 34
}

pub struct Format4 {
    pub format_1: Format1,   //28 28
    pub wave_data: WaveData, //29 57
}

pub struct Format5 {
    pub format_3: Format3,   //34 34
    pub wave_data: WaveData, //29 63
}

pub struct Format6 {
    pub point_data: PointData, //12 12
    pub intensity: u16,        //2 14
    pub bitdata: u16, //2 16 //Return Number 4 bits, NumberofReturns 4bits, ClassificationFlags 4 bits, ScannerChannel 2 bits, ScanDirFlag: 1bit, EdgeOfFlightLine 1bit
    pub classification: u8, //1 17
    pub user_data: u8, //1 18
    pub scan_angle: i16, //2 20
    pub point_source_id: u16, //2 22
    pub gps_time: f64, //8 30
}

pub struct Format7 {
    pub format_6: Format6,     //30 30
    pub color_data: ColorData, //6 36
}

pub struct Format8 {
    pub format_7: Format7, //36 36
    pub nir: u16,          //2 38
}

pub struct Format9 {
    pub format_6: Format6,   //30 30
    pub wave_data: WaveData, //29 59
}

pub struct Format10 {
    pub format_6: Format6,     //30 30
    pub color_data: ColorData, //6 36
    pub nir: u16,              //2 38
    pub wave_data: WaveData,   //29 67
}

//------------------------------------------------------------------------------

/// Error type for .las file operation
#[derive(Debug)]
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

impl From<FromBytesError> for LasError {
    fn from(_error: FromBytesError) -> Self {
        LasError::BinaryData
    }
}
