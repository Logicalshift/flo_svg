use flo_canvas::*;

use svg;
use svg::node::element::path::*;
use svg::node::element::tag::*;
use svg::parser::*;
use svgtypes::*;
use simplecss::*;
use css_color_parser::Color as CssColor;

use flo_canvas::Color as Color;

use flo_draw::*;

fn parse_event(event: Event, canvas: &mut Vec<Draw>) {
    match event {
        Event::Tag("svg", Type::Start, _attributes) => {
        },

        Event::Tag("g", Type::Start, attributes) => {
            canvas.push_state();

            if let Some(transform) = attributes.get("transform") {
                let transform: Transform            = transform.parse().unwrap();
                let Transform{ a, b, c, d, e, f }   = transform;
                let transform                       = Transform2D([[a as _, c as _, e as _], [b as _, d as _, f as _], [0.0, 0.0, 1.0]]);

                canvas.transform(transform);
            }
        },

        Event::Tag("g", Type::End, _attributes) => {
            canvas.pop_state();
        },

        Event::Tag("path", Type::Start, attributes) |
        Event::Tag("path", Type::Empty, attributes) => {
            let data = attributes.get("d").unwrap();
            let data = Data::parse(data).unwrap();

            canvas.new_path();

            for command in data.iter() {
                match &command {
                    Command::Move(Position::Absolute, param)        => canvas.move_to(param[0] as _, param[1] as _),
                    Command::Line(Position::Absolute, param)        => canvas.line_to(param[0] as _, param[1] as _),
                    Command::CubicCurve(Position::Absolute, param)  => canvas.bezier_curve_to(param[4] as _, param[5] as _, param[0] as _, param[1] as _, param[2] as _, param[3] as _),
                    Command::Close                                  => canvas.close_path(),
                    _                                               => { println!("Other: {:?}", command); }
                }
            }

            if let Some(style) = attributes.get("style") {
                let style       = format!("path {{ {} }}", style);
                let style       = StyleSheet::parse(&style);
                let rule        = &style.rules[0];

                let mut fill    = None;
                let mut stroke  = None;

                for decl in rule.declarations.iter() {
                    match decl.name {
                        "fill"              => {
                            if decl.value != "none" {
                                let col = decl.value.parse::<CssColor>().unwrap();
                                fill    = Some(Color::Rgba((col.r as f32)/255.0, (col.g as f32)/255.0, (col.b as f32)/255.0, col.a));
                            } else {
                                fill    = None;
                            }
                        }
                        "fill-opacity"      => { 
                            let alpha           = decl.value.parse::<f32>().unwrap();
                            let new_fill        = fill.unwrap_or(Color::Rgba(0.0, 0.0, 0.0, 1.0));
                            let (r, g, b, _a)   = new_fill.to_rgba_components();
                            fill                = Some(Color::Rgba(r, g, b, alpha));
                        }
                        "fill-rule"         => { }

                        "stroke"            => { 
                            if decl.value != "none" {
                                let col = decl.value.parse::<CssColor>().unwrap();
                                stroke  = Some(Color::Rgba((col.r as f32)/255.0, (col.g as f32)/255.0, (col.b as f32)/255.0, col.a));
                            } else {
                                stroke  = None;
                            }
                        }
                        "stroke-opacity"    => { 
                            let alpha           = decl.value.parse::<f32>().unwrap();
                            let new_stroke      = stroke.unwrap_or(Color::Rgba(0.0, 0.0, 0.0, 1.0));
                            let (r, g, b, _a)   = new_stroke.to_rgba_components();
                            stroke              = Some(Color::Rgba(r, g, b, alpha));
                        }
                        "stroke-width"      => { 
                            if decl.value.ends_with("px") {
                                canvas.line_width(decl.value[0..(decl.value.len()-2)].parse().unwrap());
                            } else {
                                println!("?? {:?}", decl.value);
                            }
                        }
                        "stroke-linecap"    => { }
                        "stroke-miterlimit" => { }


                        _ => { println!("Other decl: {:?}", decl.name); }
                    }
                }

                if let Some(fill) = fill {
                    canvas.fill_color(fill);
                    canvas.fill();
                }

                if let Some(stroke) = stroke {
                    canvas.stroke_color(stroke);
                    canvas.stroke();
                }
            }
        }
        _ => { }
    }
}

fn main() {
    let mut canvas  = vec![];

    canvas.clear_canvas(Color::Rgba(78.0/255.0, 189.0/255.0, 134.0/255.0, 1.0));
    canvas.canvas_height(768.0);
    canvas.transform(Transform2D::scale(1.0, -1.0));
    canvas.center_region(0.0, 0.0, 1024.0, 768.0);

    let path        = "vectors.svg";
    let mut content = String::new();

    for event in svg::open(path, &mut content).unwrap() {
        parse_event(event, &mut canvas);
    }

    let mut encoded = String::new();
    canvas.encode_canvas(&mut encoded);
    println!("{}", encoded);

    with_2d_graphics(move || {
        let window = create_canvas_window("svg");
        window.draw(|gc| {
            gc.clear_canvas(Color::Rgba(78.0/255.0, 189.0/255.0, 134.0/255.0, 1.0));
            gc.canvas_height(768.0);
            gc.transform(Transform2D::scale(1.0, -1.0));
            gc.center_region(0.0, 0.0, 1024.0, 768.0);
        });
        window.write(canvas);
    })

    //println!("{:?}", canvas);
}
