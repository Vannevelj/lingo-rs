use chrono::{Duration, NaiveDate, TimeZone, Utc};
use directories::UserDirs;
use hex::FromHex;
use log::{debug, info};
use plotters::prelude::*;
use std::ops::Add;

use crate::ChronologicalLookup;

pub fn create_graph(data: &ChronologicalLookup, chart_name: String) {
    info!("Creating graph..");
    let output_file = UserDirs::new()
        .expect("Could not find a HOME directory")
        .desktop_dir()
        .expect("No Desktop directory found")
        .join(format!("{}.png", chart_name));

    let root = BitMapBackend::new(&output_file, (800, 640)).into_drawing_area();
    root.fill(&WHITE).expect("Failed to set chart background");

    let start_date = get_min_date(data).expect("No start date found");
    let end_date = get_max_date(data).expect("No end date found");

    let mut chart = ChartBuilder::on(&root)
        .caption(chart_name, ("sans-serif", 40).into_font())
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .build_cartesian_2d(
            (Utc.from_utc_date(start_date)..Utc.from_utc_date(end_date)).monthly(),
            -0.00001..101.0,
        )
        .expect("Failed to set chart axis");

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .y_label_formatter(&|x| format!("{:.2}%", x))
        .y_desc("Prevalence")
        .draw()
        .expect("Failed to render mesh");

    info!("Creating {} series", data.len());

    for (language, values) in data.iter() {
        debug!("{:?}", values);
        let color = convert_color(&language.color);

        chart
            .draw_series(values.iter().map(|(x, y)| {
                let x0 = Utc.from_utc_date(x);
                let x1 = Utc.from_utc_date(&x.add(Duration::days(1)));

                let offset_from_bottom = y.cumulative_percentage - y.percentage;
                debug!(
                    "Rendering {} at offset {}, length {}",
                    language.name, offset_from_bottom, y.percentage
                );

                let mut bar = Rectangle::new(
                    [(x0, offset_from_bottom), (x1, y.cumulative_percentage)],
                    color.filled(),
                );
                bar.set_margin(0, 0, 5, 5);
                bar
            }))
            .expect("Failed to draw series")
            .legend(move |(x, y)| {
                PathElement::new(vec![(x, y), (x + 20, y)], color.stroke_width(3))
            })
            .label(&language.name);
    }

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .expect("Failed to render labels");
}

fn get_min_date(data: &ChronologicalLookup) -> Option<&NaiveDate> {
    data.values().flat_map(|btree| btree.keys()).min()
}

fn get_max_date(data: &ChronologicalLookup) -> Option<&NaiveDate> {
    data.values().flat_map(|btree| btree.keys()).max()
}

fn convert_color(color: &str) -> RGBColor {
    let color = &color[1..]; // hash symbol prefix
    let decoded = <[u8; 3]>::from_hex(color).expect("Decoding failed");
    RGBColor(decoded[0], decoded[1], decoded[2])
}
