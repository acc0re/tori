use macroquad::prelude::*;

/// Represents an animated sprite with multiple frames, position, and scale.
pub struct AnimatedSprite {
    /// The texture containing all animation frames.
    pub texture: Texture2D,
    /// The position of the sprite.
    pub position: Vec2,
    /// The scale of the sprite.
    pub scale: f32,
    /// The size of a single frame in pixels.
    pub frame_size: Vec2,
    /// The total number of frames in the animation.
    pub frame_count: usize,
    /// The current frame index.
    pub current_frame: usize,
    /// The speed of the animation (frames per second).
    pub frame_speed: f32,
    /// Internal timer to track animation progress.
    pub frame_timer: f32,
    /// Direction of animation (true = forward, false = backward).
    pub is_reversed: bool,

    /// Rotation of the sprite in degrees.
    pub rotation: f32,
}

impl AnimatedSprite {
    /// Creates a new animated sprite.
    pub async fn new(
        path: &str,
        position: Vec2,
        scale: f32,
        rotation: f32,
        frame_size: Vec2,
        frame_count: usize,
        frame_speed: f32
    ) -> Self {
        let texture = load_texture(path).await.unwrap();
            texture.set_filter(FilterMode::Nearest);

        Self {
            texture,
            position,
            scale,
            rotation,
            frame_size,
            frame_count,
            current_frame: 0,
            frame_speed,
            frame_timer: 0.0,
            is_reversed: false,
        }
    }

    /// Updates the animation frame based on time and direction.
    pub fn update(&mut self, delta_time: f32) {
        self.frame_timer += delta_time;

        // Wenn der Timer die Zeit für das nächste Frame überschreitet
        if self.frame_timer >= 1.0 / self.frame_speed {
            self.frame_timer = 0.0;

            // Vorwärtsbewegung, wenn `is_reversed` false
            if !self.is_reversed {
                self.current_frame += 1;
                // Wenn das Ende erreicht ist, kehre die Richtung um
                if self.current_frame >= self.frame_count {
                    self.current_frame = self.frame_count - 1;
                    self.is_reversed = true; // Wechsel zur Rückwärtsrichtung
                }
            }
            // Rückwärtsbewegung, wenn `is_reversed` true
            else {
                self.current_frame -= 1;
                // Wenn der Anfang erreicht ist, kehre die Richtung um
                if self.current_frame == 0 {
                    self.current_frame = 0;
                    self.is_reversed = false; // Wechsel zur Vorwärtsrichtung
                }
            }
        }
    }

    /// Draws the current frame of the animation.
    /// Draws the current frame of the animation.
    pub fn draw(&self) {
        // Runde Position auf Ganzzahlen
        let rounded_position = Vec2::new(self.position.x.round(), self.position.y.round());

        let src_rect = Rect {
            x: self.current_frame as f32 * self.frame_size.x,
            y: 0.0, // Falls alle Frames horizontal angeordnet sind
            w: self.frame_size.x,
            h: self.frame_size.y,
        };

        draw_texture_ex(
            &self.texture,
            rounded_position.x,
            rounded_position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.scale * self.frame_size),
                source: Some(src_rect),
                rotation: self.rotation,
                ..Default::default()
            },
        );
    }


    /// Sets a new position for the sprite.
    pub fn set_position(&mut self, new_position: Vec2) {
        self.position = new_position;
    }

    /// Moves the sprite by a given delta.
    pub fn move_by(&mut self, delta: Vec2) {
        self.position += delta;
    }
}
