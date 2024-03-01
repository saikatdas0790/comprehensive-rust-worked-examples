#![allow(dead_code)]

fn magnitude(vector: &[f64]) -> f64 {
    vector.iter().map(|x| x.powi(2)).sum::<f64>().sqrt()
}

fn normalize(vector: &mut [f64]) {
    let mag = magnitude(vector);
    vector.iter_mut().for_each(|x| *x /= mag);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_magnitude() {
        let vector = [3.0, 4.0];
        assert_eq!(magnitude(&vector), 5.0);

        let vector = [1.0, 2.0, 3.0];
        assert!(magnitude(&vector) > 3.74);
        assert!(magnitude(&vector) < 3.75);
    }

    #[test]
    fn test_normalize() {
        let mut vector = [3.0, 4.0];
        normalize(&mut vector);
        assert_eq!(vector, [0.6, 0.8]);

        let mut vector = [1.0, 2.0, 3.0];
        normalize(&mut vector);
        assert!(vector[0] > 0.267);
        assert!(vector[0] < 0.268);
        assert!(vector[1] > 0.534);
        assert!(vector[1] < 0.535);
        assert!(vector[2] > 0.801);
        assert!(vector[2] < 0.802);
    }
}
