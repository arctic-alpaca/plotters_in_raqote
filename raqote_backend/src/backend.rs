use plotters_backend::{
    BackendColor, BackendCoord, BackendStyle, BackendTextStyle, DrawingBackend, DrawingErrorKind,
    FontStyle, FontTransform,
};
use raqote::*;

#[derive(Debug)]
pub struct RaqoteError;

impl std::fmt::Display for RaqoteError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for RaqoteError {}

/// The drawing backend that is backed with a Cairo context
pub struct RaqoteBackend<'a> {
    dt: &'a mut DrawTarget,
    width: u32,
    height: u32,
}

impl<'a> RaqoteBackend<'a> {
    pub fn new(dt: &'a mut DrawTarget) -> Result<Self, RaqoteError> {
        let width = dt.width() as u32;
        let height = dt.height() as u32;
        let ret = Self { dt, width, height };
        Ok(ret)
    }
    fn conv_color(&self, color: BackendColor) -> Source {
        let color = raqote::Color::new(color.alpha as u8, color.rgb.0, color.rgb.1, color.rgb.2);
        println!("{:#?}", color);
        raqote::Source::Solid(SolidSource::from(color))
    }
}

impl<'a> DrawingBackend for RaqoteBackend<'a> {
    type ErrorType = RaqoteError;

    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        point: (i32, i32),
        color: BackendColor,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = raqote::Color::new(
            (color.alpha * 255.0) as u8,
            color.rgb.0,
            color.rgb.1,
            color.rgb.2,
        );
        //println!("{:#?}", color);
        let source = raqote::Source::Solid(SolidSource::from(color));
        self.dt.fill_rect(
            point.0 as f32,
            point.1 as f32,
            1.0,
            1.0,
            &source.clone(),
            &raqote::DrawOptions::new(), // {                alpha: color.alpha as f32,                ..Default::default()            },
        );
        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: (i32, i32),
        to: (i32, i32),
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        println!("draw_line");
        let color = raqote::Color::new(
            (style.color().alpha * 255.0) as u8,
            style.color().rgb.0,
            style.color().rgb.1,
            style.color().rgb.2,
        );
        let source = raqote::Source::Solid(SolidSource::from(color));

        let mut pb = PathBuilder::new();
        pb.move_to(from.0 as f32, from.1 as f32);
        pb.line_to(to.0 as f32, to.1 as f32);
        self.dt.stroke(
            &pb.finish(),
            &source,
            &StrokeStyle::default(),
            &DrawOptions::new(),
        );

        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: (i32, i32),
        bottom_right: (i32, i32),
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        println!("draw_rect");
        let color = raqote::Color::new(
            (style.color().alpha * 255.0) as u8,
            style.color().rgb.0,
            style.color().rgb.1,
            style.color().rgb.2,
        );
        let source = raqote::Source::Solid(SolidSource::from(color));
        if fill {
            let width = (bottom_right.0 - upper_left.0).abs() as f32;
            let height = (bottom_right.1 - upper_left.1).abs() as f32;
            self.dt.fill_rect(
                upper_left.0 as f32,
                upper_left.1 as f32,
                width,
                height,
                &source,
                &DrawOptions::new(),
            );
        } else {
            let mut pb = PathBuilder::new();
            pb.move_to(upper_left.0 as f32, upper_left.1 as f32);
            pb.line_to(bottom_right.0 as f32, upper_left.1 as f32);
            pb.line_to(bottom_right.0 as f32, bottom_right.1 as f32);
            pb.line_to(upper_left.0 as f32, bottom_right.1 as f32);
            pb.line_to(upper_left.0 as f32, upper_left.1 as f32);
            self.dt.stroke(
                &pb.finish(),
                &source,
                &StrokeStyle {
                    width: style.stroke_width() as f32,
                    ..StrokeStyle::default()
                },
                &DrawOptions::new(),
            );
        }

        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        println!("draw_path");
        let color = raqote::Color::new(
            (style.color().alpha * 255.0) as u8,
            style.color().rgb.0,
            style.color().rgb.1,
            style.color().rgb.2,
        );
        let source = raqote::Source::Solid(SolidSource::from(color));

        let mut pb = PathBuilder::new();

        let iterator = path.into_iter();
        for (index, point) in iterator.enumerate() {
            if index == 0 {
                pb.move_to(point.0 as f32, point.1 as f32);
            } else {
                pb.line_to(point.0 as f32, point.1 as f32);
            }
        }

        self.dt.stroke(
            &pb.finish(),
            &source,
            &StrokeStyle {
                width: style.stroke_width() as f32,
                ..StrokeStyle::default()
            },
            &DrawOptions::new(),
        );
        Ok(())
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: (i32, i32),
        radius: u32,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        println!("draw_circle");

        let color = raqote::Color::new(
            (style.color().alpha * 255.0) as u8,
            style.color().rgb.0,
            style.color().rgb.1,
            style.color().rgb.2,
        );
        let source = raqote::Source::Solid(SolidSource::from(color));
        let mut pb = PathBuilder::new();
        pb.arc(
            center.0 as f32,
            center.1 as f32,
            radius as f32,
            0.0,
            (std::f64::consts::PI * 2.0) as f32,
        );

        if fill {
            self.dt.fill(&pb.finish(), &source, &DrawOptions::new());
        } else {
            self.dt.stroke(
                &pb.finish(),
                &source,
                &StrokeStyle {
                    width: style.stroke_width() as f32,
                    ..StrokeStyle::default()
                },
                &DrawOptions::new(),
            );
        }

        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        vert: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        println!("draw_polygon");

        let color = raqote::Color::new(
            (style.color().alpha * 255.0) as u8,
            style.color().rgb.0,
            style.color().rgb.1,
            style.color().rgb.2,
        );
        let source = raqote::Source::Solid(SolidSource::from(color));
        let mut pb = PathBuilder::new();
        for (index, point) in vert.into_iter().enumerate() {
            if index == 0 {
                pb.move_to(point.0 as f32, point.1 as f32);
            } else {
                pb.line_to(point.0 as f32, point.1 as f32);
            }
        }
        pb.close();
        self.dt.fill(&pb.finish(), &source, &DrawOptions::new());

        Ok(())
    }
}
