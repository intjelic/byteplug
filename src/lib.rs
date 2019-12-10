pub mod geometry;
pub mod renderer;
pub mod animation;

pub mod audio;
pub mod video;

pub mod input;
pub mod application;

pub mod ui;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
