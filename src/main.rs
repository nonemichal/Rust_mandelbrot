use mandelbrot_set::plot_iterations;
use num::complex::Complex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    plot_iterations(
        Complex::new(0.0, 0.0),
        Complex::new(-0.82, 0.3),
        20,
        "plotters-doc-data/0.png",
    )?;

    Ok(())
}
