/*
Copyright 2017 Martin Buck

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

#![deny(warnings)]

use rust_3d::{io::*, test_helper::*, *};

use std::{fs::File, io::BufReader};

static GENERATE_EXCEPTED_RESULT_FILES: bool = false;

#[test]
fn point_cloud_3d_io_test() {
    {
        let path_expected = "tests/data/expected_pc_3d_save1.csv";
        let path_tmp = "tests/tmp/pc_3d_save1.tmp";
        let mut pc = PointCloud3D::<Point3D>::new();

        for i in 0..10 {
            let p = Point3D::new(0.1 * i as f64, 0.2 * i as f64, 0.3 * i as f64);
            pc.push(p);
        }

        if GENERATE_EXCEPTED_RESULT_FILES {
            save_xyz(&mut File::create(&path_expected).unwrap(), &pc, ";", "\n").unwrap();
        }

        save_xyz(&mut File::create(&path_tmp).unwrap(), &pc, ";", "\n").unwrap();

        assert_files_equal(path_expected, path_tmp);
    }

    {
        //@todo also compare values
        let mut pc = PointCloud3D::<Point3D>::new();
        load_xyz(
            &mut BufReader::new(File::open("tests/data/test_cube.xyz").unwrap()),
            &mut pc,
        )
        .unwrap();
        assert!(pc.len() == 20 * 20 * 20);
    }
}
