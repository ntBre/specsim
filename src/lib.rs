use std::f64::consts::PI;

use clap::ValueEnum;

#[derive(Clone, Debug, ValueEnum)]
pub enum LineShape {
    Gaussian,
    Lorentzian,
}

pub struct Spectrum {
    freqs: Vec<f64>,
    inten: Vec<f64>,
}

impl Spectrum {
    pub fn load(s: String) -> Self {
        let mut freqs = Vec::new();
        let mut inten = Vec::new();
        for (i, line) in s.lines().enumerate() {
            let line = line.trim();
            if line.starts_with('#') {
                continue;
            }
            let sp: Vec<&str> = line.split_ascii_whitespace().collect();
            if sp.len() != 2 {
                eprintln!("skipping line {}, wrong number of entries", i + 1);
                continue;
            }

            freqs.push(sp[0].parse::<f64>().unwrap_or_else(|_| {
                eprintln!("failed to parse line {}", i + 1);
                std::process::exit(1);
            }));
            inten.push(sp[1].parse::<f64>().unwrap_or_else(|_| {
                eprintln!("failed to parse line {}", i + 1);
                std::process::exit(1);
            }));
        }

        Self { freqs, inten }
    }

    pub fn sim(
        &self,
        shape: LineShape,
        npoints: usize,
        deltag: f64,
    ) -> (Vec<f64>, Vec<f64>) {
        let nstates = self.freqs.len();
        // let min = *self.freqs.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
        // let max = *self.freqs.iter().max_by(|a, b| a.total_cmp(b)).unwrap();

        // for some reason these are hard-coded when they used to be calculated.
        // we'll see which way we want to go
        let min = 100.;
        let max = 3800.;
        let x = self.energy_range(min, max, deltag, npoints);
        let mut y = vec![0.0; npoints];
        for i in 0..nstates {
            let ys = match shape {
                LineShape::Gaussian => {
                    self.gaussian(&x, self.freqs[i], self.inten[i], deltag)
                }
                LineShape::Lorentzian => {
                    self.lorentzian(&x, self.freqs[i], self.inten[i], deltag)
                }
            };
            for j in 0..npoints {
                y[j] += ys[j];
            }
        }

        (x, y)
    }

    fn energy_range(
        &self,
        min: f64,
        max: f64,
        _fwhm: f64, /* unused when min and max aren't computed */
        np: usize,
    ) -> Vec<f64> {
        let spacing = (max - min) / np as f64;
        let mut ret = vec![0.0; np];
        for (i, r) in ret.iter_mut().enumerate() {
            *r = min + i as f64 * spacing;
        }
        ret
    }

    fn gaussian(&self, x: &[f64], ex: f64, ab: f64, fwhm: f64) -> Vec<f64> {
        let dim = x.len();
        let mut y = Vec::with_capacity(dim);
        for xi in x {
            let tmp = (-(xi - ex).powi(2) / (2. * fwhm * fwhm))
                / (fwhm * (2. * PI).sqrt());
            y.push(ab * tmp.exp());
        }
        y
    }

    fn lorentzian(
        &self,
        _x: &[f64],
        _ex: f64,
        _ab: f64,
        _deltag: f64,
    ) -> Vec<f64> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use approx::assert_abs_diff_eq;

    use super::*;

    #[test]
    fn full() {
        let s = Spectrum::load(read_to_string("testfiles/h2o.in").unwrap());
        let (x, y) = s.sim(LineShape::Gaussian, 4000, 1.0);
        let (wantx, wanty): (Vec<f64>, Vec<f64>) =
            read_to_string("testfiles/h2o.want")
                .unwrap()
                .lines()
                .map(|s| {
                    let mut sp = s.split_ascii_whitespace();
                    (
                        sp.next().unwrap().parse::<f64>().unwrap(),
                        sp.next().unwrap().parse::<f64>().unwrap(),
                    )
                })
                .unzip();

        assert_abs_diff_eq!(&x[..], &wantx[..], epsilon = 1e-4);
        assert_abs_diff_eq!(&y[..], &wanty[..], epsilon = 1e-1);
    }
}
