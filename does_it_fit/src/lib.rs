


pub fn area_fit(
	x: usize,
	y: usize,
	objects: areas_volumes::GeometricalShapes,
	times: usize,
	a: usize,
	b: usize,
) -> bool {

    let mut area_rect = x * y;
    if objects == areas_volumes::GeometricalShapes::Square {
        if areas_volumes::square_area(a) * times <= area_rect {
            return true;
        } else {
            return false;
        }
    } else if objects == areas_volumes::GeometricalShapes::Circle {
        if areas_volumes::circle_area(a) * times <= area_rect {
            return true;
        } else {
            return false;
        }
    } else if objects == areas_volumes::GeometricalShapes::Rectangle {
        if areas_volumes::rectangle_area(a,b) * times <= area_rect {
            return true;
        } else {
            return false;
        }
    } else if objects == areas_volumes::GeometricalShapes::Triangle {
        if areas_volumes::triangle_area(a, b) * times <= area_rect {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }

}
pub fn volume_fit(
	x: usize,
	y: usize,
	z: usize,
	objects: areas_volumes::GeometricalVolumes,
	times: usize,
	a: usize,
	b: usize,
	c: usize,
) -> bool {
    let mut volume_rect = x * y * z;
    if objects == areas_volumes::GeometricalVolumes::Cube {
        if areas_volumes::cube_volume(a) * times <= volume_rect {
            return true;
        } else {
            return false;
        }
    } else if objects == areas_volumes::GeometricalVolumes::Sphere {
        if areas_volumes::sphere_volume(a) * times <= volume_rect {
            return true;
        } else {
            return false;
        }
    } else if objects == areas_volumes::GeometricalVolumes::Cone {
        if areas_volumes::cone_volume(a, b) * times <= volume_rect {
            return true;
        } else {
            return false;
        }
    } else if objects == areas_volumes::GeometricalVolumes::Pyramid {
        if areas_volumes::triangular_pyramid_volume(a as f64, b) * times <= volume_rect {
            return true;
        } else {
            return false;
        }
    } else if objects == areas_volumes::GeometricalVolumes::Parallelepiped {
        if areas_volumes::parallelepiped_volume(a, b, c) * times <= volume_rect {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

