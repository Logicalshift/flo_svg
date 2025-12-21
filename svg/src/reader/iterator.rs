use super::svg_reader_state::*;

use flo_canvas::*;

use svg::parser;

///
/// Given a iterator of SVG parser events, generates a stream of `Draw` events that describes how to render it on screen
///
pub fn svg_parser_events_to_drawing<'a>(events: impl 'a + Send + Iterator<Item=parser::Event<'a>>) -> impl 'a + Send + Iterator<Item=Draw> {
    // Create the initial state
    let mut state = SvgReaderState::default();

    // Send the events through the state to generate drawing requests
    events
        .flat_map(move |event| {
            let mut drawing = vec![];

            state.accept(event, &mut drawing);

            drawing
        })
}
