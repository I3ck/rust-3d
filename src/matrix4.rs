use std::f64;

use traits::HasPosition3D;

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
    fn perspective(width: f64, height: f64, close: f64, away: f64, fovRad: f64) -> Matrix4 {
        let ratio = width/height;
        let range = close - away;
        let tanFovHalf = (fovRad/2.0).tan();

        let mut result = Matrix4::new();
        result.data[0][0] = 1.0 / (tanFovHalf * away);  result.data[0][1] = 0.0;               result.data[0][2] = 0.0;                      result.data[0][3] = 0.0;
        result.data[1][0] = 0.0;                        result.data[1][1] = 1.0 / tanFovHalf;  result.data[1][2] = 0.0;                      result.data[1][3] = 0.0;
        result.data[2][0] = 0.0;                        result.data[2][1] = 0.0;               result.data[2][2] = (-close - away) / range;  result.data[2][3] = 2.0 * away * close / range;
        result.data[3][0] = 0.0;                        result.data[3][1] = 0.0;               result.data[3][2] = 1.0;                      result.data[3][3] = 0.0;
        result
    }
    fn look_at<P>(target: &P, up: &P) -> Option<Matrix4> where P: HasPosition3D { //@todo wont have to be an option once unitvector is defined whis is always l > 0 ( l == 1)
      let N = match target.clone().normalized() {
          None => return None,
          Some(x) => x
      };
      let U = match up.clone().normalized() {
          None => return None,
          Some(x) => *(x.cross(target))
      };
      let V = N.cross(&U);

      let mut result = Matrix4::new();
      result.data[0][0] = U.x();  result.data[0][1] = U.y();  result.data[0][2] = U.z();  result.data[0][3] = 0.0;
      result.data[1][0] = V.x();  result.data[1][1] = V.y();  result.data[1][2] = V.z();  result.data[1][3] = 0.0;
      result.data[2][0] = N.x();  result.data[2][1] = N.y();  result.data[2][2] = N.z();  result.data[2][3] = 0.0;
      result.data[3][0] = 0.0;  result.data[3][1] = 0.0;  result.data[3][2] = 0.0;  result.data[3][3] = 1.0;
      Some(result)
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
