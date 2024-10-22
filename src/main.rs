use plotters::prelude::*;

use std::io;

use num_traits::sign::Signed;

const OUT_FILE_NAME: &str = "plotters-doc-data/first_section.png";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sd = 0.60;
    let mut input = String::new();
    let mut parsed_f64: Vec<f64> = Vec::new();
    while io::stdin().read_line(&mut input)? != 0 {
        if input.trim().is_empty() { break };
        let input_split = input.trim().split(",").collect::<Vec<&str>>();
        parsed_f64.push(input_split[2].parse::<f64>()?);
        input.clear();
    };
    
    let sd = 0.60;

    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .caption("Price Distribution", ("sans-serif", 30))
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .set_label_area_size(LabelAreaPosition::Right, 60)
        .build_cartesian_2d(-2f64..2f64, 0f64..0.1)?
        .set_secondary_coord(
            (-2f64..2f64).step(0.000000001).use_round().into_segmented(),
            0u32..20u32,
        );

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .y_label_formatter(&|y| format!("{:.0}%", *y * 100.0))
        .y_desc("Percentage")
        .draw()?;

    chart.configure_secondary_axes().y_desc("Count").draw()?;
    
    println!("data {:?}", parsed_f64);

    let actual = Histogram::vertical(chart.borrow_secondary())
        .style(GREEN.filled())
        .margin(1)
        .data(parsed_f64.iter().map(|x| (*x, 1)));

    chart
        .draw_secondary_series(actual)?
        .label("Observed")
        .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], GREEN.filled()));

    let pdf = LineSeries::new(
        (-200..200).map(|x| x as f64 / 100.0).map(|x| {
            (
                x,
                (-x * x / 2.0 / sd / sd).exp() / (2.0 * std::f64::consts::PI * sd * sd).sqrt()
                    * 0.1,
            )
        }),
        &RED,
    );

    chart
        .draw_series(pdf)?
        .label("PDF")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED.filled()));

    chart.configure_series_labels().draw()?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}
#[test]
fn entry_point() {
    main().unwrap()
}