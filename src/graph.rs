use chrono::{Datelike, NaiveDate, TimeZone, Utc};
use directories::UserDirs;
use log::info;
use plotters::{
    prelude::{
        BitMapBackend, ChartBuilder, IntoDrawingArea, IntoMonthly, LabelAreaPosition, LineSeries,
        PathElement,
    },
    style::{Color, IntoFont, RGBColor, BLACK, BLUE, CYAN, GREEN, MAGENTA, RED, WHITE, YELLOW},
};

use crate::ChronologicalLookup;

pub fn create_graph(data: &ChronologicalLookup, chart_name: String) {
    info!("Creating graph");
    let output_file = UserDirs::new()
        .expect("Could not find a HOME directory")
        .desktop_dir()
        .expect("No Desktop directory found")
        .join("out.png");

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
            0.0f64..100.0f64,
        )
        .expect("Failed to set chart axis");

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .y_label_formatter(&|x| format!("{:.2}%", x))
        .draw()
        .expect("Failed to render mesh");

    info!("Creating {} series", data.len());

    for (index, (language, values)) in data.iter().enumerate() {
        info!("{:?}", values);
        let color = get_color(index);

        chart
            .draw_series(LineSeries::new(
                values
                    .iter()
                    .map(|(date, pct)| (Utc.ymd(date.year(), date.month(), date.day()), *pct)),
                color,
            ))
            .expect("Failed to draw series")
            .legend(|(x, y)| {
                PathElement::new(
                    vec![(x, y), (x + 20, y)],
                    RGBColor(x as u8, x as u8, x as u8),
                )
            })
            .label(&language.name);
    }

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .background_style(&CYAN.mix(0.8))
        .draw()
        .expect("Failed to render labels");
}

fn get_color(index: usize) -> RGBColor {
    let colors = vec![RED, BLUE, BLACK, GREEN, YELLOW, CYAN, MAGENTA];
    colors[index % colors.len()]
}

fn get_min_date(data: &ChronologicalLookup) -> Option<&NaiveDate> {
    data.values().flat_map(|btree| btree.keys()).min()
}

fn get_max_date(data: &ChronologicalLookup) -> Option<&NaiveDate> {
    data.values().flat_map(|btree| btree.keys()).max()
}
