use matrix4::Matrix4;
use traits::has_position_3d::HasPosition3D;

pub struct Matrix4Pipe {
    pub mtranslation: Matrix4,
    pub mrotation: Matrix4,
    pub mscale: Matrix4,
    pub mperspective: Matrix4,
    pub mcamtrans: Matrix4,
    pub mcamlook: Matrix4
}

impl Matrix4Pipe {
    fn new() -> Matrix4Pipe {
        Matrix4Pipe {
            mtranslation: Matrix4::new(),
            mrotation: Matrix4::new(),
            mscale: Matrix4::new(),
            mperspective: Matrix4::new(),
            mcamtrans: Matrix4::new(),
            mcamlook: Matrix4::new()
        }
    }
    //@todo might be inversed order
    //@todo better overload operator * for Matrix4 to gain nicer syntax
    fn result(&self) -> Matrix4 {
        self.mperspective
            .multiplyM(&self.mcamlook
                .multiplyM(&self.mcamtrans
                    .multiplyM(&self.mtranslation
                        .multiplyM(&self.mrotation
                            .multiplyM(&self.mscale)))))
    }
    fn add_translation(&mut self, x: f64, y: f64, z: f64) {
        self.mtranslation = Matrix4::translation(x, y, z);
    }
    fn remove_translation(&mut self) {
        self.mtranslation = Matrix4::new();
    }

    fn add_rotation(&mut self, radX: f64, radY: f64, radZ: f64) {
        self.mrotation = Matrix4::rotation(radX, radY, radZ);
    }
    fn add_rotation_axis<P>(&mut self, axis: &P, rad: f64) -> bool where P: HasPosition3D {
        match Matrix4::rotation_axis(axis, rad) {
            None => return false,
            Some(m) => { self.mrotation = m; return true; }
        }
    }
    fn remove_rotation(&mut self) {
        self.mrotation = Matrix4::new();
    }

    fn add_scale(&mut self, x: f64, y: f64, z: f64) {
        self.mscale = Matrix4::scale(x, y, z);
    }
    fn remove_scale(&mut self) {
        self.mscale = Matrix4::new();
    }

    fn add_perspective(&mut self, width: f64, height: f64, close: f64, away: f64, fovRad: f64) {
        self.mperspective = Matrix4::perspective(width, height, close, away, fovRad);
    }
    fn remove_perspective(&mut self) {
        self.mperspective = Matrix4::new();
    }

    fn add_camera_translation(&mut self, x: f64, y: f64, z: f64) {
        self.mcamtrans = Matrix4::translation(-x, -y, -z);
    }
    fn remove_camera_translation(&mut self) {
        self.mcamtrans = Matrix4::new();
    }

    fn add_look_at<P>(&mut self, target: &P, up: &P) -> bool where P: HasPosition3D {
        match Matrix4::look_at(target, up) {
            None => return false,
            Some(m) => { self.mcamlook = m; return true; }
        }
    }
    fn remove_look_at(&mut self) {
        self.mcamlook = Matrix4::new();
    }


}
