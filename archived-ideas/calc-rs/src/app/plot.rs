use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

use crate::helpers::{myerror, AppResult};

use super::App;
/*
   pub(crate) fn plot(&mut self) -> AppResult<()> {
       use self:
       let program = self.take_data();
       dbg!(program);

       Ok(())
   }

*/
impl App {
    pub(crate) fn plot(&mut self) -> AppResult<()> {
        let points = self.take_data().ok_or(myerror("No parsed datapoint!"))?;

        dbg!(&points);

        // Scatter plots expect a list of pairs
        // let data1 = vec![(-3.0, 2.3), (-1.6, 5.3), (0.3, 0.7), (4.3, -1.4), (6.4, 4.3), (8.5, 3.7)];

        // We create our scatter plot from the data
        let plot1: Plot = Plot::new(points.into()).point_style(
            PointStyle::new()
                .marker(PointMarker::Square) // setting the marker to be a square
                .colour("#DD3355"),
        ); // and a custom colour

        // We can plot multiple data sets in the same view
        let data2 = vec![(-1.4, 2.5), (7.2, -0.3)];
        let plot2: Plot = Plot::new(data2).point_style(
            PointStyle::new() // uses the default marker
                .colour("#35C788"),
        ); // and a different colour

        // The 'view' describes what set of data is drawn
        let v = ContinuousView::new()
            .add(plot1)
            .add(plot2)
            .x_range(-5., 10.)
            .y_range(-2., 6.)
            .x_label("Some varying variable")
            .y_label("The response of something");

        // A page with a single view is then saved to an SVG file
        Page::single(&v)
            .save("plot.svg")
            .map_err(|err| myerror(format!("{}{}: {}", file!(), line!(), err)))?;

        Ok(())
    }
}
