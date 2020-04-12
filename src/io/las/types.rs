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

use std::{
    convert::{TryFrom, TryInto},
    io::{Error as ioError, Read},
};

use super::super::from_bytes::*;

//------------------------------------------------------------------------------

//@todo move all these and rename

//@todo already in std? //@todo could use From, too
pub trait FromRead {
    fn from_read<R>(&mut self, read: &mut R) -> LasResult<()>
    where
        R: Read;
}

pub trait FormatGeneric: FromRead {
    //@todo this is currently not scaled etc.
    fn point_data(&self) -> &PointData;
}

//------------------------------------------------------------------------------

//@todo better name (everything here)
#[derive(Debug)]
pub struct RelevantHeader {
    pub offset_point_data: u32,
    pub point_record_format: Format,
    pub point_record_length: u16,
    pub n_point_records: u64,
    pub n_points_return: [u64; 15],
}

impl TryFrom<Header> for RelevantHeader {
    type Error = LasError;

    fn try_from(x: Header) -> LasResult<RelevantHeader> {
        // These are conversions according to the legacy mode
        let n_point_records = if x.legacy_n_point_records == 0 {
            x.n_point_records
        } else {
            x.legacy_n_point_records as u64
        };

        let n_points_return = if x.legacy_n_point_return == [0u32; 5] {
            x.n_points_return
        } else {
            let y = &x.legacy_n_point_return;
            [
                y[0] as u64,
                y[1] as u64,
                y[2] as u64,
                y[3] as u64,
                y[4] as u64,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ]
        };

        let point_record_format = Format::try_from(x.point_record_format)?;

        Ok(RelevantHeader {
            point_record_format,
            offset_point_data: x.offset_point_data,
            point_record_length: x.point_record_length,
            n_point_records,
            n_points_return,
        })
    }
}

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

#[derive(Debug, Default)]
pub struct PointData {
    pub x: i32, //4 4
    pub y: i32, //4 8
    pub z: i32, //4 12
}

impl PointData {
    //@todo this can likely never fail, consider unwrapping AND in other places
    pub fn from_bytes(buffer: &[u8; 12]) -> LasResult<Self> {
        Ok(Self {
            x: i32::from_le_bytes(buffer[0..4].try_into()?),
            y: i32::from_le_bytes(buffer[4..8].try_into()?),
            z: i32::from_le_bytes(buffer[8..12].try_into()?),
        })
    }
}

#[derive(Debug, Default)]
pub struct ColorData {
    pub red: u16,   //2 2
    pub green: u16, //2 4
    pub blue: u16,  //2 6
}

impl ColorData {
    pub fn from_bytes(buffer: &[u8; 6]) -> LasResult<Self> {
        Ok(Self {
            red: u16::from_le_bytes(buffer[0..2].try_into()?),
            green: u16::from_le_bytes(buffer[2..4].try_into()?),
            blue: u16::from_le_bytes(buffer[4..6].try_into()?),
        })
    }
}

#[derive(Debug, Default)]
pub struct WaveData {
    pub wave_descriptor_index: u8,      //1 1
    pub offset_waveform_data: u64,      //8 9
    pub waveform_packet_size: u32,      //4 13
    pub return_point_waveform_loc: f32, //4 17
    pub dx: f32,                        //4 21
    pub dy: f32,                        //4 25
    pub dz: f32,                        //4 29
}

impl WaveData {
    pub fn from_bytes(buffer: &[u8; 29]) -> LasResult<Self> {
        Ok(Self {
            wave_descriptor_index: u8::from_le_bytes(buffer[0..1].try_into()?),
            offset_waveform_data: u64::from_le_bytes(buffer[1..9].try_into()?),
            waveform_packet_size: u32::from_le_bytes(buffer[9..13].try_into()?),
            return_point_waveform_loc: f32::from_le_bytes(buffer[13..17].try_into()?),
            dx: f32::from_le_bytes(buffer[17..21].try_into()?),
            dy: f32::from_le_bytes(buffer[21..25].try_into()?),
            dz: f32::from_le_bytes(buffer[25..29].try_into()?),
        })
    }
}

#[derive(Debug)]
pub enum Format {
    F0(Format0),
    F1(Format1),
    F2(Format2),
    F3(Format3),
    F4(Format4),
    F5(Format5),
    F6(Format6),
    F7(Format7),
    F8(Format8),
    F9(Format9),
    F10(Format10),
}

impl FromRead for Format {
    fn from_read<R>(&mut self, read: &mut R) -> LasResult<()>
    where
        R: Read,
    {
        match self {
            Self::F0(x) => x.from_read(read),
            Self::F1(x) => x.from_read(read),
            Self::F2(x) => x.from_read(read),
            Self::F3(x) => x.from_read(read),
            Self::F4(x) => x.from_read(read),
            Self::F5(x) => x.from_read(read),
            Self::F6(x) => x.from_read(read),
            Self::F7(x) => x.from_read(read),
            Self::F8(x) => x.from_read(read),
            Self::F9(x) => x.from_read(read),
            Self::F10(x) => x.from_read(read),
        }
    }
}

impl FormatGeneric for Format {
    fn point_data(&self) -> &PointData {
        match self {
            Self::F0(x) => x.point_data(),
            Self::F1(x) => x.point_data(),
            Self::F2(x) => x.point_data(),
            Self::F3(x) => x.point_data(),
            Self::F4(x) => x.point_data(),
            Self::F5(x) => x.point_data(),
            Self::F6(x) => x.point_data(),
            Self::F7(x) => x.point_data(),
            Self::F8(x) => x.point_data(),
            Self::F9(x) => x.point_data(),
            Self::F10(x) => x.point_data(),
        }
    }
}

impl TryFrom<u8> for Format {
    type Error = LasError;

    fn try_from(x: u8) -> LasResult<Self> {
        match x {
            0 => Ok(Self::F0(Format0::default())),
            1 => Ok(Self::F1(Format1::default())),
            2 => Ok(Self::F2(Format2::default())),
            3 => Ok(Self::F3(Format3::default())),
            4 => Ok(Self::F4(Format4::default())),
            5 => Ok(Self::F5(Format5::default())),
            6 => Ok(Self::F6(Format6::default())),
            7 => Ok(Self::F7(Format7::default())),
            8 => Ok(Self::F8(Format8::default())),
            9 => Ok(Self::F9(Format9::default())),
            10 => Ok(Self::F10(Format10::default())),
            _ => Err(LasError::UnknownFormat),
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct Format0 {
    pub point_data: PointData, //12 12
    pub intensity: u16,        //2 14
    pub bitdata: u8, //1 15 ReturnNumber 3 bits, NumberOfReturns 3 bits, ScanDirFlag 1 bit, EdgeOfFlightLine 1 bit
    pub classification: u8, //1 16
    pub scan_angle_rank: u8, //1 17 actually signed char
    pub user_data: u8, // 1 18
    pub point_source_id: u16, //2 20
}

impl FromRead for Format0 {
    fn from_read<R>(&mut self, read: &mut R) -> LasResult<()>
    where
        R: Read,
    {
        let mut buffer = [0u8; 20]; //@todo use mem::sizeof?
        read.read_exact(&mut buffer)?;

        self.point_data = PointData::from_bytes(buffer[1..12].try_into()?)?;
        self.intensity = u16::from_le_bytes(buffer[12..14].try_into()?);
        self.bitdata = u8::from_le_bytes(buffer[14..15].try_into()?);
        self.classification = u8::from_le_bytes(buffer[15..16].try_into()?);
        self.scan_angle_rank = u8::from_le_bytes(buffer[16..17].try_into()?);
        self.user_data = u8::from_le_bytes(buffer[17..18].try_into()?);
        self.point_source_id = u16::from_le_bytes(buffer[18..20].try_into()?);

        Ok(())
    }
}

impl FormatGeneric for Format0 {
    fn point_data(&self) -> &PointData {
        &self.point_data
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct Format1 {
    pub format_0: Format0, //20 20
    pub gps_time: f64,     //8 28
}

impl FromRead for Format1 {
    fn from_read<R>(&mut self, read: &mut R) -> LasResult<()>
    where
        R: Read,
    {
        self.format_0.from_read(read)?;

        let mut buffer = [0u8; 28 - 20];
        read.read_exact(&mut buffer)?;

        self.gps_time = f64::from_le_bytes(buffer);

        Ok(())
    }
}

impl FormatGeneric for Format1 {
    fn point_data(&self) -> &PointData {
        self.format_0.point_data()
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct Format2 {
    pub format_0: Format0,     //20 20
    pub color_data: ColorData, //6 26
}

impl FromRead for Format2 {
    fn from_read<R>(&mut self, read: &mut R) -> LasResult<()>
    where
        R: Read,
    {
        self.format_0.from_read(read)?;

        let mut buffer = [0u8; 26 - 20];
        read.read_exact(&mut buffer)?;

        self.color_data = ColorData::from_bytes(&buffer)?;

        Ok(())
    }
}

impl FormatGeneric for Format2 {
    fn point_data(&self) -> &PointData {
        self.format_0.point_data()
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct Format3 {
    pub format_1: Format1,     //28 28
    pub color_data: ColorData, //6 34
}

impl FromRead for Format3 {
    fn from_read<R>(&mut self, read: &mut R) -> LasResult<()>
    where
        R: Read,
    {
        self.format_1.from_read(read)?;

        let mut buffer = [0u8; 34 - 28];
        read.read_exact(&mut buffer)?;

        self.color_data = ColorData::from_bytes(&buffer)?;

        Ok(())
    }
}

impl FormatGeneric for Format3 {
    fn point_data(&self) -> &PointData {
        self.format_1.point_data()
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct Format4 {
    pub format_1: Format1,   //28 28
    pub wave_data: WaveData, //29 57
}

impl FromRead for Format4 {
    fn from_read<R>(&mut self, read: &mut R) -> LasResult<()>
    where
        R: Read,
    {
        self.format_1.from_read(read)?;

        let mut buffer = [0u8; 57 - 28];
        read.read_exact(&mut buffer)?;

        self.wave_data = WaveData::from_bytes(&buffer)?;

        Ok(())
    }
}

impl FormatGeneric for Format4 {
    fn point_data(&self) -> &PointData {
        self.format_1.point_data()
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct Format5 {
    pub format_3: Format3,   //34 34
    pub wave_data: WaveData, //29 63
}

impl FromRead for Format5 {
    fn from_read<R>(&mut self, read: &mut R) -> LasResult<()>
    where
        R: Read,
    {
        self.format_3.from_read(read)?;

        let mut buffer = [0u8; 63 - 34];
        read.read_exact(&mut buffer)?;

        self.wave_data = WaveData::from_bytes(&buffer)?;

        Ok(())
    }
}

impl FormatGeneric for Format5 {
    fn point_data(&self) -> &PointData {
        self.format_3.point_data()
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, Default)]
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

impl FromRead for Format6 {
    fn from_read<R>(&mut self, read: &mut R) -> LasResult<()>
    where
        R: Read,
    {
        let mut buffer = [0u8; 30]; //@todo use mem::sizeof?
        read.read_exact(&mut buffer)?;

        self.point_data = PointData::from_bytes(buffer[0..12].try_into()?)?;
        self.intensity = u16::from_le_bytes(buffer[12..14].try_into()?);
        self.bitdata = u16::from_le_bytes(buffer[14..16].try_into()?);
        self.classification = u8::from_le_bytes(buffer[16..17].try_into()?);
        self.user_data = u8::from_le_bytes(buffer[17..18].try_into()?);
        self.scan_angle = i16::from_le_bytes(buffer[18..20].try_into()?);
        self.point_source_id = u16::from_le_bytes(buffer[20..22].try_into()?);
        self.gps_time = f64::from_le_bytes(buffer[22..30].try_into()?);

        Ok(())
    }
}

impl FormatGeneric for Format6 {
    fn point_data(&self) -> &PointData {
        &self.point_data
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct Format7 {
    pub format_6: Format6,     //30 30
    pub color_data: ColorData, //6 36
}

impl FromRead for Format7 {
    fn from_read<R>(&mut self, read: &mut R) -> LasResult<()>
    where
        R: Read,
    {
        self.format_6.from_read(read)?;

        let mut buffer = [0u8; 36 - 30];
        read.read_exact(&mut buffer)?;

        self.color_data = ColorData::from_bytes(&buffer)?;

        Ok(())
    }
}

impl FormatGeneric for Format7 {
    fn point_data(&self) -> &PointData {
        self.format_6.point_data()
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct Format8 {
    pub format_7: Format7, //36 36
    pub nir: u16,          //2 38
}

impl FromRead for Format8 {
    fn from_read<R>(&mut self, read: &mut R) -> LasResult<()>
    where
        R: Read,
    {
        self.format_7.from_read(read)?;

        let mut buffer = [0u8; 38 - 36];
        read.read_exact(&mut buffer)?;

        self.nir = u16::from_le_bytes(buffer);

        Ok(())
    }
}

impl FormatGeneric for Format8 {
    fn point_data(&self) -> &PointData {
        self.format_7.point_data()
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct Format9 {
    pub format_6: Format6,   //30 30
    pub wave_data: WaveData, //29 59
}

impl FromRead for Format9 {
    fn from_read<R>(&mut self, read: &mut R) -> LasResult<()>
    where
        R: Read,
    {
        self.format_6.from_read(read)?;

        let mut buffer = [0u8; 59 - 30];
        read.read_exact(&mut buffer)?;

        self.wave_data = WaveData::from_bytes(&buffer)?;

        Ok(())
    }
}

impl FormatGeneric for Format9 {
    fn point_data(&self) -> &PointData {
        self.format_6.point_data()
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct Format10 {
    pub format_6: Format6,     //30 30
    pub color_data: ColorData, //6 36
    pub nir: u16,              //2 38
    pub wave_data: WaveData,   //29 67
}

impl FromRead for Format10 {
    fn from_read<R>(&mut self, read: &mut R) -> LasResult<()>
    where
        R: Read,
    {
        self.format_6.from_read(read)?;

        let mut buffer = [0u8; 67 - 30];
        read.read_exact(&mut buffer)?;

        self.color_data = ColorData::from_bytes(&buffer[0..6].try_into()?)?;
        self.nir = u16::from_le_bytes(buffer[6..8].try_into()?);
        self.wave_data = WaveData::from_bytes(&buffer[8..37].try_into()?)?;

        Ok(())
    }
}

impl FormatGeneric for Format10 {
    fn point_data(&self) -> &PointData {
        self.format_6.point_data()
    }
}

//------------------------------------------------------------------------------

/// Error type for .las file operation
#[derive(Debug)]
pub enum LasError {
    AccessFile,
    BinaryData,
    UnknownFormat,
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
