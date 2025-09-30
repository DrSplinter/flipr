//! Example demonstrating basic image processing with flipr.

use flipr_core::{Gray, ImageProcessor};
use flipr_transform::{AffineTransform, TransformExt};

/// A simple image processor that generates a gradient.
struct GradientProcessor {
    width: usize,
    height: usize,
}

impl ImageProcessor for GradientProcessor {
    type Pixel = Gray<u8>;
    type Error = ();

    fn process_pixel(&self, x: usize, y: usize) -> Result<Option<Self::Pixel>, Self::Error> {
        if x < self.width && y < self.height {
            let value = ((x + y) * 255 / (self.width + self.height)) as u8;
            Ok(Some(Gray { value }))
        } else {
            Ok(None)
        }
    }

    fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

fn main() {
    println!("Flipr Image Processing Example");
    println!("===============================\n");

    // Create a basic gradient processor
    let processor = GradientProcessor {
        width: 100,
        height: 100,
    };

    println!("1. Basic Processing:");
    let pixel = processor.process_pixel(50, 50).unwrap();
    println!("   Pixel at (50, 50): {:?}", pixel);

    // Apply transformations using the functional API
    println!("\n2. Functional Pipeline:");
    let pipeline = processor
        .map(|p: Gray<u8>| Gray {
            value: p.value.saturating_mul(2),
        })
        .filter(|p: &Gray<u8>| p.value < 200);

    let pixel = pipeline.process_pixel(50, 50).unwrap();
    println!("   After map and filter at (50, 50): {:?}", pixel);

    // Apply affine transformations
    println!("\n3. Affine Transformations:");
    let processor2 = GradientProcessor {
        width: 100,
        height: 100,
    };

    // Create a complex transformation
    let transform =
        AffineTransform::translation(10.0, 10.0).then(&AffineTransform::scale(0.5, 0.5));

    println!("   Transform matrix:");
    println!(
        "   | {:6.2} {:6.2} {:6.2} |",
        transform.a, transform.b, transform.tx
    );
    println!(
        "   | {:6.2} {:6.2} {:6.2} |",
        transform.c, transform.d, transform.ty
    );
    println!("   | {:6.2} {:6.2} {:6.2} |", 0.0, 0.0, 1.0);

    let transformed = processor2.transform(transform);
    let pixel = transformed.process_pixel(50, 50).unwrap();
    println!("   Transformed pixel at (50, 50): {:?}", pixel);

    // Sample various points to show the gradient
    println!("\n4. Gradient Samples:");
    let sample_processor = GradientProcessor {
        width: 100,
        height: 100,
    };

    for y in (0..100).step_by(25) {
        print!("   ");
        for x in (0..100).step_by(25) {
            if let Ok(Some(pixel)) = sample_processor.process_pixel(x, y) {
                print!("{:3} ", pixel.value);
            }
        }
        println!();
    }

    println!("\n5. Chained Processors:");
    let proc1 = GradientProcessor {
        width: 50,
        height: 50,
    };
    let proc2 = GradientProcessor {
        width: 50,
        height: 50,
    };

    let chained = proc1.chain(proc2);
    println!("   Chained dimensions: {:?}", chained.dimensions());
    let pixel_left = chained.process_pixel(25, 25).unwrap();
    let pixel_right = chained.process_pixel(75, 25).unwrap();
    println!("   Left processor pixel (25, 25): {:?}", pixel_left);
    println!("   Right processor pixel (75, 25): {:?}", pixel_right);

    println!("\nExample completed successfully!");
}
