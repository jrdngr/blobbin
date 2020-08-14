#[macro_export]
macro_rules! vector2_impl {
    ($impl_type:ty) => {
        impl std::ops::Add<&Vector2<$impl_type>> for &Vector2<$impl_type> {
            type Output = Vector2<$impl_type>;
        
            fn add(self, rhs: &Vector2<$impl_type>) -> Self::Output {
                Vector2 {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl std::ops::AddAssign<&Vector2<$impl_type>> for Vector2<$impl_type> {
            fn add_assign(&mut self, rhs: &Vector2<$impl_type>) {
                self.x += rhs.x;
                self.y += rhs.y;
            }
        }

        impl std::ops::Add<Vector2<$impl_type>> for &Vector2<$impl_type> {
            type Output = Vector2<$impl_type>;
        
            fn add(self, rhs: Vector2<$impl_type>) -> Self::Output {
                Vector2 {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }
        
        impl std::ops::AddAssign<Vector2<$impl_type>> for Vector2<$impl_type> {
            fn add_assign(&mut self, rhs: Vector2<$impl_type>) {
                self.x += rhs.x;
                self.y += rhs.y;
            }
        }
        
        impl std::ops::Sub<&Vector2<$impl_type>> for &Vector2<$impl_type> {
            type Output = Vector2<$impl_type>;
        
            fn sub(self, rhs: &Vector2<$impl_type>) -> Self::Output {
                Vector2 {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }
        
        impl std::ops::SubAssign<&Vector2<$impl_type>> for Vector2<$impl_type> {
            fn sub_assign(&mut self, rhs: &Vector2<$impl_type>) {
                self.x -= rhs.x;
                self.y -= rhs.y;
            }
        }
        
        impl std::ops::Sub<Vector2<$impl_type>> for &Vector2<$impl_type> {
            type Output = Vector2<$impl_type>;
        
            fn sub(self, rhs: Vector2<$impl_type>) -> Self::Output {
                Vector2 {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }
        
        impl std::ops::SubAssign<Vector2<$impl_type>> for Vector2<$impl_type> {
            fn sub_assign(&mut self, rhs: Vector2<$impl_type>) {
                self.x -= rhs.x;
                self.y -= rhs.y;
            }
        }
        
        impl std::ops::Mul<$impl_type> for &Vector2<$impl_type> {
            type Output = Vector2<$impl_type>;
        
            fn mul(self, rhs: $impl_type) -> Self::Output {
                Vector2 {
                    x: self.x * rhs,
                    y: self.y * rhs,
                }
            }
        }
        
        impl std::ops::Mul<$impl_type> for Vector2<$impl_type> {
            type Output = Vector2<$impl_type>;
        
            fn mul(self, rhs: $impl_type) -> Self::Output {
                Vector2 {
                    x: self.x * rhs,
                    y: self.y * rhs,
                }
            }
        }
        
        impl std::ops::Mul<&Vector2<$impl_type>> for $impl_type {
            type Output = Vector2<$impl_type>;
        
            fn mul(self, rhs: &Vector2<$impl_type>) -> Self::Output {
                Vector2 {
                    x: self * rhs.x,
                    y: self * rhs.y,
                }
            }
        }
        
        impl std::ops::Mul<Vector2<$impl_type>> for $impl_type {
            type Output = Vector2<$impl_type>;
        
            fn mul(self, rhs: Vector2<$impl_type>) -> Self::Output {
                Vector2 {
                    x: self * rhs.x,
                    y: self * rhs.y,
                }
            }
        }
        
        impl std::ops::MulAssign<$impl_type> for Vector2<$impl_type> {
            fn mul_assign(&mut self, rhs: $impl_type) {
                self.x *= rhs;
                self.y *= rhs;
            }
        }
        
        impl std::ops::Div<$impl_type> for &Vector2<$impl_type> {
            type Output = Vector2<$impl_type>;
        
            fn div(self, rhs: $impl_type) -> Self::Output {
                Vector2 {
                    x: self.x / rhs,
                    y: self.y / rhs,
                }
            }
        }
        
        impl std::ops::Div<$impl_type> for Vector2<$impl_type> {
            type Output = Vector2<$impl_type>;
        
            fn div(self, rhs: $impl_type) -> Self::Output {
                Vector2 {
                    x: self.x / rhs,
                    y: self.y / rhs,
                }
            }
        }
        
        impl std::ops::DivAssign<$impl_type> for Vector2<$impl_type> {
            fn div_assign(&mut self, rhs: $impl_type) {
                self.x /= rhs;
                self.y /= rhs;
            }
        }

        impl Vector2<$impl_type> {
            pub fn magnitude(&self) -> f64 {
                let sum_squares = (self.x * self.x) + (self.y * self.y);
                (sum_squares as f64).sqrt()
            }
        
            pub fn distance(&self, other: &Vector2<$impl_type>) -> f64 {
                self.vector_to(other).magnitude()
            }
        
            pub fn vector_to(&self, target: &Vector2<$impl_type>) -> Vector2<$impl_type> {
                self - target
            }
        
            pub fn normalized(&self) -> Vector2<f64> {
                let magnitude = self.magnitude();
                Vector2 {
                    x: self.x as f64 / magnitude,
                    y: self.y as f64 / magnitude,
                }
            }
        }
    };
}
