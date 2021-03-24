#![allow(unused_variables, unused_imports, dead_code)]
use nalgebra::{Dynamic, Matrix2, Matrix2x4, Matrix4, Matrix4x2, MatrixN, SymmetricEigen, Vector2};
use plotters::prelude::*;
use time::Instant;

type DynMatrix = MatrixN<f32, Dynamic>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Truss FE Model");
    let time = Instant::now();

    /* #region Problem Setup:*/
    //1. Define Coordinate System
    let x_axis: Vector2<f32> = Vector2::new(1., 0.);
    let y_axis: Vector2<f32> = Vector2::new(0., 1.);
    let nodes: Vec<Vector2<f32>> = vec![
        Vector2::new(0f32, 10f32),
        Vector2::new(0f32, 0f32),
        Vector2::new(10f32, 5f32),
    ];
    //dofs defined for each node:
    let degrees_of_freedom: Vec<Vector2<usize>> =
        vec![Vector2::new(1, 2), Vector2::new(3, 4), Vector2::new(5, 6)];
    let elms = vec![(0, 2), (1, 2)];
    let mut restrained_dofs = vec![1, 2, 3, 4];
    let forces: Vec<Vector2<f32>> = vec![
        Vector2::new(0f32, 0f32),
        Vector2::new(0f32, 0f32),
        Vector2::new(0f32, -200f32),
    ];
    //Materials: AISI-1095, Carbon Steel (Spring Steel)
    let densities = vec![0.284, 0.284];
    let moduli = vec![3.0E7f32, 3.0E7f32];
    let areas = vec![1.0f32, 2.0f32];
    let n_dofs = 2 * nodes.len();

    assert_eq!(elms.len(), densities.len());
    assert_eq!(elms.len(), moduli.len());
    assert_eq!(elms.len(), areas.len());
    assert_eq!(elms.len() + 1, nodes.len());
    /* #endregion */

    /* #region Global Matrices:*/
    let mut m_global: DynMatrix = DynMatrix::zeros(n_dofs, n_dofs);
    let mut k_global: DynMatrix = DynMatrix::zeros(n_dofs, n_dofs);

    let mut index = 0;
    for elm in elms {
        //Find Geometry:
        let node_from: Vector2<f32> = nodes[elm.0];
        let node_to: Vector2<f32> = nodes[elm.1];
        let e_vec: Vector2<f32> = node_to - node_from;
        let length = e_vec.magnitude();

        let rho = densities[index];
        let area = areas[index];
        let modulus = moduli[index];

        let cm = rho * area * length / 6.0;
        let ck = modulus * area / length;

        let m: Matrix2<f32> = Matrix2::new(2.0, 1.0, 1.0, 2.0);
        let k: Matrix2<f32> = Matrix2::new(1.0, -1.0, -1.0, 1.0);

        //Find Rotated Matrices:
        let tau: Matrix2x4<f32> = rotation_matrix(e_vec, x_axis, y_axis);
        let m_r: Matrix4<f32> = tau.transpose() * m * tau;
        let k_r: Matrix4<f32> = tau.transpose() * k * tau;

        //Change from element to global coordinates:
        //Find DOFs for each node:
        let dofs: Vec<usize> = vec![
            degrees_of_freedom[elm.0].x,
            degrees_of_freedom[elm.0].y,
            degrees_of_freedom[elm.1].x,
            degrees_of_freedom[elm.1].y,
        ];
        let mut b: DynMatrix = DynMatrix::zeros(4, n_dofs);
        for i in 0..4 {
            b[(i, dofs[i] - 1)] = 1.0; //-1 to get to zero-index
        }
        let b_t: DynMatrix = b.transpose();
        let mr_g: DynMatrix = &b_t * m_r * &b;
        let kr_g: DynMatrix = &b_t * k_r * &b;
        m_global += mr_g;
        k_global += kr_g;

        index += 1;
    }

    //construct force vector:
    let mut f_global: Vec<f32> = Vec::new();
    for f in &forces {
        f_global.push(f.x);
        f_global.push(f.y);
    }

    //remove retrained dofs:

    let mut removed_indicies: Vec<usize> = restrained_dofs.into_iter().map(|i| i - 1).collect();
    removed_indicies.sort();

    for i in removed_indicies.iter().rev() {
        m_global = m_global.remove_column(*i);
        m_global = m_global.remove_row(*i);
        k_global = k_global.remove_column(*i);
        k_global = k_global.remove_row(*i);
    }

    println!("{:?}", m_global);
    println!("{:?}", k_global);
    println!("{:?}", f_global);

    /* #endregion */
    /* #region Natural Frequencies:*/
    /* #endregion */
    /* #region Calculate Static Displacements:*/
    /* #endregion */
    /* #region Determine Stresses:*/

    /* #endregion */

    Ok(())
}

fn rotation_matrix(
    e_vec: Vector2<f32>,
    x_axis: Vector2<f32>,
    y_axis: Vector2<f32>,
) -> Matrix2x4<f32> {
    let x_proj = dir_cos(e_vec, x_axis);
    let y_proj = dir_cos(e_vec, y_axis);
    return Matrix2x4::new(x_proj, y_proj, 0., 0., 0., 0., x_proj, y_proj);
}

fn dir_cos(v1: Vector2<f32>, v2: Vector2<f32>) -> f32 {
    return v1.dot(&v2) / (v1.norm() * v2.norm());
}

pub struct Node {
    id: usize,
    x: f32,
    y: f32,
}
impl Node {
    pub fn new(id: usize, x: f32, y: f32) -> Self {
        Self { id, x, y }
    }
}
pub struct Rod {
    id: usize,
    node_a: usize,
    node_b: usize,
    modulus: f32,
    density: f32,
    area: f32,
}
impl Rod {
    pub fn new(
        id: usize,
        node_a: usize,
        node_b: usize,
        modulus: f32,
        density: f32,
        area: f32,
    ) -> Self {
        Self {
            id,
            node_a,
            node_b,
            modulus,
            density,
            area,
        }
    }
    pub fn get_vector(&self, node_list: &Vec<Node>) -> Vector2<f32> {
        let na = node_list.iter().find(|n| n.id == self.node_a).unwrap();
        let nb = node_list.iter().find(|n| n.id == self.node_b).unwrap();
        return Vector2::new(nb.x - na.x, nb.y - na.y);
    }
    pub fn x() {}
}
