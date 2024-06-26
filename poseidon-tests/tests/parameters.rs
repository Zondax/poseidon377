use decaf377::Fq;
use proptest::prelude::*;

use poseidon_parameters::v1::{mat_mul, square_mat_mul};
use poseidon_parameters::v1::{Matrix, MatrixOperations};
use poseidon_parameters::v1::{SquareMatrix, SquareMatrixOperations};

#[test]
fn identity_matrix() {
    let identity = SquareMatrix::<2, 4>::identity();
    assert_eq!(identity.get_element(0, 0), Fq::from(1u64));
    assert_eq!(identity.get_element(0, 1), Fq::from(0u64));
    assert_eq!(identity.get_element(1, 1), Fq::from(1u64));
    assert_eq!(identity.get_element(1, 0), Fq::from(0u64));
}

#[test]
fn square_matmul() {
    let identity = SquareMatrix::<2, 4>::identity();

    let elements = &[
        Fq::from(1u64),
        Fq::from(2u64),
        Fq::from(3u64),
        Fq::from(4u64),
    ];
    let matrix_2x2 = SquareMatrix::<2, 4>::new(elements);

    let res: SquareMatrix<2, 4> = square_mat_mul(&matrix_2x2, &identity);
    assert_eq!(res.get_element(0, 0), Fq::from(1u64));
    assert_eq!(res.get_element(0, 1), Fq::from(2u64));
    assert_eq!(res.get_element(1, 0), Fq::from(3u64));
    assert_eq!(res.get_element(1, 1), Fq::from(4u64));
}

#[test]
fn nonsquare_matmul_happy() {
    let test_elements = &[
        Fq::from(1u64),
        Fq::from(2u64),
        Fq::from(3u64),
        Fq::from(4u64),
        Fq::from(5u64),
        Fq::from(6u64),
    ];
    let matrix_2x3 = Matrix::<3, 2, 6>::new(test_elements);

    let matrix_3x2 = matrix_2x3.transpose();
    let res: Matrix<3, 3, 9> = mat_mul(&matrix_2x3, &matrix_3x2);
    assert_eq!(res.get_element(0, 0), Fq::from(5u64));
    assert_eq!(res.get_element(0, 1), Fq::from(11u64));
    assert_eq!(res.get_element(0, 2), Fq::from(17u64));
    assert_eq!(res.get_element(1, 0), Fq::from(11u64));
    assert_eq!(res.get_element(1, 1), Fq::from(25u64));
    assert_eq!(res.get_element(1, 2), Fq::from(39u64));
    assert_eq!(res.get_element(2, 0), Fq::from(17u64));
    assert_eq!(res.get_element(2, 1), Fq::from(39u64));
    assert_eq!(res.get_element(2, 2), Fq::from(61u64));
}

#[test]
fn hadamard_product() {
    let test_elements: [Fq; 6] = [
        Fq::from(1u64),
        Fq::from(2u64),
        Fq::from(3u64),
        Fq::from(4u64),
        Fq::from(5u64),
        Fq::from(6u64),
    ];
    let matrix_3x2 = Matrix::<3, 2, 6>::new(&test_elements);

    let res = matrix_3x2.hadamard_product(&matrix_3x2).expect("is ok");
    assert_eq!(res.get_element(0, 0), Fq::from(1u64));
    assert_eq!(res.get_element(0, 1), Fq::from(4u64));
    assert_eq!(res.get_element(1, 0), Fq::from(9u64));
    assert_eq!(res.get_element(1, 1), Fq::from(16u64));
    assert_eq!(res.get_element(2, 0), Fq::from(25u64));
    assert_eq!(res.get_element(2, 1), Fq::from(36u64));
}

#[test]
fn transpose() {
    let test_elements = &[
        Fq::from(1u64),
        Fq::from(2u64),
        Fq::from(3u64),
        Fq::from(4u64),
        Fq::from(5u64),
        Fq::from(6u64),
    ];
    let matrix_2x3 = Matrix::<3, 2, 6>::new(test_elements);
    assert_eq!(matrix_2x3.get_element(0, 1), Fq::from(2u64));
    assert_eq!(matrix_2x3.get_element(1, 0), Fq::from(3u64));
    assert_eq!(matrix_2x3.get_element(1, 1), Fq::from(4u64));
    assert_eq!(matrix_2x3.get_element(2, 0), Fq::from(5u64));
    assert_eq!(matrix_2x3.get_element(2, 1), Fq::from(6u64));
    let res = matrix_2x3.transpose();
    assert_eq!(res.get_element(1, 0), Fq::from(2u64));
    assert_eq!(res.get_element(0, 1), Fq::from(3u64));
    assert_eq!(res.get_element(1, 1), Fq::from(4u64));
    assert_eq!(res.get_element(0, 2), Fq::from(5u64));
    assert_eq!(res.get_element(1, 2), Fq::from(6u64));

    let test_elements = &[
        Fq::from(1u64),
        Fq::from(2u64),
        Fq::from(3u64),
        Fq::from(4u64),
    ];
    let matrix_2x2 = SquareMatrix::<2, 4>::new(test_elements);

    let res = matrix_2x2.transpose();
    assert_eq!(res.get_element(0, 0), Fq::from(1u64));
    assert_eq!(res.get_element(0, 1), Fq::from(3u64));
    assert_eq!(res.get_element(1, 0), Fq::from(2u64));
    assert_eq!(res.get_element(1, 1), Fq::from(4u64));
}

#[test]
fn cofactors() {
    let identity_1x1 = SquareMatrix::<1, 1>::identity();
    let test_elements = &[Fq::from(1u64)];
    let expected_res = SquareMatrix::new(test_elements);
    assert_eq!(identity_1x1.cofactors(), expected_res);

    let identity_2x2 = SquareMatrix::<2, 4>::identity();
    let test_elements = &[
        Fq::from(1u64),
        -Fq::from(1u64),
        -Fq::from(1u64),
        Fq::from(1u64),
    ];
    let expected_res = SquareMatrix::new(test_elements);
    assert_eq!(identity_2x2.cofactors(), expected_res);
}

fn fq_strategy() -> BoxedStrategy<Fq> {
    any::<[u8; 16]>()
        .prop_map(|bytes| Fq::from_le_bytes_mod_order(&bytes[..]))
        .boxed()
}

proptest! {
    #[test]
    fn inverse_2x2(a in fq_strategy(), b in fq_strategy(), c in fq_strategy(), d in fq_strategy()) {
        let matrix_2x2 = SquareMatrix::<2, 4>::new(&[a, b, c, d]);

        let res = matrix_2x2.inverse().unwrap();
        assert_eq!(square_mat_mul(&matrix_2x2, &res), SquareMatrix::<2, 4>::identity());
    }
}

#[test]
fn inverse() {
    let matrix_1x1 = SquareMatrix::<1, 1>::new(&[Fq::from(2u64)]);
    let res = matrix_1x1.inverse().unwrap();
    assert_eq!(
        square_mat_mul(&matrix_1x1, &res),
        SquareMatrix::<1, 1>::identity()
    );

    let matrix_2x2 = SquareMatrix::<2, 4>::new(&[
        Fq::from(1u64),
        Fq::from(2u64),
        Fq::from(3u64),
        Fq::from(4u64),
    ]);

    let res = matrix_2x2.inverse().unwrap();
    assert_eq!(
        square_mat_mul(&matrix_2x2, &res),
        SquareMatrix::<2, 4>::identity()
    );

    let identity_3x3 = SquareMatrix::<3, 9>::identity();
    assert_eq!(identity_3x3, identity_3x3.inverse().unwrap());

    let matrix_3x3 = SquareMatrix::<3, 9>::new(&[
        Fq::from(3u64),
        Fq::from(0u64),
        Fq::from(2u64),
        Fq::from(2u64),
        Fq::from(0u64),
        -Fq::from(2u64),
        Fq::from(0u64),
        Fq::from(1u64),
        Fq::from(1u64),
    ]);
    let res = matrix_3x3.inverse().unwrap();
    assert_eq!(
        square_mat_mul(&matrix_3x3, &res),
        SquareMatrix::<3, 9>::identity()
    );

    let expected_res = SquareMatrix::<3, 9>::new(&[
        Fq::from(2u64),
        Fq::from(2u64),
        Fq::from(0u64),
        -Fq::from(2u64),
        Fq::from(3u64),
        Fq::from(10u64),
        Fq::from(2u64),
        -Fq::from(3u64),
        Fq::from(0u64),
    ]) * (Fq::from(1u64) / Fq::from(10u64));
    assert_eq!(res, expected_res);
}

#[test]
fn create_matrix_from_array() {
    let matrix_2x2 = SquareMatrix::<2, 4>::new(&[
        Fq::from(1u64),
        Fq::from(2u64),
        Fq::from(3u64),
        Fq::from(4u64),
    ]);
    assert_eq!(matrix_2x2.get_element(0, 0), Fq::from(1u64));
    assert_eq!(matrix_2x2.get_element(0, 1), Fq::from(2u64));
    assert_eq!(matrix_2x2.get_element(1, 0), Fq::from(3u64));
    assert_eq!(matrix_2x2.get_element(1, 1), Fq::from(4u64));

    let matrix_2x3 = Matrix::<2, 3, 6>::new(&[
        Fq::from(1u64),
        Fq::from(2u64),
        Fq::from(3u64),
        Fq::from(4u64),
        Fq::from(5u64),
        Fq::from(6u64),
    ]);
    assert_eq!(matrix_2x3.get_element(0, 0), Fq::from(1u64));
    assert_eq!(matrix_2x3.get_element(0, 1), Fq::from(2u64));
    assert_eq!(matrix_2x3.get_element(0, 2), Fq::from(3u64));
    assert_eq!(matrix_2x3.get_element(1, 0), Fq::from(4u64));
    assert_eq!(matrix_2x3.get_element(1, 1), Fq::from(5u64));
    assert_eq!(matrix_2x3.get_element(1, 2), Fq::from(6u64));
}

#[test]
fn determinant() {
    let matrix_1x1 = SquareMatrix::<1, 1>::new(&[Fq::from(1u64)]);
    assert_eq!(matrix_1x1.determinant(), Fq::from(1u64));

    let a = Fq::from(1u64);
    let b = Fq::from(1u64) + Fq::from(1u64);
    let c = Fq::from(3u64);
    let d = Fq::from(4u64);
    let matrix_2x2 = SquareMatrix::<2, 4>::new(&[a, b, c, d]);
    assert_eq!(matrix_2x2.determinant(), -Fq::from(2u64));

    let e = Fq::from(5u64);
    let f = Fq::from(6u64);
    let g = Fq::from(7u64);
    let h = Fq::from(8u64);
    let i = Fq::from(9u64);
    let matrix_3x3 = SquareMatrix::<3, 9>::new(&[a, b, c, d, e, f, g, h, i]);
    assert_eq!(matrix_3x3.determinant(), Fq::from(0u64));

    let elem = Fq::from(10u64);
    let matrix_4x4 = SquareMatrix::<4, 16>::new(&[
        a, b, c, d, e, f, g, h, i, elem, elem, elem, elem, elem, elem, elem,
    ]);
    assert_eq!(matrix_4x4.determinant(), Fq::from(0u64));

    let matrix_5x5 = SquareMatrix::<5, 25>::new(&[
        a, b, c, d, e, f, g, h, i, elem, elem, elem, elem, elem, elem, elem, elem, elem, elem,
        elem, elem, elem, elem, elem, elem,
    ]);
    assert_eq!(matrix_5x5.determinant(), Fq::from(0u64));

    let mut elements = vec![a, b, c, d, e, f, g, h, i];
    elements.extend_from_slice(&[elem; 27]);
    let matrix_6x6 = SquareMatrix::<6, 36>::new(&elements[..]);
    assert_eq!(matrix_6x6.determinant(), Fq::from(0u64));

    let mut elements = vec![a, b, c, d, e, f, g, h, i];
    elements.extend_from_slice(&[elem; 40]);
    let matrix_7x7 = SquareMatrix::<7, 49>::new(&elements[..]);
    assert_eq!(matrix_7x7.determinant(), Fq::from(0u64));

    let mut elements = vec![a, b, c, d, e, f, g, h, i];
    elements.extend_from_slice(&[elem; 55]);
    let matrix_8x8 = SquareMatrix::<8, 64>::new(&elements[..]);
    assert_eq!(matrix_8x8.determinant(), Fq::from(0u64));
}
