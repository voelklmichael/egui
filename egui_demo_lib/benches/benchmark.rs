use criterion::{criterion_group, criterion_main, Criterion};

use egui::epaint::TextShape;
use egui_demo_lib::LOREM_IPSUM_LONG;

pub fn criterion_benchmark(c: &mut Criterion) {
    use egui::RawInput;

    {
        let ctx = egui::Context::default();
        let mut demo_windows = egui_demo_lib::DemoWindows::default();

        // The most end-to-end benchmark.
        c.bench_function("demo_with_tessellate__realistic", |b| {
            b.iter(|| {
                let (_output, shapes) = ctx.run(RawInput::default(), |ctx| {
                    demo_windows.ui(ctx);
                });
                ctx.tessellate(shapes)
            })
        });

        c.bench_function("demo_no_tessellate", |b| {
            b.iter(|| {
                ctx.run(RawInput::default(), |ctx| {
                    demo_windows.ui(ctx);
                })
            })
        });

        let (_output, shapes) = ctx.run(RawInput::default(), |ctx| {
            demo_windows.ui(ctx);
        });
        c.bench_function("demo_only_tessellate", |b| {
            b.iter(|| ctx.tessellate(shapes.clone()))
        });
    }

    if false {
        let ctx = egui::Context::default();
        ctx.memory().set_everything_is_visible(true); // give us everything
        let mut demo_windows = egui_demo_lib::DemoWindows::default();
        c.bench_function("demo_full_no_tessellate", |b| {
            b.iter(|| {
                ctx.run(RawInput::default(), |ctx| {
                    demo_windows.ui(ctx);
                })
            })
        });
    }

    {
        let ctx = egui::Context::default();
        let _ = ctx.run(RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                c.bench_function("label &str", |b| {
                    b.iter(|| {
                        ui.label("the quick brown fox jumps over the lazy dog");
                    })
                });
                c.bench_function("label format!", |b| {
                    b.iter(|| {
                        ui.label("the quick brown fox jumps over the lazy dog".to_owned());
                    })
                });
            });
        });
    }

    {
        let ctx = egui::Context::default();
        ctx.begin_frame(RawInput::default());

        egui::CentralPanel::default().show(&ctx, |ui| {
            c.bench_function("Painter::rect", |b| {
                let painter = ui.painter();
                let rect = ui.max_rect();
                b.iter(|| {
                    painter.rect(rect, 2.0, egui::Color32::RED, (1.0, egui::Color32::WHITE));
                })
            });
        });

        // Don't call `end_frame` to not have to drain the huge paint list
    }

    {
        let pixels_per_point = 1.0;
        let wrap_width = 512.0;
        let text_style = egui::TextStyle::Body;
        let color = egui::Color32::WHITE;
        let fonts =
            egui::epaint::text::Fonts::new(pixels_per_point, egui::FontDefinitions::default());
        c.bench_function("text_layout_uncached", |b| {
            b.iter(|| {
                use egui::epaint::text::{layout, LayoutJob};

                let job = LayoutJob::simple(
                    LOREM_IPSUM_LONG.to_owned(),
                    egui::TextStyle::Body,
                    color,
                    wrap_width,
                );
                layout(&fonts, job.into())
            })
        });
        c.bench_function("text_layout_cached", |b| {
            b.iter(|| fonts.layout(LOREM_IPSUM_LONG.to_owned(), text_style, color, wrap_width))
        });

        let galley = fonts.layout(LOREM_IPSUM_LONG.to_owned(), text_style, color, wrap_width);
        let mut tessellator = egui::epaint::Tessellator::from_options(Default::default());
        let mut mesh = egui::epaint::Mesh::default();
        let text_shape = TextShape::new(egui::Pos2::ZERO, galley);
        c.bench_function("tessellate_text", |b| {
            b.iter(|| {
                tessellator.tessellate_text(
                    fonts.font_image().size(),
                    text_shape.clone(),
                    &mut mesh,
                );
                mesh.clear();
            })
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
