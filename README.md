rust-3d
=======
3D/2D library written in Rust.
Offering useful containers, structures and algorithms for 2D and 3D space.
Meant as basis for numeric algorithms, viewers, game engines, ...


Notes
-----
`rust-3d` is still in really early stages, there might come breaking changes with each update.  
The test coverage is far from perfect, so you might find some bugs (please report them).  
Compiling with `stable`.
 
 
Tour
----
Here's a little overview of some of `rust-3d`'s features. 
The snippets / names might not be up-to-date, so please check `tests/` for compiling examples.
 
 
### Proper error handling
No `.unwrap()` where it's not 100% safe.
 
### Strong / Smart Types
There's strong types for everything that might get mixed up easily.  
This way e.g. ids of faces can't be mistaken for ids of vertices.
```rust
fn edges_of_face(&self, faceid: FId) -> Result<(EId, EId, EId)>;
```
There's also smart types which restrict the values they can hold.  
This way distances can never be `< 0.0`, sizes can be enfored to be `> 0.0` etc.
```rust
Positive  
NonNegative
```
  
### Generic Code Base
I try and keep all algorithms and types as generic as possible (see `/src/traits`).
- Even rather basic types like `Is2D` are split into several versions: `IsEditable2D`, `IsBuildable2D`
- `IsMesh` is defined for any vertex type and any number of vertices / face
- There's traits for collections (no need to use `Vec`)  
  
This makes it possible to require as little implementation work as possible if you want to use your own types.  
  
  
### Combinators / Transformers
- Any `IsFilter<T>` can be combined via `FilterAND`, `FilterOR`, `FilterAny`, `FilterNegate`...  
- Any `IsFilter<T>` can be transformed to work for any collection of `T`s (`IsFilterRandomAccessible`).
- `IsDirectionField2D` might be transformed to an `IsFilter<Is2D>`, which can then be transformed to an `IsFilterRandomAccessible<Is2D>`.
  
  
### IO
Any `IO` method is defined on traits, so if you implement these, you'll get read/write of different file formats for free.
 

Documentation
-------------
The documentation is quite good already, come and [take a look](https://docs.rs/rust-3d/).


Examples
--------
Please take a look at the tests in `tests/`. These will be up-to-date and compiling.  
I might add extensive tutorials / examples / demo projects in the future.


Links
-----
[crates.io](https://crates.io/crates/rust-3d)  
[github.com](https://github.com/I3ck/rust-3d)  
[docs.rs](https://docs.rs/rust-3d/)


Contribute
----------
Feel free to open an issue in case you're missing something or found a bug.
Please avoid directly contributing since I might be working on breaking changes or the feature you want to implement.
Open an issue or email me beforehand.


License
------
LGPL (see LICENSE)
