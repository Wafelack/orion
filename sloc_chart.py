#!/usr/bin/env python3
from matplotlib import pyplot as plt
from datetime import datetime, timedelta
import subprocess
import json

hashes=[h.split(' ')[0] for h in subprocess.run(['git', 'log', '--oneline'], capture_output=True).stdout.decode('utf-8').strip().split('\n')]
commit_date = subprocess.run(['git', 'show', '-s', '--oneline', '--format=%ci', hashes[len(hashes) - 1]], capture_output=True).stdout.decode('utf-8').strip().split(' ')[0]
dateified = datetime.strptime(commit_date, '%Y-%m-%d')
today = datetime.now()
dates = []
sloc = []

while dateified < today:
    date = dateified - timedelta(days=1)
    stringified = f'{date.year}-{date.month}-{date.day}'
    commit_hash = subprocess.run(['git', 'log', '--oneline', f'--after="{stringified}"'], capture_output=True).stdout.decode('utf-8').strip().split('\n')[-1].split(' ')[0]
    print(f'{stringified}:{commit_hash} => ', end='')
    subprocess.run(['git', 'checkout', commit_hash])
    stats=subprocess.run(['tokei', '-o', 'json'], capture_output=True).stdout.decode('utf-8').strip()
    print(json.loads(stats)['Rust']['code'])
    subprocess.run(['git', 'switch', '-'])
    dateified=dateified + timedelta(weeks=1)


plt.plot(dates, sloc)
plt.xlabel('date')
plt.ylabel('SLOC count')
plt.show()
