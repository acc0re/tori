use macroquad::prelude::*;

/// Represents a sprite with a texture, position, and scale.
pub struct Sprite {
    /// The texture of the sprite.
    pub texture: Texture2D,
    /// The position of the sprite.
    pub position: Vec2,
    /// The scale of the sprite.
    pub scale: f32,
}

impl Sprite {
    /// Creates a new sprite.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the path to the texture file.
    /// * `position` - A `Vec2` that holds the initial position of the sprite.
    /// * `scale` - A `f32` that holds the scale of the sprite.
    ///
    /// # Returns
    ///
    /// A new `Sprite` instance.
    pub async fn new(path: &str, position: Vec2, scale: f32) -> Self {
        let texture = load_texture(path).await.unwrap();
        texture.set_filter(FilterMode::Nearest);

        Self { texture, position, scale }
    }

    /// Draws the sprite on the screen.
    pub fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.scale * vec2(self.texture.width(), self.texture.height())),
                ..Default::default()
            },
        );
    }

    /// Sets a new position for the sprite.
    ///
    /// # Arguments
    ///
    /// * `new_position` - A `Vec2` that holds the new position of the sprite.
    pub fn set_position(&mut self, new_position: Vec2) {
        self.position = new_position;
    }

    /// Moves the sprite by a given delta.
    ///
    /// # Arguments
    ///
    /// * `delta` - A `Vec2` that holds the change in position.
    pub fn move_by(&mut self, delta: Vec2) {
        self.position += delta;
    }
}