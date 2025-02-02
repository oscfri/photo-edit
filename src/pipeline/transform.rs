use cgmath;

#[derive(Debug)]
pub struct Rectangle {
    pub center_x: f32,
    pub center_y: f32,
    pub width: f32,
    pub height: f32,
    pub angle_degrees: f32
}

impl Default for Rectangle {
    fn default() -> Self {
        Self { center_x: 0.0, center_y: 0.0, width: 1.0, height: 1.0, angle_degrees: 0.0 }
    }
}

pub fn transform(from: &Rectangle, to: &Rectangle) -> cgmath::Matrix4<f32> {
    let from_center: cgmath::Matrix4<f32> = translate_transform(-from.center_x, -from.center_y);
    let from_rotate: cgmath::Matrix4<f32> = rotate_transform(-from.angle_degrees);
    let from_scale: cgmath::Matrix4<f32> = scale_transform(1.0 / from.width, 1.0 / from.height);
    let to_scale: cgmath::Matrix4<f32> = scale_transform(to.width, to.height);
    let to_rotate: cgmath::Matrix4<f32> = rotate_transform(to.angle_degrees);
    let to_center: cgmath::Matrix4<f32> = translate_transform(to.center_x, to.center_y);
    from_center * from_rotate * from_scale * to_scale * to_rotate * to_center
}

fn translate_transform(x: f32, y: f32) -> cgmath::Matrix4<f32> {
    cgmath::Matrix4::new(
        1.0, 0.0, 0.0, x,
        0.0, 1.0, 0.0, y,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn scale_transform(x: f32, y: f32) -> cgmath::Matrix4<f32> {
    cgmath::Matrix4::new(
        x, 0.0, 0.0, 0.0,
        0.0, y, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn rotate_transform(angle_degrees: f32) -> cgmath::Matrix4<f32> {
    // Clockwise rotation
    let angle_radians: f64 = (angle_degrees as f64) / 180.0 * std::f64::consts::PI;
    let cos: f32 = f64::cos(angle_radians) as f32;
    let sin: f32 = f64::sin(angle_radians) as f32;
    cgmath::Matrix4::new(
        cos, -sin, 0.0, 0.0,
        sin, cos, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use cgmath::Matrix;

    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32
    }
    
    impl Point {
        fn new(x: f32, y: f32) -> Self {
            Self { x, y }
        }
    }
    
    fn transform_point(point: &Point, from: &Rectangle, to: &Rectangle) -> Point {
        let matrix = transform(from, to);
        let vector = matrix.transpose() * cgmath::vec4(point.x, point.y, 0.0, 1.0);
        Point {
            x: vector.x,
            y: vector.y
        }
    }

    impl Rectangle {
        fn new(center_x: f32, center_y: f32, width: f32, height: f32, angle_degrees: f32) -> Self {
            Self {
                center_x,
                center_y,
                width,
                height,
                angle_degrees
            }
        }

        fn translated(center_x: f32, center_y: f32) -> Self {
            Self {
                center_x,
                center_y,
                width: 1.0,
                height: 1.0,
                angle_degrees: 0.0
            }
        }

        fn scaled(width: f32, height: f32) -> Self {
            Self {
                center_x: 0.0,
                center_y: 0.0,
                width,
                height,
                angle_degrees: 0.0
            }
        }
    
        fn angled(angle_degrees: f32) -> Self {
            Self {
                center_x: 0.0,
                center_y: 0.0,
                width: 1.0,
                height: 1.0,
                angle_degrees
            }
        }
    }

    #[rstest]
    #[case(0.0, 0.0, 2.0, 2.0)]
    #[case(-1.0, 0.0, 1.0, 2.0)]
    #[case(0.0, -1.0, 2.0, 1.0)]
    #[case(10.0, 10.0, 12.0, 12.0)]
    #[case(-10.0, 10.0, -8.0, 12.0)]
    fn transform_square_translation(#[case] x: f32, #[case] y: f32, #[case] expected_x: f32, #[case] expected_y: f32) {
        // Arrange
        let from: Rectangle = Rectangle::translated(-1.0, -1.0);
        let to: Rectangle = Rectangle::translated(1.0, 1.0);
        let point: Point = Point::new(x, y);
        let expected: Point = Point::new(expected_x, expected_y);

        // Act
        let actual = transform_point(&point, &from, &to);

        // Assert
        assert_point_equal(&actual, &expected);
    }

    #[rstest]
    #[case(0.0, 0.0, 0.0, 0.0)]
    #[case(1.0, 1.0, 4.0, 4.0)]
    #[case(-1.0, -1.0, -4.0, -4.0)]
    #[case(-1.0, 1.0, -4.0, 4.0)]
    fn transform_square_scale(#[case] x: f32, #[case] y: f32, #[case] expected_x: f32, #[case] expected_y: f32) {
        // Arrange
        let from: Rectangle = Rectangle::scaled(0.5, 0.5);
        let to: Rectangle = Rectangle::scaled(2.0, 2.0);
        let point: Point = Point::new(x, y);
        let expected: Point = Point::new(expected_x, expected_y);

        // Act
        let actual = transform_point(&point, &from, &to);

        // Assert
        assert_point_equal(&actual, &expected);
    }

    #[rstest]
    #[case(0.0, 0.0, 0.0, 0.0)]
    #[case(1.0, 1.0, 4.0, 1.5)]
    #[case(-1.0, -1.0, -4.0, -1.5)]
    #[case(-1.0, 1.0, -4.0, 1.5)]
    fn transform_rectangle_scale(#[case] x: f32, #[case] y: f32, #[case] expected_x: f32, #[case] expected_y: f32) {
        // Arrange
        let from: Rectangle = Rectangle::scaled(0.5, 1.0);
        let to: Rectangle = Rectangle::scaled(2.0, 1.5);
        let point: Point = Point::new(x, y);
        let expected: Point = Point::new(expected_x, expected_y);

        // Act
        let actual = transform_point(&point, &from, &to);

        // Assert
        assert_point_equal(&actual, &expected);
    }

    #[rstest]
    #[case(0.0, 0.0, 0.0, 0.0)]
    #[case(1.0, -1.0, 1.5, -1.5)] // Top
    #[case(1.0, 1.0, 4.0, 4.0)] // Right
    fn transform_rectangle_scale_with_same_angle(#[case] x: f32, #[case] y: f32, #[case] expected_x: f32, #[case] expected_y: f32) {
        // Arrange
        let from: Rectangle = Rectangle::new(0.0, 0.0, 0.5, 1.0, 45.0);
        let to: Rectangle = Rectangle::new(0.0, 0.0, 2.0, 1.5, 45.0);
        let point: Point = Point::new(x, y);
        let expected: Point = Point::new(expected_x, expected_y);

        // Act
        let actual = transform_point(&point, &from, &to);

        // Assert
        assert_point_equal(&actual, &expected);
    }

    #[rstest]
    #[case(0.0, 0.0, 0.0, 0.0)]
    #[case(0.0, 1.0, -1.0, 0.0)]
    #[case(-1.0, 0.0, 0.0, -1.0)]
    #[case(0.0, -1.0, 1.0, 0.0)]
    #[case(1.0, 0.0, 0.0, 1.0)]
    #[case(f32::sqrt(2.0), f32::sqrt(2.0), -f32::sqrt(2.0), f32::sqrt(2.0))]
    fn transform_square_rotation(#[case] x: f32, #[case] y: f32, #[case] expected_x: f32, #[case] expected_y: f32) {
        // Arrange
        let from: Rectangle = Rectangle::angled(-45.0);
        let to: Rectangle = Rectangle::angled(45.0);
        let point: Point = Point::new(x, y);
        let expected: Point = Point::new(expected_x, expected_y);

        // Act
        let actual = transform_point(&point, &from, &to);

        // Assert
        assert_point_equal(&actual, &expected);
    }

    #[rstest]
    #[case(0.0, 0.0, 0.0, 0.0)]
    #[case(1.0, 0.0, 1.0, 0.0)]
    #[case(1.0, 1.0, 1.0, 1.0)]
    #[case(-1.0, -1.0, -1.0, -1.0)]
    fn transform_square_rotation_same_angle(#[case] x: f32, #[case] y: f32, #[case] expected_x: f32, #[case] expected_y: f32) {
        // Arrange
        let from: Rectangle = Rectangle::angled(12.0);
        let to: Rectangle = Rectangle::angled(12.0);
        let point: Point = Point::new(x, y);
        let expected: Point = Point::new(expected_x, expected_y);

        // Act
        let actual = transform_point(&point, &from, &to);

        // Assert
        assert_point_equal(&actual, &expected);
    }

    #[rstest]
    #[case(0.0, 0.0, 0.0, 0.0)]
    #[case(2.0, 0.0, 0.0, 1.0)]
    #[case(1.0, 0.0, 0.0, 0.5)]
    #[case(-1.0, 0.0, 0.0, -0.5)]
    fn transform_square_scaled_rotation(#[case] x: f32, #[case] y: f32, #[case] expected_x: f32, #[case] expected_y: f32) {
        // Arrange
        let from: Rectangle = Rectangle::scaled(2.0, 2.0);
        let to: Rectangle = Rectangle::angled(90.0);
        let point: Point = Point::new(x, y);
        let expected: Point = Point::new(expected_x, expected_y);

        // Act
        let actual = transform_point(&point, &from, &to);

        // Assert
        assert_point_equal(&actual, &expected);
    }

    #[rstest]
    #[case(0.0, 0.0, 0.0, 0.0)]
    #[case(2.0, 3.0, -1.0, 1.0)]
    #[case(2.0, -3.0, 1.0, 1.0)]
    #[case(4.0, -6.0, 2.0, 2.0)]
    fn transform_rectangle_scaled_rotation(#[case] x: f32, #[case] y: f32, #[case] expected_x: f32, #[case] expected_y: f32) {
        // Arrange
        let from: Rectangle = Rectangle::scaled(2.0, 3.0);
        let to: Rectangle = Rectangle::angled(90.0);
        let point: Point = Point::new(x, y);
        let expected: Point = Point::new(expected_x, expected_y);

        // Act
        let actual = transform_point(&point, &from, &to);

        // Assert
        assert_point_equal(&actual, &expected);
    }

    #[rstest]
    #[case(-1.0, -1.0, 1.0, 1.0)]
    #[case(-1.0 - f32::sqrt(2.0), -1.0 - f32::sqrt(2.0), 1.0 + 2.0 * f32::sqrt(2.0), 1.0 - 2.0 * f32::sqrt(2.0))] // Top
    #[case(-1.0 - f32::sqrt(2.0), -1.0 + f32::sqrt(2.0), 1.0 - 2.0 * f32::sqrt(2.0), 1.0 - 2.0 * f32::sqrt(2.0))] // Left
    fn transform_rectangle_full_same_aspect(#[case] x: f32, #[case] y: f32, #[case] expected_x: f32, #[case] expected_y: f32) {
        // Arrange
        let from: Rectangle = Rectangle::new(-1.0, -1.0, 1.0, 0.5, -45.0);
        let to: Rectangle = Rectangle::new(1.0, 1.0, 2.0, 1.0, 45.0);
        let point: Point = Point::new(x, y);
        let expected: Point = Point::new(expected_x, expected_y);

        // Act
        let actual = transform_point(&point, &from, &to);

        // Assert
        assert_point_equal(&actual, &expected);
    }

    #[rstest]
    #[case(-1.0, -1.0, 1.0, 1.0)]
    #[case(-1.0 - f32::sqrt(2.0), -1.0 - f32::sqrt(2.0), 1.0 + 4.0 * f32::sqrt(2.0), 1.0 - 4.0 * f32::sqrt(2.0))] // Top
    #[case(-1.0 - f32::sqrt(2.0), -1.0 + f32::sqrt(2.0), 1.0 - f32::sqrt(2.0), 1.0 - f32::sqrt(2.0))] // Left
    fn transform_rectangle_full_different_aspect(#[case] x: f32, #[case] y: f32, #[case] expected_x: f32, #[case] expected_y: f32) {
        // Arrange
        let from: Rectangle = Rectangle::new(-1.0, -1.0, 1.0, 0.5, -45.0);
        let to: Rectangle = Rectangle::new(1.0, 1.0, 1.0, 2.0, 45.0);
        let point: Point = Point::new(x, y);
        let expected: Point = Point::new(expected_x, expected_y);

        // Act
        let actual = transform_point(&point, &from, &to);

        // Assert
        assert_point_equal(&actual, &expected);
    }

    #[rstest]
    #[case(0.5, 0.5, 0.5, 0.5)]
    #[case(0.75, 0.25, 0.75, 0.75)]
    #[case(1.0, 0.0, 1.0, 1.0)]
    fn transform_rectangle_first_quadrant_rotate(#[case] x: f32, #[case] y: f32, #[case] expected_x: f32, #[case] expected_y: f32) {
        // Arrange
        let from: Rectangle = Rectangle::new(0.5, 0.5, 1.0, 1.0, 0.0);
        let to: Rectangle = Rectangle::new(0.5, 0.5, 1.0, 1.0, 90.0);
        let point: Point = Point::new(x, y);
        let expected: Point = Point::new(expected_x, expected_y);

        // Act
        let actual = transform_point(&point, &from, &to);

        // Assert
        assert_point_equal(&actual, &expected);
    }

    fn assert_point_equal(actual: &Point, expected: &Point) {
        assert!((actual.x - expected.x).abs() < 1e-6, "Expected: {:?}, was: {:?}", expected, actual);
        assert!((actual.y - expected.y).abs() < 1e-6, "Expected: {:?}, was: {:?}", expected, actual);
    }
}