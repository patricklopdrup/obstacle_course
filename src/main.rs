
fn main() {
    println!("Hello, world!");
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
struct Zone {
    x: i32,
    y: i32,
}

impl Zone {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euclidian_dist_horizontal() {
        let zone1 = Zone::new(0,0);
        let zone2 = Zone::new(0,5);
        assert_eq!(zone1.get_euclidian_distance(&zone2), 5f64);
    }

    #[test]
    fn euclidian_dist_vertical() {
        let zone1 = Zone::new(3,0);
        let zone2 = Zone::new(0,0);
        assert_eq!(zone1.get_euclidian_distance(&zone2), 3f64);
    }

    #[test]
    fn euclidian_dist_diagonal() {
        let zone1 = Zone::new(0,0);
        let zone2 = Zone::new(2,4);
        let dist_round_2_decimal = (zone1.get_euclidian_distance(&zone2) * 100f64).round() / 100f64;
        assert_eq!(dist_round_2_decimal, 4.47);
    }

    #[test]
    fn furthest_away_4_zones() {
        let cur_zone = Zone::new(2,8);
        let zone_furthest_away = Zone::new(7,3);
        let zones = vec![
            Zone::new(4, 7),
            Zone::new(3, 4),
            zone_furthest_away,
        ];
        assert_eq!(cur_zone.get_furthest_away(&zones), zone_furthest_away);
    }
}
