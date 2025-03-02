#[derive(Clone)]
pub struct RgbPixel {
    pub red: f32,
    pub green: f32,
    pub blue: f32
}

#[derive(Clone)]
pub struct RgbImage {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<RgbPixel>
}

#[derive(Clone, Debug)]
pub struct RawImage {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabPixel {
    pub lightness: f32, // L*
    pub tint: f32, // a*
    pub temperature: f32 // b*
}

impl RawImage {
    fn rgb_pixel_at(&self, x: usize, y: usize) -> Option<RgbPixel> {

        if x < self.width && y < self.height {
            let pixel_index: usize = (y * self.width + x) * 4; // Times 4 due to unused alpha channel
            let red: f32 = self.pixels[pixel_index + 0] as f32 / 255.0;
            let green: f32 = self.pixels[pixel_index + 1] as f32 / 255.0;
            let blue: f32 = self.pixels[pixel_index + 2] as f32 / 255.0;
            Some(RgbPixel { red, green, blue })
        } else {
            None
        }
    }

    pub fn lab_pixel_at(&self, x: usize, y: usize) -> Option<LabPixel> {
        self.rgb_pixel_at(x, y).map(rgb_pixel_to_lab)
    }
}

/**
 * Conversion based on: https://bottosson.github.io/posts/oklab/
 */
pub fn rgb_pixel_to_lab(rgb_pixel: RgbPixel) -> LabPixel {
    let mut l: f32 = 0.4122214708 * rgb_pixel.red + 0.5363325363 * rgb_pixel.green + 0.0514459929 * rgb_pixel.blue;
    let mut m: f32 = 0.2119034982 * rgb_pixel.red + 0.6806995451 * rgb_pixel.green + 0.1073969566 * rgb_pixel.blue;
    let mut s: f32 = 0.0883024619 * rgb_pixel.red + 0.2817188376 * rgb_pixel.green + 0.6299787005 * rgb_pixel.blue;
    
    l = l.powf(1.0 / 3.0);
    m = m.powf(1.0 / 3.0);
    s = s.powf(1.0 / 3.0);
    
    LabPixel {
        lightness:   0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
        tint:        1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
        temperature: 0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)] // Black
    #[case(1.0, 1.0, 1.0, 1.0, 0.0, 0.0)] // White
    #[case(0.5, 0.5, 0.5, 0.7937005, 0.0, 0.0)] // Gray
    #[case(1.0, 0.0, 0.0, 0.6279554, 0.22, 0.13)] // Red
    #[case(0.0, 1.0, 0.0, 0.8664396, -0.23, 0.18)] // Green
    #[case(0.0, 0.0, 1.0, 0.4520137, -0.03, -0.31)] // Blue
    fn test_rgb_pixel_to_lab(
            #[case] red: f32,
            #[case] green: f32,
            #[case] blue: f32,
            #[case] expected_l: f32,
            #[case] expected_a: f32,
            #[case] expected_b: f32) {
        // Arrange
        let rgb: RgbPixel = RgbPixel { red, green, blue };
        let expected: LabPixel = LabPixel {
            lightness: expected_l,
            tint: expected_a,
            temperature: expected_b,
        };

        // Act
        let actual = rgb_pixel_to_lab(rgb);

        // Assert
        assert_color_equal(&actual, &expected);
    }

    fn assert_color_equal(actual: &LabPixel, expected: &LabPixel) {
        assert!((actual.lightness - expected.lightness).abs() < 1e-6, "Expected: {:?}, was: {:?}", expected, actual);
        assert!((actual.tint - expected.tint).abs() < 0.1, "Expected: {:?}, was: {:?}", expected, actual);
        assert!((actual.temperature - expected.temperature).abs() < 0.1, "Expected: {:?}, was: {:?}", expected, actual);
    }
}