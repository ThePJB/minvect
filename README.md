# Minvect
A mini vector library, I mean its mostly for me but feel free to use. The point is for these to be the base types to use for associated applications / libraries to not have to always be converting between.

## Todo
```
    pub fn to_mat3(&self) -> [f32; 9] {
        [
            self.wh.x, 0.0, -self.xy.x,
            0.0, self.wh.y, -self.xy.y,
            0.0, 0.0, 1.0
        ]
    }
```
its more ideal to get the transform of a rect because eg could have some vertex data to transform via a uniform
could also transform it cpu side too
but yea there is rect mat3 equivalence and i want to capture it