mod mutation;
mod query;

use std::thread::sleep;

use lazy_static::lazy_static;
pub use mutation::*;
use opentelemetry::{global, metrics,
                    trace::{TraceContextExt, Tracer},
                    Key};
pub use query::*;
pub use sea_orm;

const NAME: &str = "io.libra.app";

lazy_static! {
    pub static ref GREET_COUNTER: metrics::UpDownCounter<i64> = global::meter(NAME)
        .i64_up_down_counter("greeting_total")
        .with_description("How many times the app greeted user.")
        .init();
}

pub fn greet(name: &str) -> String {
    let tracer = global::tracer(NAME);
    let _ = tracer.start("greeting-service");
    let greeting = tracer.in_span("choosing-greeting", |cx| {
        let span = cx.span();
        span.add_event("thinking about greeting", vec![]);

        tracer.in_span("choosing-greeting-deeply", |cx| {
            cx.span()
                .add_event("thinking about greeting in deep", vec![]);
        });

        sleep(std::time::Duration::from_secs_f32(0.5));

        let greeting = "Hello";
        cx.span().add_event(
            "made a greeting choice",
            vec![Key::new("greeting").string(greeting)],
        );

        greeting
    });

    let rating = tracer.in_span("admiring-name", |cx| {
        let span = cx.span();
        span.add_event("looking at user's name", vec![]);

        sleep(std::time::Duration::from_secs_f32(0.5));

        let rating = "good";
        cx.span()
            .add_event("rated user's name", vec![Key::new("rating").string(rating)]);

        rating
    });

    GREET_COUNTER.add(1, &[]);
    format!("{greeting}, {name}! ({rating})")
}
