pub fn hamming_distance(seq1: &str, seq2: &str) -> Result<usize, ()> {
    if seq1.len() != seq2.len() {
        return Err(());
    }

	Ok(seq1.chars()
	   .zip(seq2.chars())
	   .filter(|&(i, j)| i != j)
	   .count())
}
