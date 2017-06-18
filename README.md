rust-3d
=======
3D/2D library written in Rust.
Offering useful containers, structures and algorithms for 2D and 3D space.
Meant as basis for numeric algorithms, viewers, game engines, ...


notes
-----
rust-3d is still in really early stages, and is likely to change A LOT over time.
The code coverage is also pretty bad right now, so there might be bugs.


traits / implementations / algorithms
-------------------------------------
There's traits for basically any type. `Is2D`, `IsFilter`, `IsPlane3D`, `IsEditableMesh`, `IsRandomAccessible<Is3D>`, ...  
Most algorithms are defined on those traits, but `rust-3d` also provides implementations for all the traits.  
Therefore it is easy to get started with the provided types while still offering the possibility to replace them with your own.  
Please check the [documentation](https://docs.rs/rust-3d/) for the most recent and extensive info.  


examples
--------
Please take a look at the tests in `tests/`. These will be up-to-date and compiling.  
I might add extensive tutorials / examples in the future.


links
-----
[crates.io](https://crates.io/crates/rust-3d)  
[github.com](https://github.com/I3ck/rust-3d)  
[docs.rs](https://docs.rs/rust-3d/)


contribute
----------
Feel free to open an issue in case you're missing something or found a bug.
Please avoid directly contributing since I might be working on breaking changes.
Open an issue or email me beforehand, to save you some trouble.


license
------
LGPL (see LICENSE)
