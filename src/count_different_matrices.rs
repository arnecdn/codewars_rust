
fn count_different_matrices(matrices: &[[i16; 4]]) -> usize {
    matrices.iter().fold(vec![],|mut acc:Vec<Vec<i16>>,m|-> Vec<Vec<_>>{
        if !acc.exists_with_rotation(*m, 0){
            acc.push(m.to_vec());
        }
        acc
    }).len()
}
// struct UniqueMatrices;
trait UniqueMatrices {
    fn exists_with_rotation(&mut self, target: [i16; 4], i: usize) -> bool;
}
impl UniqueMatrices for Vec<Vec<i16>> {
    fn exists_with_rotation(&mut self, target: [i16; 4], i: usize) -> bool {
        if i == 4 { return false; }

        if !self.contains(&target.to_vec()) {
            let [a, b, c, d] = target;
            return self.exists_with_rotation([c, a, d, b], i + 1);
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use super::count_different_matrices;

    #[test]
    fn sample_tests() {
        let ms =
            [[1, 2, 3, 4],
                [3, 1, 4, 2],
                [4, 3, 2, 1],
                [2, 4, 1, 3]];
        assert_eq!(count_different_matrices(&ms), 1);
        let ms =
            [[3, 1, 2, 3],
                [3, 1, 2, 3],
                [1, 3, 3, 2],
                [3, 2, 1, 3]];
        assert_eq!(count_different_matrices(&ms), 1);
        let ms =
            [[5, 1, 2, 6],
                [5, 4, 3, 5],
                [2, 5, 6, 1]];
        assert_eq!(count_different_matrices(&ms), 2);
        let ms =
            [[1, 2, 2, 1],
                [1, 1, 2, 2],
                [2, 1, 1, 2],
                [2, 1, 2, 1],
                [1, 2, 1, 2]];
        assert_eq!(count_different_matrices(&ms), 2);
    }
}
