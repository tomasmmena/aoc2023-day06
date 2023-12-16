use std::env;
use std::fs;
use std::io::{self, BufRead};

struct BoatRace {
    time: usize,
    record: usize
}

impl BoatRace {
    /// This function returns the number of solutions that manage a longer travel distance than the record.
    /// Distance traveled is equal to the time spent pressing the button multiplied by the time spent travelling, 
    /// this means, in geometric terms we are trying to maximize the surface of a tegragon with a constant perimeter,
    /// for which the optimal solution is the square. We can start from that point and move in either direction, 
    /// increasing or decreasing press or travel time and count the solutions that remain above the record to then 
    /// multiply by 2, since multiplication is conmutative. We can make this in to a bisection search to make it 
    /// faster.
    /// 
    /// For now this function does not implement bisection searching.
    fn solution_count(&self) -> usize {
        let mut solutions: usize = 0;
        let mut pressing_time: usize = self.time / 2;
        let mut travellign_time: usize = self.time / 2;
        if self.time % 2 == 1 {
            pressing_time += 1;
            solutions += 1;
        }
        if pressing_time * travellign_time > self.record {
            solutions += 1;
        } else {
            return 0;
        }
        while travellign_time > 0 {
            pressing_time += 1;
            travellign_time -= 1;
            if pressing_time * travellign_time > self.record { solutions += 2 } else { break; }
        }
        solutions
    }
}


fn main() {
    let path = env::args().nth(1).expect("Missing required parameter path!");

    let mut data = io::BufReader::new(
        fs::File::open(path).expect("Could not open file!"))
        .lines();

    let times: Vec<usize> = data
        .next()
        .expect("Unexpected EOF when reading times.")
        .expect("Could not read line for times.")
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse::<usize>().expect("Invalid time!"))
        .collect();

    let records: Vec<usize> = data
        .next()
        .expect("Unexpected EOF when reading records.")
        .expect("Could not read line for records.")
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse::<usize>().expect("Invalid record!"))
        .collect();

    let races: Vec<BoatRace> = times
        .iter()
        .zip(records.iter())
        .map(|(time, record)| BoatRace {
            time: *time,
            record: *record
        })
        .collect();

    println!(
        "Productory of solutions: {}",
        races
            .into_iter()
            .map(|r| r.solution_count())
            .reduce(|a, b| a * b)
            .expect("Could not calculate productory!")
    )

}


#[test]
fn test_solution_count() {
    let race_1 = BoatRace {
        time: 7, record: 9
    };
    assert_eq!(race_1.solution_count(), 4);

    let race_1 = BoatRace {
        time: 15, record: 40
    };
    assert_eq!(race_1.solution_count(), 8);

    let race_1 = BoatRace {
        time: 30, record: 200
    };
    assert_eq!(race_1.solution_count(), 9);
}
