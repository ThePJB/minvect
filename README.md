# Minvect
A mini vector library, I mean its mostly for me but feel free to use. The point is for these to be the base types to use for associated applications / libraries to not have to always be converting between.

## Todo
```
    pub const fn to_mat3(&self) -> [f32; 9] {
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


also eg rect dilate rect centroid

surely needing dist point to rect
signed distance probably replaces most other things
same thing with segment etc



nb a macro would just need map_each and sum and then all the code would need only the overloads and thats it. i mean the overloads just need map each i guess
i guess proc macro shit was good with map_each. maybe one day