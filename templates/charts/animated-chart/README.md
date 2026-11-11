# Animated chart

**Category:** Charts · **Canvas:** 1920 × 1080 · **Duration:** 18 seconds, or 6 seconds for a single family

A count-up metric and staggered chart entrance, followed by bar → line → area → doughnut → pie → polar area → radar → scatter → bubble → mixed. Colours and legend order stay consistent. Bars round into centred circles before travelling to line positions; the trend follows the area boundary into the doughnut’s outer edge before fading. Radial charts preserve circular edges, radar sectors gather into dots, and bubbles travel before expanding into bars. Each family change takes 1.4 seconds, followed by a 0.3-second hold. The delta appears after the count-up.

Original Mont vector geometry, rendered natively. Chart.js is the family reference, not a runtime dependency.

[Watch the showcase](../../../assets/previews/animated-chart.mp4)

```sh
npx usemont templates preview animated-chart
```

## Create and render

Requires Mont CLI and Python 3. Use a new workspace path.

```sh
python3 create.py /tmp/chart-showcase --output /tmp/chart-showcase.mp4
python3 create.py /tmp/chart-line --mode line --inputs inputs.example.json --output /tmp/chart-line.mp4
```

`--mode` accepts `showcase`, `bar`, `line`, `area`, `doughnut`, `pie`, `polar`, `radar`, `scatter`, `bubble`, and `mixed`. `--width` and `--height` accept even-sized 16:9 canvases, minimum 320 × 180. Default export: 1920 × 1080 at 30 fps.

The workspace retains the editable TSX and variables. Run the creator again in a new workspace when changing the data series, so its proportional geometry and tick labels are regenerated together.

## Inputs

Supply a JSON object through `--inputs`; see `inputs.example.json`.

| Input | Meaning |
| --- | --- |
| `label`, `value`, `delta` | Metric label, non-negative count-up target, and formatted delta string. `value` defaults to the series total and is displayed rounded to an integer. |
| `comparison` | Comparison period for the delta. |
| `series` | Three to six categories with `label` and non-negative `value`; the total must be positive. Labels: 1–18 characters. |
| `series[].hours` | Scatter/bubble x coordinate, 0–10; default 5. |
| `series[].team` | Bubble area measure, 1–36; default 10. |
| `series[].target` | Mixed-chart plan value; defaults to `value * 0.82 + 100`. |
| `palette` | Six `#RRGGBB` category colours. |
| `background`, `surface`, `ink`, `muted`, `grid` | Stage, panel, text and grid colours. |

Pie and doughnut use shares of the total. Polar sectors have equal angles and area proportional to value. Bubble area is proportional to team size. Cartesian and radar charts share a maximum that includes actual and plan values. Scatter/bubble keep the same y values but use editing hours on x; mixed returns to category positions.

## Check

```sh
python3 check.py
```

Requires Node.js. Checks invalid datasets, finite in-bounds geometry, proportional polar/bubble areas, digit boundaries, centred bar-to-circle morphs, matching trend/fill boundaries, 1.4-second morphs with 0.3-second holds, and continuity at every family transition. Preview exports are reviewed before publication.
