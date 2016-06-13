rust-3d
=======
3D/2D library written in Rust.
Offering useful containers, structures and algorithms for 2D and 3D space.
Meant as basis for numeric algorithms, viewers, game engines, ...

notes
-----
rust-3d is still in really early stages, and is likely to change A LOT over time.
Feel free to open an issue in case you're missing something or found a bug.
Also some code might be untested, I am working on 100% test coverage, tho.
All classes / algorithms of https://github.com/I3ck/lib_2d will be ported into the 2d part.
Once this is fairly well tested, I'll also add it to crates.




traits
-----
```rust
Is2D //anything defined by a position in 2D space
Is3D //anything defined by a position in 3D space
IsBuildable2D //2D and constructible
IsBuildable3D //3D and constructible
IsEditable2D //2D and modifiable
IsEditable3D //3D and modifiable
IsNormalized2D //normalized vectors in 2D space
IsNormalized3D //normalized vectors in 3D space
IsMoveable2D //anything movable within the 2D space
IsMoveable3D //anything movable within the 3D space
HasBoundingBox2D //having a size and position in 2D space
HasBoundingBox3D //having a size and position in 3D space
IsMesh //3-vertex mesh in 3D space
IsEditableMesh //3-vertex mesh in 3D space which is editable
IsTree3D //tree structures for 3D
IsKdTree3D //KdTree for 3D space
IsOctree //Octree
IsPlane3D //a 2D plane within 3D space
IsProjectionToPlane //something projected to a 2D plane
TransformableTo2D //something which can be transformed / projected into 2D space (for projections onto planes)
TransformableTo3D //something which can be transformed / extruded into 3D space (for extrusions from planes)
```
Most algorithms are defined for these traits, so feel free to implement them and use the algorithms on your own types.
Although implementations are provided as well:


implementations
---------------
```rust
Point2D //a point in 2D space
Point3D //a point in 3D space
Norm2D //normalized vector in 2D space
Norm3D //normalized vector in 3D space
Plane3D //a 2D plane in 3D space
KdTree //a kdtree in 3D space [nearest, k-nearst, in-box, in-sphere]
OcTree //an octree
Matrix4 //a 4x4 matrix with several builder methods [scale matrix, rotation matrix, translation matrix, ...]
Matrix4Pipe //a 4x4 matrix pipe to easily chain matrix operations into a single matrix
Mesh3D //a mesh in 3D space holding a 3d point cloud
PointCloud2D //a point cloud holding 2d positions
PointCloud3D //a point cloud holding 3d positions
ProjectionToPlane //a projection onto a plane
```

examples
--------
Please take a look at the tests in src/lib.rs.
Currently main.rs also holds some example code, but will be fully moved to the tests.


contribute
----------
Feel free to open issues at any time to report bugs / request features. (but please check TODO first)

license
------
LGPL (see LICENSE)
