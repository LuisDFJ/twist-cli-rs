use super::Graph;
use ratatui::prelude::*;
use ratatui::widgets::{Chart,Axis,Dataset,GraphType};

impl Widget for &Graph {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized
    {
        let ds = Dataset::default()
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .red()
            .data(&self.data[..]);
        let x_axis = Axis::default()
            .title(self.units.x.clone().unwrap_or( "x axis".to_string() ).blue() )
            .labels(marks(self.x_limits))
            .bounds([self.x_limits.0,self.x_limits.1]);
        let y_axis = Axis::default()
            .title(self.units.y.clone().unwrap_or( "y axi".to_string() ).blue() )
            .labels(marks(self.y_limits))
            .bounds([self.y_limits.0,self.y_limits.1]);
        let chart = Chart::new(vec![ds])
            .x_axis(x_axis)
            .y_axis(y_axis);
        chart.render(area, buf);
    }
}

fn marks( l : (f64,f64) ) -> [String;4] {
    [
        format!("{:7.2}", l.0),
        format!("{:7.2}", (l.0 + l.1) * 1.0 / 3.0),
        format!("{:7.2}", (l.0 + l.1) * 2.0 / 3.0),
        format!("{:7.2}", l.1),
    ]
}
