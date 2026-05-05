use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let lines: Vec<&str> = match_results.lines().collect();
    let mut matches_played = HashMap::new();
    let mut matches_won = HashMap::new();
    let mut matches_drawn = HashMap::new();
    let mut matches_lost = HashMap::new();
    let mut points = HashMap::new();
    
    for line in lines {
        let parts: Vec<_> = line.split(';').collect();
        match parts[2] {
            "win" => {
                *matches_won.entry(parts[0]).or_insert(0) += 1;
                *matches_lost.entry(parts[1]).or_insert(0) += 1;
                *points.entry(parts[0]).or_insert(0) += 3;
                for i in 0..=1 {
                    *matches_played.entry(parts[i]).or_insert(0) += 1;
                    }
            }
            "loss" => {
                *matches_won.entry(parts[1]).or_insert(0) += 1;
                *matches_lost.entry(parts[0]).or_insert(0) += 1;
                *points.entry(parts[1]).or_insert(0) += 3;
                for i in 0..=1 {
                    *matches_played.entry(parts[i]).or_insert(0) += 1;
                }
            }
            "draw" => {
                for i in 0..=1 {
                    *points.entry(parts[i]).or_insert(0) += 1;
                    *matches_played.entry(parts[i]).or_insert(0) += 1;
                    *matches_drawn.entry(parts[i]).or_insert(0) += 1;
                }
            }
            _ => {}, 
        }
    }
let mut teams: Vec<&str> = matches_played.keys()
        .chain(matches_won.keys())
        .chain(matches_drawn.keys())
        .chain(matches_lost.keys())
        .chain(points.keys())
        .copied()
        .collect();
    teams.sort();
    teams.dedup();
    teams.sort_by(|a, b| {
        let pa = points.get(a).unwrap_or(&0);
        let pb = points.get(b).unwrap_or(&0);
        pb.cmp(pa).then_with(|| a.cmp(b))
    });

    let header = "Team                           | MP |  W |  D |  L |  P";
    // build result lines
    let mut result_lines = vec![header.to_string()];
    for team in teams {
        let mp = matches_played.get(team).unwrap_or(&0);
        let w = matches_won.get(team).unwrap_or(&0);
        let d = matches_drawn.get(team).unwrap_or(&0);
        let l = matches_lost.get(team).unwrap_or(&0);
        let p = points.get(team).unwrap_or(&0);
        // format with left-justified team name, right-justified numbers
        let row = format!("{:<31}| {:>2} | {:>2} | {:>2} | {:>2} | {:>2}", team, mp, w, d, l, p);
        result_lines.push(row);
    }
    // join lines into final result string
    result_lines.join("\n")
} 