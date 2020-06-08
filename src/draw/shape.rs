// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.

// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, May 2020

use crate::geometry::compute_bounds;
use crate::geometry::{Position, Box};
use crate::image::Color;
use crate::draw::{Primitive, Usage};
use crate::draw::Surface;
use crate::draw::{Vertex, VertexArray};

fn compute_normal(p1: &Position<f32>, p2: &Position<f32>) -> Position<f32> {
    // Compute the dot product of two vectors.
    let mut normal = Position::new(p1.y - p2.y, p2.x - p1.x);
    let length = (normal.x * normal.x + normal.y * normal.y).sqrt();
    if length != 0.0 {
        normal /= length;
    }

    normal
}

fn dot_product(p1: &Position<f32>, p2: &Position<f32>) -> f32 {
    // Compute the dot product of two vectors.
    p1.x * p2.x + p1.y * p2.y
}

/// A drawable custom shape.
///
/// A shape is one of the high-level drawable entity (built on top of vertex array) that allows you
/// to draw a custom shape on a surface. It's defined by a set of points and it can have a fill
/// color, an outline with a given color and thickness, and additionally, it can also have a
/// texture (not implemented yet).
///
/// The fill color, the outline and the texture are all optional as the color can be transparent,
/// the outline thickness set to 0, and no texture has to be set. There must be at least 3 points,
/// or it's called an empty shape and nothing is drawn.
///
/// Use `new()` to construct an empty shape and `with_points()` to construct a custom shape from a
/// list of points.
///
/// ```
/// let empty_shape = Shape::new();
///
/// let points = vec![
///     Position::new(-0.5, -0.5),
///     Position::new( 0.0,  0.5),
///     Position::new( 0.5, -0.5)
/// ];
/// let mut triangle = Shape::with_points(&points);
/// ```
///
/// To update the points of the shape, use `set_point_number()` and `set_point()`, or
/// `update_points()`. To query the points of the shape, use `point_number()` and `point()`, or
/// `points()`.
///
/// ```
/// let mut triangle = Shape::new();
/// triangle.set_point_number(3);
/// triangle.set_point(0, Position::new(-0.5, -0.5));
/// triangle.set_point(1, Position::new(0.0, 0.5));
/// triangle.set_point(2, Position::new(0.5, -0.5));
///
/// // ... or ...
///
/// let points = vec![
///     Position::new(-0.5, -0.5),
///     Position::new( 0.0,  0.5),
///     Position::new( 0.5, -0.5)
/// ];
/// triangle.update_points(points);
/// ```
///
/// To change the fill color, use `set_color()`. To change the outline, use `set_outline_color()`
/// and `set_outline_thickness()`. To change the texture, use `set_texture()`. Each of these
/// properties also have getters which are `color()`, `outline_color()`, `outline_thickness()` and
/// `texture()`.
///
/// ```
/// // To be written.
/// ```
///
/// Documentation about the `update()` method here.
///
/// ```
/// // To be written.
/// ```
///
/// Documentation about the `draw()` method here.
///
/// ```
/// // To be written.
/// ```
///
/// **Implementation notes**
///
/// - I didn't spend much time on designing this interface; it's technically a container of points,
///   should it implement the Index and IndexMut traits ? Should it provide a `as_slice()` and
///   `as_mut_slice()` methods ? For now, `point_number()`, `set_point_number()`, `point()` and
///   `set_point()` can be used to update the points individually. People can also use
///   `update_points()`. The overall interface may also evolve to be more consistent with the
///   overall framework interface.
/// - After spending some time thinking about the interface: It can be seen as an array, yes, but
///   it's not conceptually an array, therefore, it's wrong to make an array-like interface. Another
///   thing that make an array-like interface not likable is because we would not know when the user
///   is actually modifying the points, and thus, when to update the vertices.
/// - The set_point() method could have been point_mut() method but there's a need to mark the
///   vertices to be updated.
/// - I couldn't determine what was the best way to implement point() and set_point() with regards
///   to the index that can be out of range. In the meantime, I chose to return Position::default()
///   if the index is incorrect.
/// - It keeps an extra array of points, which could be avoided by using the vertex array directly
///   (to store the points data). However, it makes the implementation simpler (and the points() and
///   update_points() methods faster) and in practice, there will never contains huge amount of
///   points to the point it becomes an issue.
/// - An `update()` method is available to synchronize the system memory with the graphics memory
///   but it's not mandatory as it's done before the shape is used for drawing, if it's not done
///   yet.
/// - Implementation will not change when the vertex array implementation changes to use mapped
///   memory technique (this note should be removed after the vertex array implementation is
///   updated).
/// - The implementation supports convex shapes only. It will be updated later to support concave
///   shapes.
/// - The implementation doesn't support texture yet. It will be implemented later.
/// - The set_usage() could be exposed, but is that really needed in practice ?
/// - If the shape has less than 3 points, nothing is drawn. No error message is displayed too.
///   Should that change ?
///
pub struct Shape {
    points: Vec<Position<f32>>,
    color: Color,
    outline_color: Color,
    outline_thickness: f32,
    vertices: VertexArray,
    outline_vertices: VertexArray,
    center: Position<f32>,
    inside_bounds: Box<f32>, // not taking the outline into account
    bounds: Box<f32>,
    update: bool, // indicate if vertices need to be re-computed
    update_outline: bool // indicate if the vertices of the outline need to be re-computed
}

impl Shape {

    /// Constructs an empty shape.
    ///
    /// This function is the default constructor. It creates a shape with no points (aka. an empty
    /// shape); nothing is drawn until the shape is updated with at least 3 points.
    ///
    pub fn new() -> Shape {
        let mut vertices = VertexArray::new();
        vertices.set_primitive(Primitive::TriangleFans);
        vertices.set_usage(Usage::Stream);

        let mut outline_vertices = VertexArray::new();
        outline_vertices.set_primitive(Primitive::TriangleStrips);
        outline_vertices.set_usage(Usage::Stream);

        Shape {
            points: Vec::new(),
            color: Color::BLACK,
            outline_color: Color::BLACK,
            outline_thickness: 0.0,
            vertices: vertices,
            outline_vertices: outline_vertices,
            center: Position::default(),
            inside_bounds: Box::default(),
            bounds: Box::default(),
            update: true,
            update_outline: true
        }
    }

    /// Constructs a shape from a set of points.
    ///
    /// This function constructs a custom shape from a list of points. Note that there must be at
    /// least 3 points.
    ///
    pub fn with_points(points: &Vec<Position<f32>>) -> Shape {
        let mut shape = Shape::new();
        shape.update_points(points);

        shape
    }

    /// Returns the number of points.
    ///
    /// This function returns the number of points that makes the shape.
    ///
    pub fn point_number(&self) -> usize {
        self.points.len()
    }

    /// Changes the number of points.
    ///
    /// This function changes the number of points that makes the shape.
    ///
    pub fn set_point_number(&mut self, number: usize) {
        self.points.resize(number, Position::default());

        // Changing the number of points requires updating the shape vertices and the outline
        // vertices.
        self.update = true;
        self.update_outline = true;
    }

    /// Returns the point position of a giv
    ///
    /// Long description.
    ///
    pub fn point(&self, index: usize) -> Position<f32> {
        *self.points.get(index).unwrap_or(&Position::default())
    }

    /// Brief description.
    ///
    /// Long description.
    ///
    pub fn set_point(&mut self, index: usize, position: Position<f32>) {
        *self.points.get_mut(index).unwrap_or(&mut Position::default()) = position;

        // Changing the points requires updating the shape vertices and the outline vertices.
        self.update = true;
        self.update_outline = true;
    }

    /// Brief description.
    ///
    /// Long description.
    ///
    pub fn points(&self) -> &Vec<Position<f32>> {
        &self.points
    }

    /// Brief description.
    ///
    /// Long description.
    ///
    pub fn update_points(&mut self, points: &Vec<Position<f32>>) {
        self.points = points.clone();

        // Changing the points requires updating the shape vertices and the outline vertices.
        self.update = true;
        self.update_outline = true;
    }

    /// Returns the color of the shape.
    ///
    /// This function returns the **fill color** of the shape.
    ///
    pub fn color(&self) -> Color {
        self.color
    }

    /// Changes the color of the shape.
    ///
    /// This function changes the **fill color** of the shape.
    ///
    /// Note that the vertices on the graphics memory must be updated which will happen right before
    /// the shape is used for drawing. You may call the `update()` function for this to take effect
    /// immediately.
    ///
    pub fn set_color(&mut self, color: Color) {
        self.color = color;

        // Changing the color requires updating the shape vertices (the outline vertices are
        // untouched).
        self.update = true;
    }

    /// Returns the outline color of the shape.
    ///
    /// This function returns the **outline color** of the shape.
    ///
    pub fn outline_color(&self) -> Color {
        self.outline_color
    }

    /// Changes the outline color of the shape.
    ///
    /// This function changes the **online color** of the shape.
    ///
    /// Note that the vertices on the graphics memory must be updated which will happen right before
    /// the shape is used for drawing. You may call the `update()` function for this to take effect
    /// immediately.
    ///
    pub fn set_outline_color(&mut self, outline_color: Color) {
        self.outline_color = outline_color;

        // Changing the outline color requires updating the outline vertices (the shape vertices are
        // untouched).
        self.update_outline = true;
    }

    /// Returns the outline thickness of the shape.
    ///
    /// This function returns the **outline thickness** of the shape.
    ///
    pub fn outline_thickness(&self) -> f32 {
        self.outline_thickness
    }

    /// Changes the outline thickness of the shape.
    ///
    /// This function changes the **outline thickness** of the shape.
    ///
    /// Note that the vertices on the graphics memory must be updated which will happen right before
    /// the shape is used for drawing. You may call the `update()` function for this to take effect
    /// immediately.
    ///
    pub fn set_outline_thickness(&mut self, outline_thickness: f32) {
        self.outline_thickness = outline_thickness;

        // Changing the outline color requires updating the outline vertices (the shape vertices are
        // untouched).
        self.update_outline = true;
    }

    /// Brief description.
    ///
    /// Long description.
    ///
    pub fn update(&mut self) {
        // Update the shape vertices if they're marked for update.
        if self.update {
            self.update_shape_vertices();

            // Mark the shape vertices as updated.
            self.update = false;
        }

        // Update the outline vertices if they're marked for update.
        if self.update_outline {
            self.update_outline_vertices();

            // Mark the outline vertices as updated.
            self.update_outline = false;
        }
    }

    fn update_shape_vertices(&mut self) {
        // A shape must have a least 3 points, if it doesn't, empty the vertices.
        if self.points.len() < 3 {
            self.vertices.update_vertices(&vec![]);
            return
        }

        // Compute and update the bounding box and the center.
        self.inside_bounds = compute_bounds(&self.points);

        self.center.x = self.inside_bounds.position.x + self.inside_bounds.size.width  / 2.0;
        self.center.y = self.inside_bounds.position.y + self.inside_bounds.size.height / 2.0;

        // Create the new array of vertices, two slots bigger than the given points (one for the
        // center, and one for the repeated first point). See triangle fans drawing primitive doc.
        let size = self.points.len() + 2;

        let mut vertices = Vec::<Vertex>::with_capacity(size);
        vertices.resize(size, Vertex::new());

        // Set up the center vertex (after computing the center position).
        let mut center_vertex = vertices.first_mut().unwrap();
        center_vertex.x = self.center.x;
        center_vertex.y = self.center.y;

        // Fill the array of vertices with the points data.
        for (index, point) in self.points.iter().enumerate() {
            let mut vertice = vertices.get_mut(index + 1).unwrap();
            vertice.x = point.x;
            vertice.y = point.y;
        }

        // Make the very last vertex equal as the first vertex (not the center).
        let mut last_vertex = vertices.last_mut().unwrap();
        last_vertex.x = self.points[0].x;
        last_vertex.y = self.points[0].y;

        // Set up the color property of the vertices.
        for vertex in vertices.iter_mut() {
            vertex.r = self.color.red   as f32 / 255.0;
            vertex.g = self.color.green as f32 / 255.0;
            vertex.b = self.color.blue  as f32 / 255.0;
            vertex.a = self.color.alpha as f32 / 255.0;
        }

        // Set up the texture property of the vertices.
        // ... to be done after texture is implemented.

        // Update the shape vertices with the newly computed vertices.
        self.vertices.update_vertices(&vertices);
    }

    fn update_outline_vertices(&mut self) {
        // The algorithm requires that the shape vertices (with inside bounds and center) are
        // already computed.
        assert_eq!(self.update, false);

        // A shape must have a least 3 points, if it doesn't, it has no outline. Same if the
        // the outline thickness is zero.
        if self.points.len() < 3 || self.outline_thickness == 0.0 {
            self.outline_vertices.update_vertices(&vec![]);
            return
        }

        let size = (self.points.len() + 1) * 2;

        let mut vertices = Vec::<Vertex>::with_capacity(size);
        vertices.resize(size, Vertex::new());

        for (index, point) in self.points.iter().enumerate() {

            // Get the two segments shared by the current point.
            let left_point = self.points.get(index.overflowing_sub(1).0).unwrap_or_else(|| self.points.last().unwrap());
            let right_point = self.points.get(index + 1).unwrap_or_else(|| self.points.first().unwrap());

            // Compute their normal.
            let mut n1 = compute_normal(left_point, point);
            let mut n2 = compute_normal(point, right_point);

            // Make sure that the normals point towards the outside of the shape (this depends on
            // the order in which the points were defined).
            let difference = self.center - *point;
            if dot_product(&n1, &difference) > 0.0 {
                n1 = -n1;
            }
            if dot_product(&n2, &difference) > 0.0 {
                n2 = -n2;
            }

            // Combine them to get the extrusion direction.
            let factor = 1.0 + (n1.x * n2.x + n1.y * n2.y);
            let normal = (n1 + n2) / factor;

            // Compute the position of the outline vertices.
            vertices[index * 2 + 0].x = point.x;
            vertices[index * 2 + 0].y = point.y;

            let extruded_position = *point + normal * self.outline_thickness;
            vertices[index * 2 + 1].x = extruded_position.x;
            vertices[index * 2 + 1].y = extruded_position.y;
        }

        // Duplicate the first point at the end, to close the outline (there may be a way to write
        // something like `vertices[-2..0] = vertices[0..2];`.
        let len = vertices.len();
        vertices[len - 2] = vertices[0];
        vertices[len - 1] = vertices[1];

        // Set up the color property of the vertices.
        for vertex in vertices.iter_mut() {
            vertex.r = self.outline_color.red   as f32 / 255.0;
            vertex.g = self.outline_color.green as f32 / 255.0;
            vertex.b = self.outline_color.blue  as f32 / 255.0;
            vertex.a = self.outline_color.alpha as f32 / 255.0;
        }

        // Update the outline vertices with the newly computed vertices.
        self.outline_vertices.update_vertices(&vertices);

        // Compute and update the outside bounds of the shape.
        self.bounds = self.outline_vertices.bounds();
    }

    /// Brief description.
    ///
    /// Long description.
    ///
    pub fn draw(&mut self, surface: &mut Surface) {
        // Make sure the graphics memory is synchronized with the vertices on system memory.
        self.update();

        // Draw the shape vertices first, then the outline vertices.
        surface.draw_vertices(&self.vertices);
        surface.draw_vertices(&self.outline_vertices);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape() {
        // To be implemented.
    }
}
