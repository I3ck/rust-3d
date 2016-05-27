use std::f64;

pub struct Matrix4 {
    pub data: [[f64; 4]; 4]
}

impl Matrix4 {
    fn new() -> Matrix4 {
        Matrix4{
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }
    fn zeroes() -> Matrix4 {
        Matrix4{
            data: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0]
            ]
        }
    }
    fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
        Matrix4{
            data: [
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }
    fn scale(x: f64, y: f64, z: f64) -> Matrix4 {
        Matrix4{
            data: [
                [x,   0.0, 0.0, 0.0],
                [0.0, y,   0.0, 0.0],
                [0.0, 0.0, z,   0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }




    fn rotation(radX: f64, radY: f64, radZ: f64) -> Matrix4 {
        let (mut mX, mut mY, mut mZ) = (Matrix4::new(), Matrix4::new(), Matrix4::new());

        mX.data[0][0] = 1.0;     mX.data[0][1] = 0.0;           mX.data[0][2] = 0.0;            mX.data[0][3] = 0.0;
        mX.data[1][0] = 0.0;     mX.data[1][1] = radX.cos();    mX.data[1][2] = -radX.sin();    mX.data[1][3] = 0.0;
        mX.data[2][0] = 0.0;     mX.data[2][1] = radX.sin();    mX.data[2][2] = radX.cos();     mX.data[2][3] = 0.0;
        mX.data[3][0] = 0.0;     mX.data[3][1] = 0.0;           mX.data[3][2] = 0.0;            mX.data[3][3] = 1.0;

        mY.data[0][0] = radY.cos();     mY.data[0][1] = 0.0;      mY.data[0][2] = radY.sin();   mY.data[0][3] = 0.0;
        mY.data[1][0] = 0.0;            mY.data[1][1] = 1.0;      mY.data[1][2] = 0.0;          mY.data[1][3] = 0.0;
        mY.data[2][0] = -radY.sin();    mY.data[2][1] = 0.0;      mY.data[2][2] = radY.cos();   mY.data[2][3] = 0.0;
        mY.data[3][0] = 0.0;            mY.data[3][1] = 0.0;      mY.data[3][2] = 0.0;          mY.data[3][3] = 1.0;

        mZ.data[0][0] = radZ.cos(); mZ.data[0][1] = -radZ.sin();    mZ.data[0][2] = 0.0;      mZ.data[0][3] = 0.0;
        mZ.data[1][0] = radZ.sin(); mZ.data[1][1] = radZ.cos();     mZ.data[1][2] = 0.0;      mZ.data[1][3] = 0.0;
        mZ.data[2][0] = 0.0;        mZ.data[2][1] = 0.0;            mZ.data[2][2] = 1.0;      mZ.data[2][3] = 0.0;
        mZ.data[3][0] = 0.0;        mZ.data[3][1] = 0.0;            mZ.data[3][2] = 0.0;      mZ.data[3][3] = 1.0;

        mX.multiplyM(&mY.multiplyM(&mZ))
    }











    fn multiplyM(&self, other: &Matrix4) -> Matrix4 {
        let mut result = Matrix4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] =
                    self.data[i][0] * other.data[0][j] +
				    self.data[i][1] * other.data[1][j] +
				    self.data[i][2] * other.data[2][j] +
				    self.data[i][3] * other.data[3][j];
            }
        }
        result
    }

    fn multiplyF(&self, other: f64) -> Matrix4 {
        let mut result = Matrix4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = other * self.data[i][j];
            }
        }
        result
    }
}
