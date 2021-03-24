#![allow(unused_variables)]
use nalgebra::{Dynamic, Matrix2, MatrixN};

//https://misoraclette.github.io/2018/08/04/data_manipulation.html

// type NxNMatrix = nalgebra::MatrixN<f32, U12>;
type DynMatrix = MatrixN<f32, Dynamic>;

fn main() {
    println!("1d linear element, first mode axial frequency. s/b PI/2.");
    let n_elm:usize = 2;

    let restrained_dofs = [0];

    let m = Matrix2::new(2.0, 1.0,
                         1.0, 2.0) / (6.0 * n_elm as f32);
    let k = Matrix2::new( 1.0, -1.0,
                         -1.0,  1.0) * n_elm as f32;      
                         
    println!("Local mass Matrix: \n{:?}", m.data);
    println!("Local Stiffness Matrix: \n{:?}", k.data);

    let mut mass_global = DynMatrix::zeros(n_elm+1,n_elm+1);
    let mut stiffness_global = DynMatrix::zeros(n_elm+1,n_elm+1);
    
    for elm in 0..n_elm {
        for i in 0..2 {
            for j in 0..2 {
                // println!("i,j: {}, {}",i+elm, j+elm);
                mass_global[(i+elm,j+elm)] += m[(i,j)];
                stiffness_global[(i+elm,j+elm)] += k[(i,j)];
            }
        }
    }
    println!("Global Mass Matrix: \n{:?}", mass_global.data);
    println!("Global Stiffness Matrix: \n{:?}", stiffness_global);
    
    for dof in &restrained_dofs {
        mass_global = mass_global.remove_column(*dof);
        mass_global = mass_global.remove_row(*dof);
        stiffness_global = stiffness_global.remove_column(*dof);
        stiffness_global = stiffness_global.remove_row(*dof);
    }
    let res = nalgebra::SymmetricEigen::new(stiffness_global*mass_global);
    let f1 = res.eigenvalues.map(|i| f32::sqrt(i));
    println!("RESULTS: \n{:?}", f1);
}
