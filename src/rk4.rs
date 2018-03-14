use super::traits::{Function, Number};

/**
 * Solves `func` using the 4th order Runge-Kutta algorithm.
 *
 * The solver returns two vectors, containing the times used in each step of the
 * algorithm and the respectful values for that time.
 */
pub fn solver<T, F>(
    func: F,
    initial_conditions: Vec<T>,
    time_interval: &[T; 2],
    step: T,
    weights: &Vec<T>,
    weight_sum: T,
) -> (Vec<T>, Vec<Vec<T>>)
where
    T: Number,
    F: Function<T>,
{
    // values to be returned
    let mut time_stamps: Vec<T> = vec![time_interval[0]];
    let mut calculated_vals: Vec<Vec<T>> =
        Vec::with_capacity(initial_conditions.len());

    for i in 0..initial_conditions.len() {
        calculated_vals.push(vec![]);
        calculated_vals[i].push(initial_conditions[i]);
    }

    // initial state of the calculation
    let mut current_vals: Vec<T> = initial_conditions.clone();
    let mut current_time: T = time_stamps[time_stamps.len() - 1];

    // the number 2 in the T type
    let t_2 = T::from_str_radix("2", 10).ok().unwrap();

    // the actual calculation
    while current_time + step < time_interval[1] {
        // k2 = dt * func(t, x)
        let mut k1: Vec<T> = func(&current_time, &current_vals);
        for i in 0..k1.len() {
            k1[i] = k1[i] * step;
        }

        // k2 = dt * func(t + dt/2, x + k1/2)
        let mut currvls_k2 = current_vals.clone();

        for i in 0..currvls_k2.len() {
            currvls_k2[i] = currvls_k2[i] + k1[i] / t_2;
        }

        let mut k2: Vec<T> = func(&(current_time + step / t_2), &currvls_k2);

        for i in 0..k2.len() {
            k2[i] = k2[i] * step;
        }

        // k3 = dt * func(t + dt/2, x + k2/2)
        let mut currvls_k3 = current_vals.clone();
        for i in 0..currvls_k3.len() {
            currvls_k3[i] = currvls_k3[i] + k2[i] / t_2;
        }

        let mut k3: Vec<T> = func(&(current_time + step / t_2), &currvls_k3);

        for i in 0..k3.len() {
            k3[i] = k3[i] * step;
        }

        // k4 = dt * func(t + dt, x + k3)
        //                       |______|
        //                           |
        //                      currvls_k4
        let mut currvls_k4 = current_vals.clone();

        for i in 0..currvls_k4.len() {
            currvls_k4[i] = currvls_k4[i] + k3[i];
        }

        let mut k4: Vec<T> = func(&(current_time + step), &currvls_k4);

        for i in 0..k4.len() {
            k4[i] = k4[i] * step;
        }

        // y[n+1] = y[n] + h*(k1 + 2*k2 + 2*k3 + k4)/6
        let mut curr_point = current_vals.clone();

        for i in 0..curr_point.len() {
            curr_point[i] = curr_point[i]
                + step
                    * (k1[i] * weights[0] + k2[i] * weights[1]
                        + k3[i] * weights[2]
                        + k4[i] * weights[3]) / weight_sum;
        }

        // update values for the next iteration
        for i in 0..calculated_vals.len() {
            current_vals[i] = calculated_vals[i].last().unwrap().clone();
        }

        for i in 0..calculated_vals.len() {
            calculated_vals[i].push(curr_point[i]);
            current_vals[i] = calculated_vals[i].last().unwrap().clone();
        }

        current_time = time_stamps[time_stamps.len() - 1] + step;
        time_stamps.push(current_time);
    }

    (time_stamps, calculated_vals)
}

//fn stepper(func: F,
//           initial_conditions: Vec<T>,
//           time_interval: &[T; 2],
//           step: T) {}

#[cfg(test)]
mod tests {
    use super::solver;

    /*
     * Consider the ODE:
     *
     *     y'(t) = 2*t
     *
     * The analitical solution is
     *
     *     y(t) = t^2 + c
     */
    #[test]
    fn integrate_2_t() {
        let start = 0;
        let end = 500;

        let (_, num_sol) = solver(
            |t: &f32, _: &Vec<f32>| vec![2. * t],
            vec![0.],
            &[start as f32, end as f32],
            1.0,
            &vec![1., 2., 2., 1.],
            6.,
        );

        let mut an_sol: Vec<u32> = vec![];
        for i in start..end {
            an_sol.push(i * i);
        }

        // to compare the results, we need to convert the numerical solution to
        // u32 (easy to do, since the solution is exact)
        let mut num_sol_u32: Vec<u32> = Vec::new();
        for el in num_sol[0].clone() {
            let el_cl = el.clone();
            num_sol_u32.push(el_cl as u32);
        }

        // compare lengths
        assert_eq!(num_sol[0].len(), an_sol.len());
        assert_eq!(num_sol_u32, an_sol);
    }
}
