use crate::{error::PoseidonParameterError, matrix::Matrix, matrix_ops::MatrixOperations};
use decaf377::Fq;

/// Represents an matrix of round constants.
///
/// Arc stands for `AddRoundConstant` which is the
/// step in the permutation where this matrix is used.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArcMatrix<const N_ROWS: usize, const N_COLS: usize, const N_ELEMENTS: usize>(
    pub Matrix<N_ROWS, N_COLS, N_ELEMENTS>,
);

impl<const N_ROWS: usize, const N_COLS: usize, const N_ELEMENTS: usize>
    ArcMatrix<N_ROWS, N_COLS, N_ELEMENTS>
{
    pub fn transpose(&self) -> ArcMatrix<N_COLS, N_ROWS, N_ELEMENTS> {
        ArcMatrix(self.0.transpose())
    }

    pub fn inner_elements(&self) -> [Fq; N_ELEMENTS] {
        self.0.elements
    }

    /// Create a new matrix from a slice of elements.
    pub const fn new_from_known(elements: [Fq; N_ELEMENTS]) -> Self {
        Self(Matrix::new_from_known(elements))
    }
}

impl<const N_ROWS: usize, const N_COLS: usize, const N_ELEMENTS: usize> MatrixOperations
    for ArcMatrix<N_ROWS, N_COLS, N_ELEMENTS>
{
    fn new(elements: &[Fq]) -> Self {
        Self(Matrix::new(elements))
    }

    fn elements(&self) -> &[Fq] {
        self.0.elements()
    }

    fn get_element(&self, i: usize, j: usize) -> Fq {
        self.0.get_element(i, j)
    }

    fn set_element(&mut self, i: usize, j: usize, val: Fq) {
        self.0.set_element(i, j, val)
    }

    fn n_rows(&self) -> usize {
        self.0.n_rows()
    }

    fn n_cols(&self) -> usize {
        self.0.n_cols()
    }

    fn hadamard_product(&self, rhs: &Self) -> Result<Self, PoseidonParameterError>
    where
        Self: Sized,
    {
        Ok(Self(self.0.hadamard_product(&rhs.0)?))
    }
}

/// Represents an optimized matrix of round constants.
///
/// This modifies the partial rounds in the middle of the permutation,
/// wherein you add constants _first_ before iterating through the partial
/// rounds.
///
/// This method follows `calc_equivalent_constants` from Appendix B's
/// `poseidonperm_x3_64_24_optimized.sage`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OptimizedArcMatrix<const N_ROWS: usize, const N_COLS: usize, const N_ELEMENTS: usize>(
    pub ArcMatrix<N_ROWS, N_COLS, N_ELEMENTS>,
);

impl<const N_ROWS: usize, const N_COLS: usize, const N_ELEMENTS: usize>
    OptimizedArcMatrix<N_ROWS, N_COLS, N_ELEMENTS>
{
    pub fn transpose(&self) -> OptimizedArcMatrix<N_COLS, N_ROWS, N_ELEMENTS> {
        OptimizedArcMatrix(self.0.transpose())
    }

    /// Create a new matrix from a slice of elements.
    pub const fn new_from_known(elements: [Fq; N_ELEMENTS]) -> Self {
        Self(ArcMatrix::new_from_known(elements))
    }
}

impl<const N_ROWS: usize, const N_COLS: usize, const N_ELEMENTS: usize> MatrixOperations
    for OptimizedArcMatrix<N_ROWS, N_COLS, N_ELEMENTS>
{
    /// Create a `OptimizedArcMatrix` from its elements.
    fn new(elements: &[Fq]) -> Self {
        Self(ArcMatrix::new(elements))
    }

    fn elements(&self) -> &[Fq] {
        self.0.elements()
    }

    fn get_element(&self, i: usize, j: usize) -> Fq {
        self.0.get_element(i, j)
    }

    fn set_element(&mut self, i: usize, j: usize, val: Fq) {
        self.0.set_element(i, j, val)
    }

    fn n_rows(&self) -> usize {
        self.0.n_rows()
    }

    fn n_cols(&self) -> usize {
        self.0.n_cols()
    }

    fn hadamard_product(&self, rhs: &Self) -> Result<Self, PoseidonParameterError>
    where
        Self: Sized,
    {
        Ok(Self(self.0.hadamard_product(&rhs.0)?))
    }
}
