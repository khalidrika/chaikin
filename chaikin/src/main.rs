mod app;
use app::*;
use macroquad::prelude::*;

#[macroquad::main("Chaikin's Algorithm")]
async fn main() {
    let mut app: App = App::new();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::R) {
            app.reset();
        }

        if is_key_pressed(KeyCode::Enter) {
            if !app.start_animation {
                app.chaikin_points = app.default_points.clone();
            }

            app.start_animation = true;
        }

        if app.start_animation {
            app.animate();
        } else {
            if is_mouse_button_pressed(MouseButton::Left) {
                let (x, y) = mouse_position();
                app.new_point(x, y);
            }
        }

        for point in &app.default_points {
            draw_circle(point.x, point.y, 3.0, WHITE);
            draw_circle(point.x, point.y, 2.0, BLACK);
        }
        draw_ui(&app);

        next_frame().await;
    }
}
