#!/usr/bin/env python3

from urllib.request import urlopen
import json
import csv
import io

def main():
    url = "https://statsapi.mlb.com/api/v1/stats?stats=season&group=hitting&season=2024&limit=2000&playerPool=All"
    keys = ['gamesPlayed', 'groundOuts', 'airOuts', 'runs', 'doubles', 'triples',
            'homeRuns', 'strikeOuts', 'baseOnBalls', 'intentionalWalks', 'hits',
            'hitByPitch', 'avg', 'atBats', 'obp', 'slg', 'ops', 'caughtStealing',
            'stolenBases', 'stolenBasePercentage', 'groundIntoDoublePlay',
            'numberOfPitches', 'plateAppearances', 'totalBases', 'rbi', 'leftOnBase',
            'sacBunts', 'sacFlies', 'babip', 'groundOutsToAirouts',
            'catchersInterference', 'atBatsPerHomeRun']
    resp = urlopen(url)
    print(f"{resp}")
    
    f = io.StringIO()
    w = csv.DictWriter(f, ['player', 'team']+keys)
    w.writeheader()

    data = json.load(resp)
    #print(data)

    for stats in data['stats']:
        for player in stats['splits']:
            data = player['stat'] | {
                    'player': player['player']['fullName'],
                    'team': player['team']['name'],
                    }
            w.writerow(data)
            #print(player)

    print(f.getvalue())

if __name__=="__main__":
    main()

