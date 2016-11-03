mod vec2{
    use std::ops::Add;

    struct Vec2{
        x: f32,
        y: f32,
    }

    impl Add for Vec2 {
        type Output = Vec2;
        #[inline(always)]
        fn add(self, other: Vec2) -> Vec2 {
            Vec2{
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }


}