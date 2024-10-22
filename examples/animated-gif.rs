use plotters::prelude::*;

fn product_invariant(x: f64, y: f64) -> f64 {
    x*y/1.0
}

const OUT_FILE_NAME: &str = "plotters-doc-data/3d-plot2.gif";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::gif(OUT_FILE_NAME, (600, 400), 100)?.into_drawing_area();

    for yaw in 0..157 {
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("Product-Invariant z=x*y", ("sans-serif", 20))
            .build_cartesian_3d(-3.0..3.0, 0.0..6.0, -3.0..3.0)?;
        chart.with_projection(|mut p| {
            p.yaw = 3.14 - (1.57 - yaw as f64 / 50.0).abs();
            p.scale = 0.7;
            p.into_matrix() // build the projection matrix
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(3)
            .draw()?;

        chart.draw_series(
            SurfaceSeries::xoz(
                (-15..=15).map(|x| x as f64 / 5.0),
                (-15..=15).map(|x| x as f64 / 5.0),
                product_invariant,
            )
            .style_func(&|&v| (VulcanoHSL::get_color(v / 5.0)).into()),
        )?;

        root.present()?;
    }

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}
#[test]
fn entry_point() {
    main().unwrap()
}