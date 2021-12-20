


#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}

impl Point {
    // euclidean distance is the square root of this function
    // but that puts us in floating point, and isn't required for what we're looking for
    fn distance_squared(&self, other: &Point) -> i32 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

//(x,y,z)
//(x,z,y)
//(y,x,z)
//(y,z,x)
//(z,x,y)
//(z,y,x)

//no sign
//-x
//-y
//-z
//-x,-y