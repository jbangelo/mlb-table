use std::io::Cursor;
use anyhow::{bail,Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Response<T> {
    copyright: String,
    stats: Vec<Group<T>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Group<T> {
    //group: String,
    playerPool: String,
    splits: Vec<Split<T>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    firstName: String,
    lastName: String,
    fullName: String,
    id: i32,
    link: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Position {
    abbreviation: String,
    code: String,
    name: String,
    #[serde(rename = "type")]
    t: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Team {
    id: i32,
    link: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Split<T> {
    player: Player,
    position: Position,
    rank: i32,
    season: String,
    stat: T,
    team: Team,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Hitting {
    gamesPlayed: i32,
    groundOuts: i32,
    airOuts: i32,
    runs: i32,
    doubles: i32,
    triples: i32,
    homeRuns: i32,
    strikeOuts: i32,
    baseOnBalls: i32,
    intentionalWalks: i32,
    hits: i32,
    hitByPitch: i32,
    avg: String,
    atBats: i32,
    obp: String,
    slg: String,
    ops: String,
    caughtStealing: i32,
    stolenBases: i32,
    stolenBasePercentage: String,
    groundIntoDoublePlay: i32,
    numberOfPitches: i32,
    plateAppearances: i32,
    totalBases: i32,
    rbi: i32,
    leftOnBase: i32,
    sacBunts: i32,
    sacFlies: i32,
    babip: String,
    groundOutsToAirouts: String,
    catchersInterference: i32,
    atBatsPerHomeRun: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Pitching {
    gamesPlayed: i32,
    gamesStarted: i32,
    groundOuts: i32,
    airOuts: i32,
    runs: i32,
    doubles: i32,
    triples: i32,
    homeRuns: i32,
    strikeOuts: i32,
    baseOnBalls: i32,
    intentionalWalks: i32,
    hits: i32,
    hitByPitch: i32,
    avg: String,
    atBats: i32,
    obp: String,
    slg: String,
    ops: String,
    caughtStealing: i32,
    stolenBases: i32,
    stolenBasePercentage: String,
    groundIntoDoublePlay: i32,
    numberOfPitches: i32,
    era: String,
    inningsPitched: String,
    wins: i32,
    losses: i32,
    saves: i32,
    saveOpportunities: i32,
    holds: i32,
    blownSaves: i32,
    earnedRuns: i32,
    whip: String,
    battersFaced: i32,
    outs: i32,
    gamesPitched: i32,
    completeGames: i32,
    shutouts: i32,
    strikes: i32,
    strikePercentage: String,
    hitBatsmen: i32,
    balks: i32,
    wildPitches: i32,
    pickoffs: i32,
    totalBases: i32,
    groundOutsToAirouts: String,
    winPercentage: String,
    pitchesPerInning: String,
    gamesFinished: i32,
    strikeoutWalkRatio: String,
    strikeoutsPer9Inn: String,
    walksPer9Inn: String,
    hitsPer9Inn: String,
    runsScoredPer9: String,
    homeRunsPer9: String,
    inheritedRunners: i32,
    inheritedRunnersScored: i32,
    catchersInterference: i32,
    sacBunts: i32,
    sacFlies: i32,
}

fn get_stats<T: for<'a> Deserialize<'a>>(params: &[(&str, &str)]) -> Result<Response<T>> {
    let client = reqwest::blocking::Client::new();
    let url = "https://statsapi.mlb.com/api/v1/stats";
    Ok(client.get(url)
        .query(params)
        .send()?
        .json()?)
}

fn write_stats<T: Serialize>(stats: &Response<T>) -> Result<Vec<u8>> {
    let mut w = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(Cursor::new(Vec::new()));

    for group in &stats.stats {
        for split in &group.splits {
            w.serialize((
                    &split.player.fullName,
                    &split.team.name,
                    &split.stat,
                    ))?;
        }
    }

    Ok(w.into_inner()?.into_inner())
}

pub fn run(group: &str, season: &str) -> Result<String> {
    let params = &[("stats", "season"), ("group", group), ("season", season), ("limit", "2000"), ("playerPool", "All")];
    let csv = match group {
        "hitting" => {
            let stats: Response<Hitting> = get_stats(params)?;
            write_stats(&stats)?
        },
        "pitching" => {
            let stats: Response<Pitching> = get_stats(params)?;
            write_stats(&stats)?
        },
        /*"hitting" => {
            let stats: Response<hitting> = get_stats(params)?;
            write_stats(&stats)?
        },*/
        _ => {
            bail!("Invalid group value: {}", group);
        }
    };

    Ok(String::from_utf8(csv)?)
}
