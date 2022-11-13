
fn main() {

}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
struct Restriction {
    type_1: i32,
    type_2: i32,
    type_3: i32,
    type_4: i32,
}

impl Restriction {
    fn new(type_1: i32, type_2: i32, type_3: i32, type_4: i32) -> Self {
        Self { type_1, type_2, type_3, type_4 }
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
struct Zone {
    x: i32,
    y: i32,
    restriction: Restriction,
}

impl Zone {
    fn new(x: i32, y: i32, restriction: Restriction) -> Self {
        Self { x, y, restriction }
    }
    
    fn get_furthest_away(&self, zones: &Vec<Zone>) -> Zone {
        let default_zone = Zone::default();
        let mut result: &Zone = &default_zone;
        let mut max_dist: f64 = f64::MIN;
        let mut cur_dist: f64;
        for zone in zones {
            cur_dist = self.get_euclidian_distance(&zone);
            if cur_dist > max_dist {
                max_dist = cur_dist;
                result = zone;
            }
        }
        
        *result
    }

    fn get_euclidian_distance(&self, other: &Zone) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }

    fn climbing_wall_o1(&self, zones: &Vec<Zone>) -> Vec<Zone> {
        let mut result: Vec<Zone> = Vec::with_capacity(zones.len());

        for zone in zones {
            if self.is_on_vertical_plane(zone) || self.is_on_horizontal_plane(zone) {
                result.push(*zone);
            }
        }

        result
    }

    fn is_on_vertical_plane(&self, other: &Zone) -> bool {
        self.y == other.y
    }

    fn is_on_horizontal_plane(&self, other: &Zone) -> bool {
        self.x == other.x
    }

}

// maybe optimize with return &[Zone] and using lifetimes
fn construct_zones_for_test(size: usize, lines: &[&str]) -> Vec<Zone> {
    let input_size: usize = 6; // x,y + 4 restrictions
    let mut zones: Vec<Zone> = Vec::with_capacity(size);
    let mut split_line;
    let mut restriction: Restriction;
    let mut cur_zone = Zone::default();
    for line in lines {
        split_line = line.split(" ").into_iter();
        cur_zone.x = split_line.next().expect("x not found").parse().unwrap();
        cur_zone.y = split_line.next().expect("y not found").parse().unwrap();
        restriction = Restriction::new(
            split_line.next().unwrap().parse().unwrap(),
            split_line.next().unwrap().parse().unwrap(),
            split_line.next().unwrap().parse().unwrap(),
            split_line.next().unwrap().parse().unwrap()
        );
        cur_zone.restriction = restriction;
        zones.push(cur_zone);
    }

    zones
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euclidian_dist_horizontal() {
        let zone1 = Zone::new(0,0, Restriction::default());
        let zone2 = Zone::new(0,5, Restriction::default());
        assert_eq!(zone1.get_euclidian_distance(&zone2), 5f64);
    }

    #[test]
    fn euclidian_dist_vertical() {
        let zone1 = Zone::new(3,0, Restriction::default());
        let zone2 = Zone::new(0,0, Restriction::default());
        assert_eq!(zone1.get_euclidian_distance(&zone2), 3f64);
    }

    #[test]
    fn euclidian_dist_diagonal() {
        let zone1 = Zone::new(0,0, Restriction::default());
        let zone2 = Zone::new(2,4, Restriction::default());
        let dist_round_2_decimal = (zone1.get_euclidian_distance(&zone2) * 100f64).round() / 100f64;
        assert_eq!(dist_round_2_decimal, 4.47);
    }

    #[test]
    fn furthest_away_4_zones() {
        let cur_zone = Zone::new(2,8, Restriction::default());
        let zone_furthest_away = Zone::new(7,3, Restriction::default());
        let zones = vec![
            Zone::new(4, 7, Restriction::default()),
            Zone::new(3, 4, Restriction::default()),
            zone_furthest_away,
        ];
        assert_eq!(cur_zone.get_furthest_away(&zones), zone_furthest_away);
    }

    #[test]
    fn construct_zones() {
        let size: usize = 8;
        let input = [
            "0 1 4 0 2 0",
            "2 2 1 2 3 4",
            "4 1 3 1 1 0",
            "4 3 1 0 0 1",
            "6 4 0 0 1 1",
            "9 1 1 0 0 2",
            "9 4 2 0 0 1",
            "9 3 0 0 0 0",
        ];
        let actual = construct_zones_for_test(size, &input);

        let expected = vec![
            Zone::new(0, 1, Restriction::new(4, 0, 2, 0)),
            Zone::new(2, 2, Restriction::new(1, 2, 3, 4)),
            Zone::new(4, 1, Restriction::new(3, 1, 1, 0)),
            Zone::new(4, 3, Restriction::new(1, 0, 0, 1)),
            Zone::new(6, 4, Restriction::new(0, 0, 1, 1)),
            Zone::new(9, 1, Restriction::new(1, 0, 0, 2)),
            Zone::new(9, 4, Restriction::new(2, 0, 0, 1)),
            Zone::new(9, 3, Restriction::new(0, 0, 0, 0)),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn o1_climbing_wall() {
        let cur_zone = Zone::new(2,2, Restriction::default());

        let zones = vec![
            Zone::new(4, 1, Restriction::default()),
            Zone::new(5, 2, Restriction::default()), // correct
            Zone::new(3, 4, Restriction::default()),
            Zone::new(2, 5, Restriction::default()), // correct
        ];

        let correct_zones = vec![
            Zone::new(5, 2, Restriction::default()),
            Zone::new(2, 5, Restriction::default()),
        ];
        assert_eq!(cur_zone.climbing_wall_o1(&zones), correct_zones);
    }

}
