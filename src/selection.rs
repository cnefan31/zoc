use core::unit::{UnitId};
use core::game_state::{State};
use core::dir::{dirs};
use geom;
use fs;
use scene::{Scene, SceneNode, NodeId};
use context::{Context};
use texture::{load_texture};
use types::{WorldPos};
use mesh::{MeshId};
use mesh::{Mesh};
use pipeline::{Vertex};
use unit_type_visual_info::{UnitTypeVisualInfoManager};

#[derive(Clone, Debug)]
pub struct SelectionManager {
    unit_id: Option<UnitId>,
    mesh_id: MeshId,
    selection_marker_node_id: Option<NodeId>,
}

impl SelectionManager {
    pub fn new(mesh_id: MeshId) -> SelectionManager {
        SelectionManager {
            unit_id: None,
            mesh_id: mesh_id,
            selection_marker_node_id: None,
        }
    }

    fn get_pos(&self, state: &State) -> WorldPos {
        let unit_id = self.unit_id
            .expect("Can`t get pos if no unit is selected");
        let map_pos = state.unit(unit_id).pos;
        WorldPos{v: geom::lift(geom::exact_pos_to_world_pos(state, map_pos).v)}
    }

    // TODO: учитывать размер выделенного отряда!
    pub fn create_selection_marker(
        &mut self,
        visual_info: &UnitTypeVisualInfoManager,
        state: &State,
        scene: &mut Scene,
        unit_id: UnitId,
    ) {
        let unit = state.unit(unit_id);
        self.unit_id = Some(unit_id);
        if let Some(node_id) = self.selection_marker_node_id {
            if scene.nodes().get(&node_id).is_some() {
                scene.remove_node(node_id);
            }
        }
        let size = visual_info.get(unit.type_id).size;
        let node = SceneNode {
            pos: self.get_pos(state),
            mesh_id: Some(self.mesh_id),
            scale: size * 0.5, // TODO: подобрать размеры получше
            // TODO: еще смущает что толщина линии меняется :(
            .. Default::default()
        };
        self.selection_marker_node_id = Some(scene.add_node(node));
    }

    pub fn deselect(&mut self, scene: &mut Scene) {
        self.unit_id = None;
        if let Some(node_id) = self.selection_marker_node_id {
            scene.remove_node(node_id);
        }
        self.selection_marker_node_id = None;
    }
}

pub fn get_selection_mesh(context: &mut Context) -> Mesh {
    let texture_data = fs::load("shell.png").into_inner();
    let texture = load_texture(context, &texture_data);
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let scale_1 = 0.6;
    let scale_2 = scale_1 + 0.05;
    let mut i = 0;
    for dir in dirs() {
        let dir_index = dir.to_int();
        let vertex_1_1 = geom::index_to_hex_vertex_s(scale_1, dir_index);
        let vertex_1_2 = geom::index_to_hex_vertex_s(scale_2, dir_index);
        let vertex_2_1 = geom::index_to_hex_vertex_s(scale_1, dir_index + 1);
        let vertex_2_2 = geom::index_to_hex_vertex_s(scale_2, dir_index + 1);
        vertices.push(Vertex{pos: vertex_1_1.v.into(), uv: [0.0, 0.0]});
        vertices.push(Vertex{pos: vertex_1_2.v.into(), uv: [0.0, 1.0]});
        vertices.push(Vertex{pos: vertex_2_1.v.into(), uv: [1.0, 0.0]});
        vertices.push(Vertex{pos: vertex_2_2.v.into(), uv: [1.0, 1.0]});
        indices.extend_from_slice(&[i, i + 1, i + 2, i + 1, i + 2, i + 3]);
        i += 4;
    }
    Mesh::new(context, &vertices, &indices, texture)
}
