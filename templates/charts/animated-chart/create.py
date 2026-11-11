#!/usr/bin/env python3
"""Create editable native Mont charts; render the showcase or one chart family."""
import argparse
import json
import math
from pathlib import Path
import re
import subprocess
import uuid

HERE = Path(__file__).resolve().parent
MODES = ('showcase', 'bar', 'line', 'area', 'doughnut', 'pie', 'polar', 'radar', 'scatter', 'bubble', 'mixed')
DEFAULT_SERIES = [
    {'label': 'Studio', 'value': 240, 'hours': 2.2, 'team': 6},
    {'label': 'Social', 'value': 410, 'hours': 3.7, 'team': 11},
    {'label': 'Learning', 'value': 330, 'hours': 5.1, 'team': 9},
    {'label': 'Product', 'value': 590, 'hours': 6.3, 'team': 15},
    {'label': 'Events', 'value': 470, 'hours': 7.4, 'team': 8},
    {'label': 'Brand', 'value': 800, 'hours': 8.8, 'team': 20},
]

def run(*args, stdin=None):
    result = subprocess.run(args, input=stdin, capture_output=True)
    if result.returncode:
        raise RuntimeError((result.stdout + result.stderr).decode(errors='replace'))
    return result.stdout


def numeric(value, name, minimum=0, maximum=1e9):
    if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value) or not minimum <= value <= maximum:
        raise ValueError(f'{name} must be a finite number in [{minimum}, {maximum}]')
    return value


def series_tokens(rows):
    # ponytail: six categories keep labels legible; add pagination for larger reports.
    if not isinstance(rows, list) or not 3 <= len(rows) <= 6:
        raise ValueError('series must contain 3–6 categories')
    for row in rows:
        if not isinstance(row, dict) or not isinstance(row.get('label'), str) or not 1 <= len(row['label']) <= 18:
            raise ValueError('each category needs a label of 1–18 characters')
        numeric(row.get('value'), 'series.value')
        numeric(row.get('hours', 5), 'series.hours', 0, 10)
        numeric(row.get('team', 10), 'series.team', 1, 36)
        numeric(row.get('target', row['value'] * .82 + 100), 'series.target')
    total = sum(row['value'] for row in rows)
    if total <= 0:
        raise ValueError('series must have a positive total for share-based charts')
    maximum = max(max(row['value'], row.get('target', row['value'] * .82 + 100)) for row in rows)
    normalized = [row['value'] / maximum for row in rows]
    output, cumulative = [], 0
    for i, row in enumerate(rows):
        share = row['value'] / total
        output.append(dict(row, index=i, normalized=normalized[i], next=normalized[(i+1) % len(rows)],
                           left=normalized[0] if i == 0 else (normalized[i-1] + normalized[i]) / 2,
                           right=normalized[-1] if i == len(rows)-1 else (normalized[i+1] + normalized[i]) / 2,
                           share=share, start=cumulative, hours=row.get('hours', 5), team=row.get('team', 10),
                           target=row.get('target', row['value'] * .82 + 100) / maximum))
        cumulative += share
    return output, total


def create(workspace, inputs, mode, width=1920, height=1080):
    variables = dict(inputs)
    rows, total = series_tokens(variables.pop('series', DEFAULT_SERIES))
    palette = variables.pop('palette', None)
    if palette is not None:
        if not isinstance(palette, list) or len(palette) != 6 or any(not isinstance(c, str) or not re.fullmatch(r'#[0-9a-fA-F]{6}', c) for c in palette):
            raise ValueError('palette must contain six #RRGGBB colours')
        variables.update({f'color{i+1}': color for i, color in enumerate(palette)})
    variables.setdefault('value', total)
    numeric(variables['value'], 'value')
    variables.update(mode=mode, width=width, height=height, axisMax=max(row['value'] / row['normalized'] for row in rows if row['normalized'] > 0))
    source = (HERE / 'scene.tsx').read_text()
    source, count = re.subn(r'tokens: \{series: \[.*?\], ticks:', lambda _: 'tokens: {series: ' + json.dumps(rows, ensure_ascii=False) + ', ticks:', source, count=1, flags=re.S)
    source = source.replace('tickLabels: ["0", "200", "400", "600", "800"]', 'tickLabels: ' + json.dumps([str(round(variables['axisMax'] * tick)) for tick in (0, .25, .5, .75, 1)]))
    if count != 1:
        raise ValueError('scene has no category token block')
    duration = 18000 if mode == 'showcase' else 6000
    source = source.replace('durationMs: 18000, editPreviewMs: 12300', f'durationMs: {duration}, editPreviewMs: {min(3000, duration)}')
    run('mont', 'init', str(workspace), '--json')
    def dispatch(command):
        return run('mont', 'dispatch', '-C', str(workspace), '-', '--json', stdin=json.dumps({'commandId': str(uuid.uuid4()), 'command': command}).encode())
    for key, value in (('sceneWidth', width), ('sceneHeight', height)):
        dispatch({'type': 'setProjectValue', 'pointer': '/' + key, 'value': value})
    dispatch({'type': 'user_intent:programmatic_span.create', 'request': {
        'requestedId': 'animated-chart',
        'placement': {'startMs': 0, 'durationMs': duration, 'trackId': 'template', 'layer': 0},
        'capability': {'kind': 'programmatic.scene', 'version': 1, 'state': {
            'sourceTsx': source, 'variables': variables, 'frame': {'x': 0, 'y': 0, 'width': width, 'height': height}}},
        'interaction': {'selectable': True},
        'producer': {'kind': 'programmatic', 'id': 'mont-templates', 'version': '1'}}})
    return workspace


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('workspace', type=Path)
    parser.add_argument('--inputs', type=Path)
    parser.add_argument('--mode', choices=MODES, default='showcase')
    parser.add_argument('--output', type=Path)
    parser.add_argument('--width', type=int, default=1920)
    parser.add_argument('--height', type=int, default=1080)
    args = parser.parse_args()
    if args.width < 320 or args.height < 180 or args.width * 9 != args.height * 16 or args.width % 2 or args.height % 2:
        parser.error('canvas must be even-sized 16:9, at least 320×180')
    inputs = json.loads(args.inputs.read_text()) if args.inputs else {}
    if not isinstance(inputs, dict):
        parser.error('inputs must be a JSON object')
    create(args.workspace.resolve(), inputs, args.mode, args.width, args.height)
    if args.output:
        run('mont', 'render', '-C', str(args.workspace.resolve()), '--output', str(args.output.resolve()), '--fps', '30', '--json')
    print(json.dumps({'workspace': str(args.workspace.resolve()), 'mode': args.mode, 'output': str(args.output.resolve()) if args.output else None}))
