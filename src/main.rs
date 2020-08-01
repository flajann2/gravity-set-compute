#[macro_use]
extern crate conrod;
mod mgs_gui;

fn main() {
    println!("MGS -- launched");
    feature::main();
}

mod feature {
    extern crate find_folder;
    extern crate piston_window;
    use conrod;
    use mgs_gui;

    use self::piston_window::texture::UpdateTexture;
    use self::piston_window::OpenGL;
    use self::piston_window::{Flip, G2d, G2dTexture, Texture, TextureSettings};
    use self::piston_window::{PistonWindow, UpdateEvent, Window, WindowSettings};

    pub fn main() {
        const WIDTH: u32 = mgs_gui::WIN_W;
        const HEIGHT: u32 = mgs_gui::WIN_H;

        // Construct the window.
        let mut window: PistonWindow =
            WindowSettings::new("All Widgets - Piston Backend", [WIDTH, HEIGHT])
                .opengl(OpenGL::V3_2) // If not working, try `OpenGL::V2_1`.
                .samples(4)
                .exit_on_esc(true)
                .vsync(true)
                .build()
                .unwrap();

        let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64])
            .theme(mgs_gui::theme())
            .build();

        let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("assets")
            .unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        ui.fonts.insert_from_file(font_path).unwrap();

        // Create a texture to use for efficiently caching text on the GPU.
        let mut text_vertex_data = Vec::new();
        let (mut glyph_cache, mut text_texture_cache) = {
            const SCALE_TOLERANCE: f32 = 0.1;
            const POSITION_TOLERANCE: f32 = 0.1;
            let cache =
                conrod::text::GlyphCache::new(WIDTH, HEIGHT, SCALE_TOLERANCE, POSITION_TOLERANCE);
            let buffer_len = WIDTH as usize * HEIGHT as usize;
            let init = vec![128; buffer_len];
            let settings = TextureSettings::new();
            let factory = &mut window.factory;
            let texture =
                G2dTexture::from_memory_alpha(factory, &init, WIDTH, HEIGHT, &settings).unwrap();
            (cache, texture)
        };

        let ids = mgs_gui::Ids::new(ui.widget_id_generator());

        let mgs_logo: G2dTexture = {
            let assets = find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets")
                .unwrap();
            let path = assets.join("images/mgs3_logo.png");
            let factory = &mut window.factory;
            let settings = TextureSettings::new();
            Texture::from_path(factory, &path, Flip::None, &settings).unwrap()
        };

        let mut image_map = conrod::image::Map::new();
        let mgs_logo = image_map.insert(mgs_logo);

        let mut app = mgs_gui::MGSApp::new(mgs_logo);

        while let Some(event) = window.next() {
            // Convert the piston event to a conrod event.
            let size = window.size();
            let (win_w, win_h) = (size.width as conrod::Scalar, size.height as conrod::Scalar);
            if let Some(e) = conrod::backend::piston::event::convert(event.clone(), win_w, win_h) {
                ui.handle_event(e);
            }

            event.update(|_| {
                let mut ui = ui.set_widgets();
                mgs_gui::gui(&mut ui, &ids, &mut app);
            });

            window.draw_2d(&event, |context, graphics| {
                if let Some(primitives) = ui.draw_if_changed() {
                    // A function used for caching glyphs to the texture cache.
                    let cache_queued_glyphs =
                        |graphics: &mut G2d,
                         cache: &mut G2dTexture,
                         rect: conrod::text::rt::Rect<u32>,
                         data: &[u8]| {
                            let offset = [rect.min.x, rect.min.y];
                            let size = [rect.width(), rect.height()];
                            let format = piston_window::texture::Format::Rgba8;
                            let encoder = &mut graphics.encoder;
                            text_vertex_data.clear();
                            text_vertex_data
                                .extend(data.iter().flat_map(|&b| vec![255, 255, 255, b]));
                            UpdateTexture::update(
                                cache,
                                encoder,
                                format,
                                &text_vertex_data[..],
                                offset,
                                size,
                            )
                            .expect("failed to update texture")
                        };

                    // Specify how to get the drawable texture from the image. In this case, the image
                    // *is* the texture.
                    fn texture_from_image<T>(img: &T) -> &T {
                        img
                    }

                    // Draw the conrod `render::Primitives`.
                    conrod::backend::piston::draw::primitives(
                        primitives,
                        context,
                        graphics,
                        &mut text_texture_cache,
                        &mut glyph_cache,
                        &image_map,
                        cache_queued_glyphs,
                        texture_from_image,
                    );
                }
            });
        }
    }
}