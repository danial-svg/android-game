use macroquad::prelude::*;

#[macroquad::main("MyAndroidGame")]
async fn main() {
    loop {
        clear_background(BROWN);

        // گرفتن موقعیت لمس صفحه یا موس
        let (x, y) = mouse_position();

        // رسم یک دایره زردرنگ که دنبال انگشت/موس می‌رود
        draw_circle(x, y, 50.0, YELLOW);

        draw_text("Touch the screen!", 20.0, 30.0, 30.0, WHITE);

        next_frame().await
    }
}