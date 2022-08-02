use std::f64::NAN;

use ::geo::{
    BoundingRect, Centroid, Closest, ClosestPoint, ConvexHull, CoordsIter, FrechetDistance,
    Geometry, Intersects, Point, RotatePoint, Translate,
};
use wkt::{ToWkt, TryFromWkt};

wit_bindgen_rust::export!("geo.wit");

struct Geo;

impl crate::geo::Geo for Geo {
    fn st_convexhull(geo: String) -> String {
        match Geometry::<f64>::try_from_wkt_str(&geo).ok() {
            Some(Geometry::Polygon(poly)) => poly.convex_hull().wkt_string(),
            Some(Geometry::LineString(string)) => string.convex_hull().wkt_string(),
            Some(Geometry::MultiPoint(points)) => points.convex_hull().wkt_string(),
            None => "Invalid geometry".to_owned(),
            _ => "Geometry doesn't support convex hull".to_owned(),
        }
    }

    fn st_centroid(geo: String) -> String {
        match Geometry::<f64>::try_from_wkt_str(&geo).ok() {
            Some(geo) => geo
                .centroid()
                .map(|point| point.wkt_string())
                .unwrap_or("Geometry doesn't support centroid".to_owned()),
            None => "Invalid geometry".to_owned(),
        }
    }

    fn st_pointn(geo: String, index: i64) -> String {
        match Geometry::<f64>::try_from_wkt_str(&geo).ok() {
            Some(Geometry::LineString(string)) => {
                let coords = string
                    .coords()
                    .nth(index.rem_euclid(string.coords_count() as i64) as usize)
                    .unwrap();
                Point::new(coords.x, coords.y).wkt_string()
            }
            None => "Invalid geometry".to_owned(),
            _ => "Geometry doesn't support point n".to_owned(),
        }
    }

    fn st_closestpoint(geo: String, point: String) -> String {
        let point = if let Ok(point) = Point::try_from_wkt_str(&point) {
            point
        } else {
            return "Invalid point".to_owned();
        };

        match Geometry::<f64>::try_from_wkt_str(&geo)
            .ok()
            .map(|geo| geo.closest_point(&point))
        {
            Some(Closest::SinglePoint(point) | Closest::Intersection(point)) => point.wkt_string(),
            Some(Closest::Indeterminate) => "Indeterminate".to_owned(),
            None => "Invalid geometry".to_owned(),
        }
    }

    fn st_envelope(geo: String) -> String {
        match Geometry::<f64>::try_from_wkt_str(&geo).ok() {
            Some(geo) => geo
                .bounding_rect()
                .map(|rect| rect.wkt_string())
                .unwrap_or("Geometry doesn't support envelope".to_owned()),
            None => "Invalid geometry".to_owned(),
        }
    }

    fn st_intersects(geo1: String, geo2: String) -> bool {
        match (
            Geometry::<f64>::try_from_wkt_str(&geo1),
            Geometry::<f64>::try_from_wkt_str(&geo2),
        ) {
            (Ok(geo1), Ok(geo2)) => geo1.intersects(&geo2),
            _ => false,
        }
    }

    fn st_translate(geo: String, dx: f64, dy: f64) -> String {
        match Geometry::<f64>::try_from_wkt_str(&geo).ok() {
            Some(geo) => geo.translate(dx, dy).wkt_string(),
            None => "Invalid geometry".to_owned(),
        }
    }

    fn st_rotate(geo: String, rads: f64, point: String) -> String {
        let point = if let Ok(point) = Point::try_from_wkt_str(&point) {
            point
        } else {
            return "Invalid point".to_owned();
        };

        match Geometry::<f64>::try_from_wkt_str(&geo).ok() {
            Some(geo) => geo
                .rotate_around_point(rads.to_degrees(), point)
                .wkt_string(),
            None => "Invalid geometry".to_owned(),
        }
    }

    fn st_frechetdistance(geo1: String, geo2: String) -> f64 {
        match (
            Geometry::<f64>::try_from_wkt_str(&geo1),
            Geometry::<f64>::try_from_wkt_str(&geo2),
        ) {
            (Ok(Geometry::LineString(string1)), Ok(Geometry::LineString(string2))) => {
                string1.frechet_distance(&string2)
            }
            _ => NAN,
        }
    }
}
