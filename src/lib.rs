use num::complex::Complex;
use plotters::prelude::*;

#[derive(Clone)]
struct Mandelbrot {
    val: Complex<f64>,
    c: Complex<f64>,
}

impl Mandelbrot {
    const fn new(val: Complex<f64>, c: Complex<f64>) -> Mandelbrot {
        Mandelbrot { val, c }
    }
}

impl Iterator for Mandelbrot {
    type Item = Complex<f64>;

    fn next(&mut self) -> Option<Complex<f64>> {
        self.val = self.val * self.val + self.c;

        if self.val.is_normal() {
            Some(self.val)
        } else {
            None
        }
    }
}

pub fn plot_iterations(
    val: Complex<f64>,
    c: Complex<f64>,
    n: usize,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mandelbrot_iterations = Mandelbrot::new(val, c).take(n).map(|val| (val.re, val.im));

    for num in mandelbrot_iterations.clone() {
        println!("{:?}\n", num);
    }

    let root = BitMapBackend::new(path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let (mut x_min, mut x_max, mut y_min, mut y_max) = (-2.1f64, 0.6f64, -1.2f64, 1.2f64);

    let re = mandelbrot_iterations.clone().map(|x| x.0);
    let re_max = re.clone().reduce(f64::max).unwrap();
    let re_min = re.clone().reduce(f64::min).unwrap();
    let im = mandelbrot_iterations.clone().map(|x| x.1);
    let im_max = im.clone().reduce(f64::max).unwrap().to_owned();
    let im_min = im.clone().reduce(f64::min).unwrap().to_owned();

    if re_max > x_max {
        x_max = re_max;
    }
    if re_min < x_min {
        x_min = re_min;
    }
    if im_min > y_max {
        y_max = im_max;
    }
    if im_min < y_min {
        y_min = im_min;
    }

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .x_label_area_size(10)
        .y_label_area_size(10)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(PointSeries::of_element(
        mandelbrot_iterations.clone(),
        3,
        &RED,
        &|c, s, st| Circle::new(c, s, st.filled()),
    ))?;

    chart.draw_series(LineSeries::new(mandelbrot_iterations.clone(), &RED))?;

    root.present()?;

    Ok(())
}
