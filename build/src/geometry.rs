use geo::{Coord, LineString, Polygon};
use geojson::Geometry;

pub fn parse_geometry(geometry: &Geometry) -> Vec<Polygon<f32>> {
    match &geometry.value {
        geojson::Value::Polygon(polygon_coords) => {
            if let Some(polygon) = parse_polygon(polygon_coords) {
                vec![polygon]
            } else {
                vec![]
            }
        }
        geojson::Value::MultiPolygon(multi_polygon_coords) => multi_polygon_coords
            .iter()
            .filter_map(|polygon_coords| parse_polygon(polygon_coords))
            .collect(),
        _ => {
            unimplemented!("Unsupported geometry type");
        }
    }
}

fn parse_polygon(polygon_coords: &Vec<Vec<Vec<f64>>>) -> Option<Polygon<f32>> {
    let Some(exterior_ring_coords) = polygon_coords.get(0) else {
        return None;
    };

    let exterior_points: Vec<Coord<f32>> = exterior_ring_coords
        .iter()
        .filter_map(|coord| {
            if coord.len() >= 2 {
                Some(Coord {
                    x: coord[0] as f32,
                    y: coord[1] as f32,
                })
            } else {
                None
            }
        })
        .collect();

    if exterior_points.is_empty() {
        return None;
    }

    let exterior = LineString(exterior_points);

    let interiors: Vec<LineString<f32>> = polygon_coords
        .iter()
        .skip(1)
        .filter_map(|interior_ring_coords| {
            let interior_points: Vec<Coord<f32>> = interior_ring_coords
                .iter()
                .filter_map(|coord| {
                    if coord.len() >= 2 {
                        Some(Coord {
                            x: coord[0] as f32,
                            y: coord[1] as f32,
                        })
                    } else {
                        None
                    }
                })
                .collect();

            if interior_points.is_empty() {
                None
            } else {
                Some(LineString(interior_points))
            }
        })
        .collect();

    Some(Polygon::new(exterior, interiors))
}
