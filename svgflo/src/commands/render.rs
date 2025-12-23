use crate::arguments::*;

use flo_canvas::*;
use flo_svg::*;
use flo_render_software::draw::*;
use flo_render_software::pixel::*;
use flo_render_software::render::*;
use flo_render_software::scanplan::*;

use rascii_art;
use image::{DynamicImage, ImageBuffer};

use std::fs::*;
use std::io::*;

///
/// Renders a SVG file to a PNG file
///
pub fn render(arguments: &SvgFloCli, svg_file: &String) {
    // Set up the output file
    // TODO: file error handling
    let is_terminal;

    let mut output: BufWriter<Box<dyn Write>> = if let Some(output_file) = &arguments.output {
        let output_file     = File::create(output_file).unwrap();
        is_terminal         = output_file.is_terminal();
        
        BufWriter::new(Box::new(output_file))
    } else {
        let stdout  = stdout();
        is_terminal = stdout.is_terminal();

        BufWriter::new(Box::new(stdout))
    };

    // Read the svg file
    let svg_file = File::open(svg_file).unwrap();
    let svg_file = BufReader::new(svg_file);
    let svg_file = svg_file.bytes().collect::<Result<Vec<u8>>>().unwrap();
    let svg_file = String::from_utf8_lossy(&svg_file);

    // Parse the svg file
    let mut drawing = vec![];
    let document    = parse_svg(svg_file, &mut drawing).unwrap();

    // Figure out the size of the region to render on
    let render_width    = document.render_width().unwrap_or(1024.0);
    let render_height   = document.render_height().unwrap_or(768.0);

    // TODO: Use width, height properties to figure out the pixel width/height (will need to scale them if one is missing. x, y coordinates don't matter for us)
    let pixel_width     = render_width.ceil() as usize;
    let pixel_height    = render_height.ceil() as usize;

    // Create the setup steps for this rendering
    let mut setup_steps = vec![];

    setup_steps.clear_canvas(Color::Rgba(0.0, 0.0, 0.0, 0.0));
    setup_steps.canvas_height(render_height);
    setup_steps.transform(Transform2D::scale(1.0, -1.0));

    if let Some(viewbox) = document.viewbox() {
        setup_steps.center_region(viewbox.0.0, viewbox.0.1, viewbox.1.0, viewbox.1.1);
    } else {
        setup_steps.center_region(0.0, 0.0, render_width, render_height);
    }

    // Render the image
    let mut canvas_drawing = CanvasDrawing::<F32LinearPixel, 4>::empty();
    canvas_drawing.draw(setup_steps);
    canvas_drawing.draw(drawing);

    if is_terminal {
        // Ascii art if we're on the terminal
        let mut frame   = vec![0u8; pixel_width*pixel_height*4];
        let mut rgba    = FrameU8Rgba::from_bytes(pixel_width, pixel_height, 2.2, &mut frame).unwrap();
        let renderer    = CanvasDrawingRegionRenderer::new(ShardScanPlanner::default(), ScanlineRenderer::new(canvas_drawing.program_runner(1080.0)), 1080);

        // Render to the frame
        rgba.render(renderer, &canvas_drawing);

        let image   = DynamicImage::ImageRgba8(ImageBuffer::from_vec(pixel_width as _, pixel_height as _, frame).unwrap());
        let options = rascii_art::RenderOptions::new().colored(true).width(79).charset(rascii_art::charsets::BLOCK);
        rascii_art::render_image(&image, &mut output, &options).unwrap();

        output.write(&[10, 10]).unwrap();
    } else {
        // Render as a png file if not attached to a terminal
        let mut render_target   = PngRenderTarget::from_bufwriter(output, pixel_width, pixel_height, 2.2);
        let renderer            = CanvasDrawingRegionRenderer::new(ShardScanPlanner::default(), ScanlineRenderer::new(canvas_drawing.program_runner(1080.0)), 1080);

        render_target.render(renderer, &canvas_drawing);
    }
}
