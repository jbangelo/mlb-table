#!/usr/bin/env python3

from urllib.request import urlopen
from urllib.parse import urlencode
import json
import csv
import io

def get_stats(params):
    params = urlencode(params)
    url = f"https://statsapi.mlb.com/api/v1/stats?{params}"
    resp = urlopen(url)
    return json.load(resp)

def extract_keys(data):
    keys = []
    for stats in data['stats']:
        for player in stats['splits']:
            for key in player['stat'].keys():
                if key not in keys:
                    keys.append(key)
    keys = ["player","team"] + keys
    return keys

def get_lines(data, keys):
    lines = []
    for stats in data['stats']:
        for player in stats['splits']:
            lines.append(player['stat'] | {
                    'player': player['player']['fullName'],
                    'team': player['team']['name'],
                    })
    return lines

def write_csv(data, keys, f):
    w = csv.DictWriter(f, keys)
    w.writeheader()
    for line in data:
        w.writerow(line)

def main(f, group, season, fmt):
    supported_groups = ["hitting", "pitching", "fielding"]
    if group not in supported_groups:
        raise ValueError(f"Unsupported group: '{group}'")
    
    supported_fmt = ["csv"]
    if fmt not in supported_fmt:
        raise ValueError(f"Unsupported format: '{fmt}'")

    params = {"stats": "season", "group": group, "season": season, "limit": 2000, "playerPool": "All"}
    data = get_stats(params)
    keys = extract_keys(data)
    lines = get_lines(data, keys)
    if fmt == "csv":
        write_csv(lines, keys, f)


if __name__=="__main__":
    f = io.StringIO()
    main(f, "pitching", 2024, "csv")
    print(f.getvalue())

