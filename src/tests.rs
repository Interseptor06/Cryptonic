#[cfg(test)]
mod test_matrix_functionality {
    use core::prelude;
    use crate::errors::MatrixError;
    use crate::matrix;
    use crate::layout::Layout;
    use crate::matrix::Matrix;

    #[test]
    fn test_row_major_gen() {
        let mat: Matrix<i32> = Matrix::new(vec![3, 4], Layout::RowMajor);

        // Tests that the instantiation succeeds and that the strides are generated properly
        assert_eq!(mat.strides, vec![4, 1]);
    }

    #[test]
    fn test_col_major_gen() {
        let mat: Matrix<i32> = Matrix::new(vec![3, 4], Layout::ColumnMajor);

        // Tests that the instantiation succeeds and that the strides are generated properly
        assert_eq!(mat.strides, vec![1, 3]);
    }

    #[test]
    fn test_size_calc() {
        // Tests that the size is calculated correctly
        // Important as it's used in many other methods
        let mat: Matrix<i32> = Matrix::new(vec![3, 4, 7], Layout::ColumnMajor);
        assert_eq!(mat.size(), 3*4*7);

        let mat: Matrix<i32> = Matrix::new(vec![12, 23, 21, 1, 21], Layout::RowMajor);
        assert_eq!(mat.size(), 12*23*21*1*21);

        let mat: Matrix<i32> = Matrix::new(vec![21, 37, 12], Layout::ColumnMajor);
        assert_eq!(mat.size(), 21*37*12);

    }

    #[test]
    fn test_shape() {
        // Tests that the size is calculated correctly
        // Important as it's used in many other methods
        let mat: Matrix<i32> = Matrix::new(vec![3, 4, 7], Layout::ColumnMajor);
        assert_eq!(mat.shape(), &vec![3, 4, 7]);

        let mat: Matrix<i32> = Matrix::new(vec![21, 44, 34, 32, 12], Layout::RowMajor);
        assert_eq!(mat.shape(), &vec![21, 44, 34, 32, 12]);

        let mat: Matrix<i32> = Matrix::new(vec![32, 37, 12], Layout::ColumnMajor);
        assert_eq!(mat.shape(), &vec![32, 37, 12]);

    }

    #[test]
    fn test_reshape() {
        let mut mat: Matrix<i32> = Matrix::new(vec![100], Layout::RowMajor);

        assert_eq!(Err(MatrixError::ReshapeError), mat.reshape(&vec![20, 6]));
        let l = mat.reshape(&vec![20, 5]);
        assert_eq!(mat.shape(), &vec![20, 5]);
    }


    #[test]
    fn test_bound_check() {
        let mat_1: Matrix<i32> = Matrix::new(vec![3, 4], Layout::RowMajor);
        let mat_2: Matrix<i32> = Matrix::new(vec![10, 20, 30], Layout::ColumnMajor);
        assert_eq!(Err(MatrixError::OutOfBounds), mat_1.check_bounds(&vec![3, 4]));
        assert_eq!(Ok(()), mat_1.check_bounds(&vec![2, 3]));

        assert_eq!(Err(MatrixError::OutOfBounds), mat_2.check_bounds(&vec![3, 4, 832]));
        assert_eq!(Ok(()), mat_2.check_bounds(&vec![2, 3, 2]));
    }

    #[test]
    fn test_strides() {
        // Tests that the size is calculated correctly
        // Important as it's used in many other methods
        let mat: Matrix<i32> = Matrix::new(vec![3, 4, 7], Layout::ColumnMajor);
        assert_eq!(mat.strides(), &vec![1, 3, 12]);

        let mat: Matrix<i32> = Matrix::new(vec![100, 100, 100], Layout::RowMajor);
        assert_eq!(mat.strides(), &vec![10000, 100, 1]);

        let mat: Matrix<i32> = Matrix::new(vec![10, 10, 10, 10, 10, 10], Layout::ColumnMajor);
        assert_eq!(mat.strides(), &vec![1, 10, 100, 1000, 10000, 100000]);

    }

    #[test]
    fn test_get_physical_idx() {
        let mat: Matrix<i32> = Matrix::new(vec![4, 3, 7], Layout::RowMajor);

        let (x, y, z) = (4, 0, 0);
        // Expect error  to be thrown if index out of bounds
        let expected_error = Err(MatrixError::OutOfBounds);
        assert_eq!(expected_error, mat.get_physical_idx(&vec![x, y, z]));

        // The matrix [4, 3, 7] has stride (21, 7, 1)
        // Expect physical id of element [1, 2, 3] to equal 1*21 + 2*7 + 3*1
        //
        let (x, y, z) = (1, 2, 3);
        assert_eq!(Ok(1*21 + 2*7 + 3*1), mat.get_physical_idx(&vec![x, y, z]));
    }

    // TODO: Add more tests
    #[test]
    fn test_broadcasting(){
        match matrix::broadcast(&vec![3], Layout::ColumnMajor, &vec![3,1], Layout::RowMajor) {
            Ok((v1, v2, v3)) => {
                println!("{:?}", v1);
                println!("{:?}", v2);
                println!("{:?}", v3);
            },
            Err(E) => panic!("{}", E)
        }
    }

    #[test]
    fn test_get() {
        let mut mat: Matrix<i32> = Matrix::from_iter(vec![3, 4], 1..,Layout::RowMajor);
        mat.apply_mut(|mut n| *n *= 2);

        assert_eq!(Ok(&14), mat.get(&vec![1, 2]));
        assert_eq!(Err(MatrixError::OutOfBounds), mat.get(&vec![3, 4]));

    }

    #[test]
    fn test_get_mut() {
        let mut mat: Matrix<i32> = Matrix::from_iter(vec![3, 4], 1..,Layout::RowMajor);
        mat.apply_mut(|mut n| *n *= 2);

        let x = match mat.get_mut(&vec![0,0]) {
            Ok(T) => T,
            Err(E) => panic!()
        };
        *x = 5;

        assert_eq!(Ok(&5), mat.get(&vec![0, 0]));
    }

    #[test]
    fn test_set() {
        let mut mat: Matrix<i32> = Matrix::from_iter(vec![3, 4], 1..,Layout::RowMajor);

        assert_eq!(Ok(()), mat.set(&vec![0, 0], 5));
        assert_eq!(Ok(()), mat.set(&vec![0, 1], 2));
        assert_eq!(Ok(()), mat.set(&vec![0, 2], 8));
        // Index out of bounds, since 7 > 4-1
        assert_eq!(Err(MatrixError::OutOfBounds), mat.set(&vec![0, 7], 8));
        // Number of dims doesn't match
        assert_eq!(Err(MatrixError::DimError), mat.set(&vec![0, 2, 1], 8));
    }

    #[test]
    fn test_apply() {
        let mut mat: Matrix<i32> = Matrix::from_iter(vec![3, 6], 1..,Layout::RowMajor);
        let mut sum = 0;
        mat.apply(|n| sum += *n);
        println!("{:#?}", mat);
        assert_eq!(sum, 171);
    }

    #[test]
    fn test_apply_mut() {
        let mut mat: Matrix<i32> = Matrix::from_iter(vec![2, 2], 1.., Layout::RowMajor);
        mat.apply_mut(|mut n| *n *= 2);

        assert_eq!(mat.data, vec![2, 4, 6, 8]);
    }

    /*
    // Write tests for everything cause it's buggy as fuck
     #[test]
     fn test_transpose(){
         let mut mat: Matrix<i32> = Matrix::new(vec![2, 3], Layout::RowMajor);
         mat.set(&vec![0,0], 6);
         mat.set(&vec![0,1], 4);
         mat.set(&vec![0,2], 24);
         mat.set(&vec![1,0], 1);
         mat.set(&vec![1,1], -9);
         mat.set(&vec![1,2], 8);

         match mat.get(&vec![0,0]) {
             Ok(val) => println!("{}", val),
             Err(err) => println!("{}", err)
         }

         mat.transpose();
         println!("{:?}", mat.strides);
         println!("{:?}", mat.shape);

         match mat.get(&vec![0,0]) {
             Ok(val) => println!("{}", val),
             Err(err) => println!(err)
         }
     }
     */
}




/*
#[cfg(test)]
mod test_layers
{
    use crate::dense_layer;

    #[test]
    fn test_dense_layer_weights_and_biases() {
        let dense_layer = dense_layer::DenseLayer::new(20, 10);
        assert_eq!(10, dense_layer.weights.len());
        assert_eq!(20, dense_layer.weights[0].len());
        assert_eq!(10, dense_layer.biases.len());
    }
}
*/
