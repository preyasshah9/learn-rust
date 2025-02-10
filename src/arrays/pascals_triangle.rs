impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut matrix: Vec<Vec<i32>> = Vec::with_capacity(num_rows as usize);
        for i in 0..num_rows as usize {
            let mut output = vec![1; i + 1];
            for j in 1..i {
                output[j] = matrix[i - 1][j - 1] + matrix[i - 1][j];
            }
            println!("Vector: {:?}", output);
            matrix.push(output); 
        }
        return matrix;
    }
}
