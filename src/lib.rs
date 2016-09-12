pub mod rk4;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let _ = rk4::solver(|t: &f32, _: &Vec<f32>| -> Vec<f32> {
            vec![2.*t]
        }, vec![0.], &[0., 0.], 1.0);
    }
}
