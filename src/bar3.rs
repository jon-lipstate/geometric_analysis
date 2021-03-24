#![allow(unused_variables)]
use nalgebra::{Dynamic, Matrix3, MatrixN};
use time::Instant;
//https://misoraclette.github.io/2018/08/04/data_manipulation.html

// type NxNMatrix = nalgebra::MatrixN<f32, U12>;
type DynMatrix = MatrixN<f32, Dynamic>;

fn main() {
    println!("1d quadraic element, first mode axial frequency. s/b PI/2.");
    let time = Instant::now();
    let n_elm:usize = 1;
    let restrained_dofs = [0];
    let mass_local = Matrix3::new(4.0, 2.0, -1.0,
                                  2.0, 16.0, 2.0,
                                 -1.0, 2.0,  4.0) / (30.0 * n_elm as f32);
    let stiffness_local = Matrix3::new( 7.0, -8.0,  1.0,
                                       -8.0, 16.0, -8.0,
                                        1.0, -8.0,  7.0) * 3.0 * n_elm as f32;      
                         
    // println!("Local mass Matrix: \n{:?}", mass_local.data);
    // println!("Local Stiffness Matrix: \n{:?}", stiffness_local.data);

    let mut mass_global = DynMatrix::zeros(2*n_elm+1,2*n_elm+1);
    let mut stiffness_global = DynMatrix::zeros(2*n_elm+1,2*n_elm+1);
    
    for elm in 0..n_elm {
        // println!("elm: {}", elm);
        for i in 0..3 {
            for j in 0..3 {
                // println!("i,j: {}, {}",i+2*elm, j+2*elm);
                mass_global[(i+(2*elm),j+(2*elm))] += mass_local[(i,j)];
                stiffness_global[(i+(2*elm),j+(2*elm))] += stiffness_local[(i,j)];
            }
        }
    }
    // println!("Global Mass Matrix: \n{:?}", mass_global.data);
    // println!("Global Stiffness Matrix: \n{:?}", stiffness_global);
    
    for dof in &restrained_dofs {
        mass_global = mass_global.remove_column(*dof);
        mass_global = mass_global.remove_row(*dof);
        stiffness_global = stiffness_global.remove_column(*dof);
        stiffness_global = stiffness_global.remove_row(*dof);
    }
    let res = nalgebra::SymmetricEigen::new(stiffness_global*mass_global);
    let f1 = res.eigenvalues.map(|i| f32::sqrt(i));
    println!("First Mode: {:?}", f1[0]);
    println!("Elapsed Duration: {:?}", time.elapsed());
}
