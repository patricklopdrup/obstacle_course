use std::collections::{HashMap};


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
    is_end_zone: bool,
}

impl Zone {
    fn new(x: i32, y: i32, restriction: Restriction) -> Self {
        Self { x, y, restriction, is_end_zone: false }
    }

    fn get_length_from_origin(&self) -> f32 {
        let origin: Zone = Zone::new(0, 0, Restriction::default());
        self.get_euclidian_distance(&origin)
    }
    
    fn bungee_slingshow_o2(&self, zones: &Vec<Zone>) -> Zone {
        let default_zone = Zone::default();
        let mut result: &Zone = &default_zone;
        let mut max_dist: f32 = f32::MIN;
        let mut cur_dist: f32;
        for zone in zones {
            cur_dist = self.get_euclidian_distance(&zone);
            if cur_dist > max_dist {
                max_dist = cur_dist;
                result = zone;
            }
        }
        
        *result
    }

    fn get_euclidian_distance(&self, other: &Zone) -> f32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f32).sqrt()
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

    fn salmon_ladder_slide_o3(&self, zones: &Vec<Zone>) -> Vec<Zone> {
        let mut result: Vec<Zone> = Vec::with_capacity(zones.len());
        let mut candidates: HashMap<u32, Vec<Zone>> = HashMap::new();
        let mut cur_slope_value_round: u32;
        for zone in zones {
            cur_slope_value_round = get_hash_key_from_f32(self.get_slope_value(zone));

            candidates.entry(cur_slope_value_round)
                .or_insert(Vec::new())
                .push(*zone);
        }

        for (key, val) in candidates.iter_mut() {
            if val.len() < 3 {
                continue;
            }
            self.get_zones_after_two_zone_on_same_line(val, &mut result);
        }

        result
    }

    fn get_zones_after_two_zone_on_same_line(&self, zones: &Vec<Zone>, result: &mut Vec<Zone>) {
        // TODO: make this method
        // Sort zones
        let mut sorted_zones: Vec<Zone> = Vec::new();
        zones.clone_into(&mut sorted_zones);
        if is_line_vertical(&sorted_zones) {
            sorted_zones.sort_unstable_by_key(|v| v.y);
        } else {
            sorted_zones.sort_unstable_by_key(|v| v.x);
        }
        
        // Get index of self in sorted array
        let self_index: usize;
        for (i, zone) in sorted_zones.iter().enumerate() {
            if self.x == zone.x && self.y == zone.y {
                self_index = i;
                break;
            }
        }

        // return other zones if 2 before or after self_index
    }

    fn get_slope_value(&self, other: &Zone) -> f32 {
        (other.y - self.y).abs() as f32 / (other.x - self.x).abs() as f32
    }

}

fn get_hash_key_from_f32(float_val: f32) -> u32 {
    (float_val * 100_000.0) as u32
}

fn is_line_vertical(zones: &Vec<Zone>) -> bool {
    let x_val = zones[0].x;
    for zone in zones {
        if zone.x != x_val {
            return false;
        }
    }
    true
}

fn is_line_horizontal(zones: &Vec<Zone>) -> bool {
    let y_val = zones[0].y;
    for zone in zones {
        if zone.y != y_val {
            return false;
        }
    }
    true
}

// fn sort_zones(zones: &Vec<Zone>) -> Vec<Zone> {

// }

// maybe optimize with return &[Zone] and using lifetimes
fn construct_zones_for_test(size: usize, lines: &[&str]) -> Vec<Zone> {
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

    zones[size-1].is_end_zone = true;

    zones
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euclidian_dist_horizontal() {
        let zone1 = Zone::new(0,0, Restriction::default());
        let zone2 = Zone::new(0,5, Restriction::default());
        assert_eq!(zone1.get_euclidian_distance(&zone2), 5f32);
    }

    #[test]
    fn euclidian_dist_vertical() {
        let zone1 = Zone::new(3,0, Restriction::default());
        let zone2 = Zone::new(0,0, Restriction::default());
        assert_eq!(zone1.get_euclidian_distance(&zone2), 3f32);
    }

    #[test]
    fn euclidian_dist_diagonal() {
        let zone1 = Zone::new(0,0, Restriction::default());
        let zone2 = Zone::new(2,4, Restriction::default());
        let dist_round_2_decimal = (zone1.get_euclidian_distance(&zone2) * 100f32).round() / 100f32;
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
        assert_eq!(cur_zone.bungee_slingshow_o2(&zones), zone_furthest_away);
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

        let mut expected = vec![
            Zone::new(0, 1, Restriction::new(4, 0, 2, 0)),
            Zone::new(2, 2, Restriction::new(1, 2, 3, 4)),
            Zone::new(4, 1, Restriction::new(3, 1, 1, 0)),
            Zone::new(4, 3, Restriction::new(1, 0, 0, 1)),
            Zone::new(6, 4, Restriction::new(0, 0, 1, 1)),
            Zone::new(9, 1, Restriction::new(1, 0, 0, 2)),
            Zone::new(9, 4, Restriction::new(2, 0, 0, 1)),
            Zone::new(9, 3, Restriction::new(0, 0, 0, 0)),
        ];
        expected[size-1].is_end_zone = true;

        assert_eq!(actual, expected);
        assert_eq!(actual.last().expect("not empty").is_end_zone, true);
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

    #[test]
    fn hash_key_test() {
        let float_key = 12.3456789;
        assert_eq!(get_hash_key_from_f32(float_key), 1234567);
    }

    #[test]
    fn o3_salmon_ladder_slide_slope_eq_2() {
        let cur_zone = Zone::new(2, 2, Restriction::default());

        let mut zones = vec![
            Zone::new(3, 4, Restriction::default()),
            Zone::new(4, 6, Restriction::default()),
            Zone::new(3, 3, Restriction::default()), // not on line
        ];
        let two_zones_away = Zone::new(5, 8, Restriction::default());
        zones.push(two_zones_away);

        let actual = cur_zone.salmon_ladder_slide_o3(&zones);
        assert_eq!(actual, vec![two_zones_away]);
    }

}
