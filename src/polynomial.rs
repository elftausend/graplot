use crate::{Plot, PlotArg, XEnd};

#[derive(Debug, Clone)]
/// Use the Polynomial struct or polynomial() function to create a polynomial function that runs through all given points.
/// # Example
/// ```
/// use graplot::{x, Plot, Polynomial};
///
/// let poly = Polynomial::new(&[2., 3., 1.], &[2., 3., 2.]);
/// let plot = Plot::new((poly, x(10.)));
/// plot.show();
/// ```
pub struct Polynomial {
    coefficients: Vec<f64>,
}

impl Polynomial {
    pub fn new(xs: &[f64], ys: &[f64]) -> Polynomial {
        polynomial(xs, ys)
    }
}

pub fn polynomial(xs: &[f64], ys: &[f64]) -> Polynomial {
    let degree = xs.len() - 1;

    let mut coeffs = Vec::<f64>::with_capacity(xs.len() * xs.len());

    for x in xs {
        for pow in (0..=degree).rev() {
            coeffs.push(x.powi(pow as i32));
        }
    }
    Polynomial {
        coefficients: solve_lu(xs.len(), &coeffs, ys),
    }
}

impl PlotArg for Polynomial {
    fn as_plot(&self) -> crate::Plot {
        let mut xs = [0.; 200];

        let mut add = -100f64;
        for x in &mut xs {
            *x = add / 100.;
            add += 1.;
        }

        let mut ys = [0.; 200];

        let degree = self.coefficients.len() - 1;

        for (i, y) in ys.iter_mut().enumerate() {
            for (pow, coefficient) in self.coefficients.iter().enumerate() {
                *y += coefficient * xs[i].powi((degree - pow) as i32);
            }
        }
        Plot {
            xs: vec![xs.to_vec()],
            ys: vec![ys.to_vec()],
            line_desc: vec![Default::default()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl PlotArg for (Polynomial, Option<XEnd>) {
    fn as_plot(&self) -> crate::Plot {
        let mut xs = [0.; 200];

        let mut add = -100f64;
        for x in &mut xs {
            *x = (add / 100.) * self.1.unwrap().0;
            add += 1.;
        }

        let mut ys = [0.; 200];

        let degree = self.0.coefficients.len() - 1;

        for (i, y) in ys.iter_mut().enumerate() {
            for (pow, coefficient) in self.0.coefficients.iter().enumerate() {
                *y += coefficient * xs[i].powi((degree - pow) as i32);
            }
        }
        Plot {
            xs: vec![xs.to_vec()],
            ys: vec![ys.to_vec()],
            line_desc: vec![Default::default()],
            axis_desc: Default::default(),
            desc: Default::default(),
        }
    }
}

impl PlotArg for (Polynomial, &str) {
    fn as_plot(&self) -> Plot {
        let mut plot = self.0.as_plot();
        plot.line_desc = vec![self.1.into()];
        plot
    }
}

impl PlotArg for (Polynomial, Option<XEnd>, &str) {
    fn as_plot(&self) -> Plot {
        let mut plot = (self.0.clone(), self.1).as_plot();
        plot.line_desc = vec![self.2.into()];
        plot
    }
}

pub fn solve_lu(n: usize, lhs: &[f64], rhs: &[f64]) -> Vec<f64> {
    let mut lu = vec![0f64; n * n];
    let mut sum;
    for i in 0..n {
        for j in i..n {
            sum = 0.;
            for k in 0..i {
                sum += lu[i * n + k] * lu[k * n + j];
            }
            lu[i * n + j] = lhs[i * n + j] - sum;
        }
        for j in (i + 1)..n {
            sum = 0.;
            for k in 0..i {
                sum += lu[j * n + k] * lu[k * n + i];
            }
            lu[j * n + i] = (1. / lu[i * n + i]) * (lhs[j * n + i] - sum)
        }
    }

    let mut y = vec![0.; n];
    for i in 0..n {
        sum = 0.;
        for k in 0..i {
            sum += lu[i * n + k] * y[k];
        }
        y[i] = rhs[i] - sum;
    }

    let mut x = vec![0.; n];
    for i in (0..n).rev() {
        sum = 0.;
        for k in (i + 1)..n {
            sum += lu[i * n + k] * x[k];
        }
        x[i] = (1. / lu[i * n + i]) * (y[i] - sum);
    }
    x
}
