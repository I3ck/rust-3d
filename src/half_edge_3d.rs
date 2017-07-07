/*
Copyright 2017 Martin Buck
This file is part of rust-3d.
rust-3d is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
rust-3d is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.
You should have received a copy of the GNU Lesser General Public License
along with rust-3d.  If not, see <http://www.gnu.org/licenses/>.
*/

//! HalfEdge3D, the half edge data structure for 3D

use result::*;
use strong_types::*;

/// HalfEdge3D, the half edge data structure for 3D
pub struct HalfEdge3D {
    edges: Vec<Edge>,
    vertices_start_edges: Vec<Vec<EId>> //@todo better name
}

//@todo info text (file also has to note it includes both, OR move to own file)
//@todo docs for pub methods
pub struct Edge {
    tail: VId,
    twin: Option<EId>
}

impl HalfEdge3D {

    pub fn tail(&self, id: EId) -> Result<VId> {
        self.ensure_edge_id(id)?;
        Ok(self.edges[id.val].tail.clone())
    }

    pub fn face(&self, id: EId) -> Result<FId> {
        self.ensure_edge_id(id)?;
        Ok(FId{val: id.val / 3})
    }

    pub fn twin(&self, id: EId) -> Result<Option<EId>> {
        self.ensure_edge_id(id)?;
        Ok(self.edges[id.val].twin.clone())
    }

    pub fn next(&self, id: EId) -> Result<EId> {
        self.ensure_edge_id(id)?;
        if Self::last_in_face(id) {
            return Ok(EId{val: id.val - 2});
        }
        Ok(EId{val: id.val + 1})
    }

    pub fn edges_originating(&self, id: VId) -> Result<Vec<EId>> {
        self.ensure_vertex_id(id)?;
        Ok(self.vertices_start_edges[id.val].clone())
    }

    pub fn edges_ending(&self, id: VId) -> Result<Vec<EId>> {
        let originatings = self.edges_originating(id)?;
        let mut result = Vec::with_capacity(originatings.len());
        for edge in originatings {
            match self.prev(edge) {
                Err(_) => {},
                Ok(id) => result.push(id)
            }
        }
        Ok(result)
    }

    pub fn edges_all(&self, id: VId) -> Result<Vec<EId>> {
        let originatings = self.edges_originating(id)?;
        let mut result = Vec::with_capacity(originatings.len());
        for edge in originatings {
            result.push(edge);
            match self.prev(edge) {
                Err(_) => {},
                Ok(id) => result.push(id)
            }
        }
        Ok(result)
    }

    pub fn faces(&self, id: VId) -> Result<Vec<FId>> {
        let originatings = self.edges_originating(id)?;
        let mut result = Vec::with_capacity(originatings.len());
        for edge in originatings {
            match self.face(edge) {
                Err(_) => {}
                Ok(id) => result.push(id)
            }
        }
        Ok(result)
    }

    pub fn prev(&self, id: EId) -> Result<EId> {
        self.ensure_edge_id(id)?;
        if Self::first_in_face(id) {
            return Ok(EId{val: id.val + 2});
        }
        Ok(EId{val: id.val - 1})
    }


    fn first_in_face(id: EId) -> bool {
        id.val % 3 == 0
    }

    fn last_in_face(id: EId) -> bool {
        id.val % 3 == 2
    }

    fn ensure_edge_id(&self, id: EId) -> Result<()> {
        if id.val >= self.edges.len() { //@todo could cache len later if edges never changes
            return Err(ErrorKind::IncorrectFaceID); //@todo IncorrectEdgeID
        }
        Ok(())
    }

    fn ensure_vertex_id(&self, id: VId) -> Result<()> {
        if id.val >= self.vertices_start_edges.len() { //@todo could cache len later if never changes
            return Err(ErrorKind::IncorrectFaceID); //@todo IncorrectVId
        }
        Ok(())
    }


}

