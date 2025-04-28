// Documentation in Rust
// This example demonstrates different documentation patterns and best practices

/// A simple struct representing a point in 2D space.
///
/// # Examples
///
/// ```
/// let point = Point::new(3, 4);
/// assert_eq!(point.x, 3);
/// assert_eq!(point.y, 4);
/// ```
///
/// # Panics
///
/// The `new` function will panic if either coordinate is negative.
///
/// # Safety
///
/// This struct is safe to use as long as the coordinates are valid numbers.
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    /// The x-coordinate of the point
    pub x: i32,
    /// The y-coordinate of the point
    pub y: i32,
}

impl Point {
    /// Creates a new `Point` with the given coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate
    /// * `y` - The y-coordinate
    ///
    /// # Returns
    ///
    /// A new `Point` instance
    ///
    /// # Panics
    ///
    /// Panics if either coordinate is negative
    ///
    /// # Examples
    ///
    /// ```
    /// let point = Point::new(3, 4);
    /// ```
    pub fn new(x: i32, y: i32) -> Point {
        assert!(x >= 0, "x coordinate must be non-negative");
        assert!(y >= 0, "y coordinate must be non-negative");
        Point { x, y }
    }

    /// Calculates the distance from the origin (0,0).
    ///
    /// # Returns
    ///
    /// The distance as a floating-point number
    ///
    /// # Examples
    ///
    /// ```
    /// let point = Point::new(3, 4);
    /// assert_eq!(point.distance_from_origin(), 5.0);
    /// ```
    pub fn distance_from_origin(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

/// A trait for shapes that can be drawn.
///
/// # Examples
///
/// ```
/// struct Circle;
/// impl Drawable for Circle {
///     fn draw(&self) {
///         println!("Drawing a circle");
///     }
/// }
/// ```
pub trait Drawable {
    /// Draws the shape.
    ///
    /// # Examples
    ///
    /// ```
    /// let circle = Circle;
    /// circle.draw();
    /// ```
    fn draw(&self);
}

/// A module containing utility functions.
///
/// # Examples
///
/// ```
/// use my_crate::utils;
/// let result = utils::add(2, 2);
/// assert_eq!(result, 4);
/// ```
pub mod utils {
    /// Adds two numbers together.
    ///
    /// # Arguments
    ///
    /// * `a` - First number
    /// * `b` - Second number
    ///
    /// # Returns
    ///
    /// The sum of the two numbers
    ///
    /// # Examples
    ///
    /// ```
    /// let result = utils::add(2, 2);
    /// assert_eq!(result, 4);
    /// ```
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

/// A type alias for a vector of points.
///
/// # Examples
///
/// ```
/// let points: PointList = vec![Point::new(1, 2), Point::new(3, 4)];
/// ```
pub type PointList = Vec<Point>;

/// A constant representing the maximum number of points.
///
/// # Examples
///
/// ```
/// assert!(MAX_POINTS > 0);
/// ```
pub const MAX_POINTS: usize = 1000;

/// A function that processes a list of points.
///
/// # Arguments
///
/// * `points` - A vector of points to process
///
/// # Returns
///
/// The number of points processed
///
/// # Errors
///
/// Returns an error if the number of points exceeds `MAX_POINTS`
///
/// # Examples
///
/// ```
/// let points = vec![Point::new(1, 2), Point::new(3, 4)];
/// let result = process_points(&points).unwrap();
/// assert_eq!(result, 2);
/// ```
pub fn process_points(points: &[Point]) -> Result<usize, String> {
    if points.len() > MAX_POINTS {
        return Err(format!("Too many points: {}", points.len()));
    }
    Ok(points.len())
}

// Private module with internal documentation
mod internal {
    //! This module contains internal implementation details.
    //! It is not part of the public API.

    /// A private struct used internally.
    struct InternalStruct {
        value: i32,
    }

    impl InternalStruct {
        /// Creates a new instance.
        fn new(value: i32) -> Self {
            InternalStruct { value }
        }
    }
} 