// One continuous shared plane: no individually re-spaced or billboarded tiles.
function clamp01(x) { return Math.max(0, Math.min(1, x)); }
function smoother(p) { return p * p * p * (p * (p * 6 - 15) + 10); }
function reveal(t, start, duration) { return smoother(clamp01((t - start) / duration)); }
function wallScale(t, hold, duration) { return 3.36 - 2.36 * reveal(t, hold, duration); }
function wallLean(t, start, duration) { return reveal(t, start, duration); }
function isoSquash(lean) { return 1 - (1 - 0.5773502691896258) * lean; }
function wallRotation(lean) { return -45 * lean; }
function panTravel(p) { return p <= 0 ? 0 : p >= 1 ? p - 0.5 : p * p * p * (1 - 0.5 * p); }
function wallPan(t, start, duration, speed) { return speed * duration / 1000 * panTravel((t - start) / duration); }

export default defineSpanScene({
  id: "tile-wall-reveal",
  width: 960, height: 960, durationMs: 6000, editPreviewMs: 4000,
  variables: {
    width: numberVar(960), height: numberVar(960), speed: numberVar(1),
    holdMs: numberVar(350), pullbackMs: numberVar(1100), tiltMs: numberVar(1500),
    panSpeed: numberVar(70), background: colorVar("#080D19"),
  },
  tokens: {
    // Original vector mini-tools. Header labels and captions remain native editable Text.
    editArt: "data:image/svg+xml,<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 420 262\"><defs><linearGradient id=\"sky\" gradientUnits=\"userSpaceOnUse\" x1=\"18\" y1=\"17\" x2=\"264\" y2=\"162\"><stop offset=\"0\" stop-color=\"%238297E8\"/><stop offset=\"1\" stop-color=\"%23E6B9BF\"/></linearGradient></defs><path fill=\"%23111A2B\" d=\"M0 0h420v262H0z\"/><rect x=\"18\" y=\"17\" width=\"246\" height=\"145\" rx=\"7\" fill=\"url(%23sky)\"/><circle cx=\"155\" cy=\"88\" r=\"48\" fill=\"%23F6CF90\"/><path d=\"M18 146Q91 78 167 126T264 113V162H18\" fill=\"%23344B62\"/><path d=\"M18 150Q111 124 171 147T264 138V162H18\" fill=\"%231B3244\"/><rect x=\"282\" y=\"17\" width=\"120\" height=\"145\" rx=\"7\" fill=\"%231C2940\"/><rect x=\"295\" y=\"33\" width=\"65\" height=\"6\" rx=\"3\" fill=\"%23A5B5D6\"/><g fill=\"%233A4A66\"><rect x=\"295\" y=\"54\" width=\"92\" height=\"5\" rx=\"2\"/><rect x=\"295\" y=\"79\" width=\"92\" height=\"5\" rx=\"2\"/><rect x=\"295\" y=\"104\" width=\"92\" height=\"5\" rx=\"2\"/><rect x=\"295\" y=\"129\" width=\"92\" height=\"5\" rx=\"2\"/></g><g fill=\"%23B7C5FF\"><circle cx=\"352\" cy=\"56\" r=\"4\"/><circle cx=\"323\" cy=\"81\" r=\"4\"/><circle cx=\"372\" cy=\"106\" r=\"4\"/><circle cx=\"341\" cy=\"131\" r=\"4\"/></g><path stroke=\"%233A4960\" d=\"M18 181H402M18 203H402M18 225H402M18 247H402\"/><g fill=\"%238297E8\"><rect x=\"20\" y=\"186\" width=\"105\" height=\"12\" rx=\"3\"/><rect x=\"129\" y=\"186\" width=\"150\" height=\"12\" rx=\"3\"/><rect x=\"283\" y=\"186\" width=\"116\" height=\"12\" rx=\"3\"/></g><g fill=\"%236BAF9D\"><rect x=\"20\" y=\"208\" width=\"181\" height=\"12\" rx=\"3\"/><rect x=\"205\" y=\"208\" width=\"194\" height=\"12\" rx=\"3\"/></g><g fill=\"%23BD8BBD\"><rect x=\"65\" y=\"230\" width=\"88\" height=\"12\" rx=\"3\"/><rect x=\"174\" y=\"230\" width=\"151\" height=\"12\" rx=\"3\"/></g><path stroke=\"%23F8D385\" stroke-width=\"2\" d=\"M173 175V250\"/><path fill=\"%23F8D385\" d=\"M168 174h10v7l-5 5-5-5z\"/></svg>",
    captionArt: "data:image/svg+xml,<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 420 262\"><path fill=\"%23ECEAE2\" d=\"M0 0h420v262H0z\"/><path d=\"M262 0H420V134C338 141 268 100 262 0Z\" fill=\"%23DBD9EF\"/><path d=\"M302 0H420V95C356 97 304 62 302 0Z\" fill=\"%23E6E3ED\"/><rect x=\"25\" y=\"19\" width=\"80\" height=\"19\" rx=\"9\" fill=\"%23D9D4E9\"/><rect x=\"34\" y=\"26\" width=\"60\" height=\"4\" rx=\"2\" fill=\"%2387829F\"/><rect x=\"21\" y=\"68\" width=\"270\" height=\"48\" rx=\"9\" fill=\"%23D5D2F0\"/><path stroke=\"%23C9C5D2\" d=\"M22 195H398\"/><rect x=\"24\" y=\"218.0\" width=\"4\" height=\"8\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"32\" y=\"211.0\" width=\"4\" height=\"22\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"40\" y=\"207.0\" width=\"4\" height=\"30\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"48\" y=\"208.0\" width=\"4\" height=\"28\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"56\" y=\"214.0\" width=\"4\" height=\"16\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"64\" y=\"215.0\" width=\"4\" height=\"14\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"72\" y=\"208.5\" width=\"4\" height=\"27\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"80\" y=\"207.0\" width=\"4\" height=\"30\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"88\" y=\"210.5\" width=\"4\" height=\"23\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"96\" y=\"217.5\" width=\"4\" height=\"9\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"104\" y=\"211.5\" width=\"4\" height=\"21\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"112\" y=\"207.0\" width=\"4\" height=\"30\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"120\" y=\"208.0\" width=\"4\" height=\"28\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"128\" y=\"213.0\" width=\"4\" height=\"18\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"136\" y=\"215.5\" width=\"4\" height=\"13\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"144\" y=\"209.0\" width=\"4\" height=\"26\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"152\" y=\"207.0\" width=\"4\" height=\"30\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"160\" y=\"209.5\" width=\"4\" height=\"25\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"168\" y=\"216.5\" width=\"4\" height=\"11\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"176\" y=\"212.5\" width=\"4\" height=\"19\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"184\" y=\"207.5\" width=\"4\" height=\"29\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"192\" y=\"207.5\" width=\"4\" height=\"29\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"200\" y=\"212.5\" width=\"4\" height=\"19\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"208\" y=\"216.5\" width=\"4\" height=\"11\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"216\" y=\"209.5\" width=\"4\" height=\"25\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"224\" y=\"207.0\" width=\"4\" height=\"30\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"232\" y=\"209.0\" width=\"4\" height=\"26\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"240\" y=\"215.5\" width=\"4\" height=\"13\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"248\" y=\"213.0\" width=\"4\" height=\"18\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"256\" y=\"207.5\" width=\"4\" height=\"29\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"264\" y=\"207.0\" width=\"4\" height=\"30\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"272\" y=\"211.5\" width=\"4\" height=\"21\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"280\" y=\"217.0\" width=\"4\" height=\"10\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"288\" y=\"210.0\" width=\"4\" height=\"24\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"296\" y=\"207.0\" width=\"4\" height=\"30\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"304\" y=\"208.5\" width=\"4\" height=\"27\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"312\" y=\"215.0\" width=\"4\" height=\"14\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"320\" y=\"214.0\" width=\"4\" height=\"16\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"328\" y=\"208.0\" width=\"4\" height=\"28\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"336\" y=\"207.0\" width=\"4\" height=\"30\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"344\" y=\"211.0\" width=\"4\" height=\"22\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"352\" y=\"218.0\" width=\"4\" height=\"8\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"360\" y=\"211.0\" width=\"4\" height=\"22\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"368\" y=\"207.0\" width=\"4\" height=\"30\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"376\" y=\"208.0\" width=\"4\" height=\"28\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"384\" y=\"214.0\" width=\"4\" height=\"16\" rx=\"2\" fill=\"%238F8EC5\"/><rect x=\"392\" y=\"214.5\" width=\"4\" height=\"15\" rx=\"2\" fill=\"%238F8EC5\"/></svg>",
    recordArt: "data:image/svg+xml,<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 420 262\"><defs><linearGradient id=\"desk\" gradientUnits=\"userSpaceOnUse\" x1=\"28\" y1=\"28\" x2=\"278\" y2=\"193\"><stop offset=\"0\" stop-color=\"%237788BF\"/><stop offset=\"1\" stop-color=\"%23333E66\"/></linearGradient></defs><path fill=\"%23CBD0EC\" d=\"M0 0h420v262H0z\"/><rect x=\"20\" y=\"19\" width=\"266\" height=\"184\" rx=\"10\" fill=\"%231C2540\"/><rect x=\"28\" y=\"28\" width=\"250\" height=\"165\" rx=\"5\" fill=\"url(%23desk)\"/><rect x=\"66\" y=\"59\" width=\"173\" height=\"107\" rx=\"9\" fill=\"%23E9E7E0\"/><rect x=\"77\" y=\"69\" width=\"151\" height=\"13\" rx=\"3\" fill=\"%23CACCE5\"/><rect x=\"78\" y=\"94\" width=\"89\" height=\"56\" rx=\"5\" fill=\"%23889DCC\"/><circle cx=\"132\" cy=\"122\" r=\"19\" fill=\"%23F1CFA1\"/><rect x=\"176\" y=\"94\" width=\"43\" height=\"6\" rx=\"3\" fill=\"%23ABABC0\"/><rect x=\"176\" y=\"108\" width=\"31\" height=\"5\" rx=\"2\" fill=\"%23C8C5CE\"/><rect x=\"176\" y=\"122\" width=\"36\" height=\"5\" rx=\"2\" fill=\"%23C8C5CE\"/><rect x=\"300\" y=\"19\" width=\"100\" height=\"184\" rx=\"10\" fill=\"%23E5E7F5\"/><circle cx=\"350\" cy=\"143\" r=\"25\" fill=\"%23F2B8BE\"/><circle cx=\"350\" cy=\"143\" r=\"13\" fill=\"%23D65F73\"/><rect x=\"20\" y=\"220\" width=\"230\" height=\"7\" rx=\"3\" fill=\"%23A5ACD0\"/><rect x=\"20\" y=\"236\" width=\"160\" height=\"6\" rx=\"3\" fill=\"%23B4BAD8\"/><rect x=\"302\" y=\"222\" width=\"98\" height=\"22\" rx=\"11\" fill=\"%23A3AAD1\"/></svg>",
    motionArt: "data:image/svg+xml,<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 420 262\"><path fill=\"%23172923\" d=\"M0 0h420v262H0z\"/><g stroke=\"%2330463B\" stroke-width=\"1\"><path d=\"M25 86H395M25 132H395M25 178H395M25 224H395M71 66V240M140 66V240M210 66V240M280 66V240M349 66V240\"/></g><path d=\"M30 222C95 222 102 107 174 153S297 78 389 78\" fill=\"none\" stroke=\"%23D8E9A4\" stroke-width=\"6\" stroke-linecap=\"round\"/><g fill=\"%23D8E9A4\"><circle cx=\"30\" cy=\"222\" r=\"7\"/><circle cx=\"174\" cy=\"153\" r=\"7\"/><circle cx=\"389\" cy=\"78\" r=\"7\"/></g><g stroke=\"%2383AC9C\" stroke-width=\"2\" stroke-dasharray=\"4 4\"><path d=\"M30 222L95 222M174 153L102 107M174 153L246 199M389 78L297 78\"/></g><g fill=\"%2383AC9C\"><circle cx=\"95\" cy=\"222\" r=\"4\"/><circle cx=\"102\" cy=\"107\" r=\"4\"/><circle cx=\"246\" cy=\"199\" r=\"4\"/><circle cx=\"297\" cy=\"78\" r=\"4\"/></g></svg>",
    vignette: "data:image/svg+xml,<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 420 262\"><defs><radialGradient id=\"v\"><stop offset=\".2\" stop-color=\"%23030813\" stop-opacity=\"0\"/><stop offset=\"1\" stop-color=\"%23030813\" stop-opacity=\".8\"/></radialGradient></defs><path fill=\"url(%23v)\" d=\"M0 0h420v262H0z\"/></svg>",
    tiles: [
      {"column":-3,"row":-5,"kind":3,"name":"MOTION","caption":"Give the story momentum"},
      {"column":-2,"row":-5,"kind":2,"name":"RECORD","caption":"A clear point of view"},
      {"column":-1,"row":-5,"kind":3,"name":"MOTION","caption":"Give the story momentum"},
      {"column":0,"row":-5,"kind":2,"name":"RECORD","caption":"A clear point of view"},
      {"column":1,"row":-5,"kind":3,"name":"MOTION","caption":"Give the story momentum"},
      {"column":2,"row":-5,"kind":2,"name":"RECORD","caption":"A clear point of view"},
      {"column":3,"row":-5,"kind":3,"name":"MOTION","caption":"Give the story momentum"},
      {"column":-3,"row":-4,"kind":1,"name":"CAPTIONS","caption":"Say it your way"},
      {"column":-2,"row":-4,"kind":0,"name":"EDIT","caption":"Riverlight / Assembly"},
      {"column":-1,"row":-4,"kind":1,"name":"CAPTIONS","caption":"Say it your way"},
      {"column":0,"row":-4,"kind":0,"name":"EDIT","caption":"Riverlight / Assembly"},
      {"column":1,"row":-4,"kind":1,"name":"CAPTIONS","caption":"Say it your way"},
      {"column":2,"row":-4,"kind":0,"name":"EDIT","caption":"Riverlight / Assembly"},
      {"column":3,"row":-4,"kind":1,"name":"CAPTIONS","caption":"Say it your way"},
      {"column": -3, "row": -3, "kind": 3, "name": "MOTION", "caption": "Give the story momentum"},
      {"column": -2, "row": -3, "kind": 2, "name": "RECORD", "caption": "A clear point of view"},
      {"column": -1, "row": -3, "kind": 3, "name": "MOTION", "caption": "Give the story momentum"},
      {"column": 0, "row": -3, "kind": 2, "name": "RECORD", "caption": "A clear point of view"},
      {"column": 1, "row": -3, "kind": 3, "name": "MOTION", "caption": "Give the story momentum"},
      {"column": 2, "row": -3, "kind": 2, "name": "RECORD", "caption": "A clear point of view"},
      {"column": 3, "row": -3, "kind": 3, "name": "MOTION", "caption": "Give the story momentum"},
      {"column": -3, "row": -2, "kind": 1, "name": "CAPTIONS", "caption": "Say it your way"},
      {"column": -2, "row": -2, "kind": 0, "name": "EDIT", "caption": "Riverlight / Assembly"},
      {"column": -1, "row": -2, "kind": 1, "name": "CAPTIONS", "caption": "Say it your way"},
      {"column": 0, "row": -2, "kind": 0, "name": "EDIT", "caption": "Riverlight / Assembly"},
      {"column": 1, "row": -2, "kind": 1, "name": "CAPTIONS", "caption": "Say it your way"},
      {"column": 2, "row": -2, "kind": 0, "name": "EDIT", "caption": "Riverlight / Assembly"},
      {"column": 3, "row": -2, "kind": 1, "name": "CAPTIONS", "caption": "Say it your way"},
      {"column": -3, "row": -1, "kind": 3, "name": "MOTION", "caption": "Give the story momentum"},
      {"column": -2, "row": -1, "kind": 2, "name": "RECORD", "caption": "A clear point of view"},
      {"column": -1, "row": -1, "kind": 3, "name": "MOTION", "caption": "Give the story momentum"},
      {"column": 0, "row": -1, "kind": 2, "name": "RECORD", "caption": "A clear point of view"},
      {"column": 1, "row": -1, "kind": 3, "name": "MOTION", "caption": "Give the story momentum"},
      {"column": 2, "row": -1, "kind": 2, "name": "RECORD", "caption": "A clear point of view"},
      {"column": 3, "row": -1, "kind": 3, "name": "MOTION", "caption": "Give the story momentum"},
      {"column": -3, "row": 0, "kind": 1, "name": "CAPTIONS", "caption": "Say it your way"},
      {"column": -2, "row": 0, "kind": 0, "name": "EDIT", "caption": "Riverlight / Assembly"},
      {"column": -1, "row": 0, "kind": 1, "name": "CAPTIONS", "caption": "Say it your way"},
      {"column": 0, "row": 0, "kind": 0, "name": "EDIT", "caption": "Riverlight / Assembly"},
      {"column": 1, "row": 0, "kind": 1, "name": "CAPTIONS", "caption": "Say it your way"},
      {"column": 2, "row": 0, "kind": 0, "name": "EDIT", "caption": "Riverlight / Assembly"},
      {"column": 3, "row": 0, "kind": 1, "name": "CAPTIONS", "caption": "Say it your way"},
      {"column": -3, "row": 1, "kind": 3, "name": "MOTION", "caption": "Give the story momentum"},
      {"column": -2, "row": 1, "kind": 2, "name": "RECORD", "caption": "A clear point of view"},
      {"column": -1, "row": 1, "kind": 3, "name": "MOTION", "caption": "Give the story momentum"},
      {"column": 0, "row": 1, "kind": 2, "name": "RECORD", "caption": "A clear point of view"},
      {"column": 1, "row": 1, "kind": 3, "name": "MOTION", "caption": "Give the story momentum"},
      {"column": 2, "row": 1, "kind": 2, "name": "RECORD", "caption": "A clear point of view"},
      {"column": 3, "row": 1, "kind": 3, "name": "MOTION", "caption": "Give the story momentum"},
      {"column": -3, "row": 2, "kind": 1, "name": "CAPTIONS", "caption": "Say it your way"},
      {"column": -2, "row": 2, "kind": 0, "name": "EDIT", "caption": "Riverlight / Assembly"},
      {"column": -1, "row": 2, "kind": 1, "name": "CAPTIONS", "caption": "Say it your way"},
      {"column": 0, "row": 2, "kind": 0, "name": "EDIT", "caption": "Riverlight / Assembly"},
      {"column": 1, "row": 2, "kind": 1, "name": "CAPTIONS", "caption": "Say it your way"},
      {"column": 2, "row": 2, "kind": 0, "name": "EDIT", "caption": "Riverlight / Assembly"},
      {"column": 3, "row": 2, "kind": 1, "name": "CAPTIONS", "caption": "Say it your way"},
      {"column": -3, "row": 3, "kind": 3, "name": "MOTION", "caption": "Give the story momentum"},
      {"column": -2, "row": 3, "kind": 2, "name": "RECORD", "caption": "A clear point of view"},
      {"column": -1, "row": 3, "kind": 3, "name": "MOTION", "caption": "Give the story momentum"},
      {"column": 0, "row": 3, "kind": 2, "name": "RECORD", "caption": "A clear point of view"},
      {"column": 1, "row": 3, "kind": 3, "name": "MOTION", "caption": "Give the story momentum"},
      {"column": 2, "row": 3, "kind": 2, "name": "RECORD", "caption": "A clear point of view"},
      {"column": 3, "row": 3, "kind": 3, "name": "MOTION", "caption": "Give the story momentum"},
      {"column":-3,"row":4,"kind":1,"name":"CAPTIONS","caption":"Say it your way"},
      {"column":-2,"row":4,"kind":0,"name":"EDIT","caption":"Riverlight / Assembly"},
      {"column":-1,"row":4,"kind":1,"name":"CAPTIONS","caption":"Say it your way"},
      {"column":0,"row":4,"kind":0,"name":"EDIT","caption":"Riverlight / Assembly"},
      {"column":1,"row":4,"kind":1,"name":"CAPTIONS","caption":"Say it your way"},
      {"column":2,"row":4,"kind":0,"name":"EDIT","caption":"Riverlight / Assembly"},
      {"column":3,"row":4,"kind":1,"name":"CAPTIONS","caption":"Say it your way"},
      {"column":-3,"row":5,"kind":3,"name":"MOTION","caption":"Give the story momentum"},
      {"column":-2,"row":5,"kind":2,"name":"RECORD","caption":"A clear point of view"},
      {"column":-1,"row":5,"kind":3,"name":"MOTION","caption":"Give the story momentum"},
      {"column":0,"row":5,"kind":2,"name":"RECORD","caption":"A clear point of view"},
      {"column":1,"row":5,"kind":3,"name":"MOTION","caption":"Give the story momentum"},
      {"column":2,"row":5,"kind":2,"name":"RECORD","caption":"A clear point of view"},
      {"column":3,"row":5,"kind":3,"name":"MOTION","caption":"Give the story momentum"},
    ],
  },
  computeds: {
    clock: computed.number(() => time.ms * Math.max(0.25, Math.min(2, vars.speed))),
    fit: computed.number(() => Math.min(vars.width / 960, vars.height / 960)),
    hold: computed.number(() => Math.max(0, vars.holdMs)),
    pullback: computed.number(() => Math.max(500, vars.pullbackMs)),
    tiltStart: computed.number(() => computed.hold),
    tiltDuration: computed.number(() => Math.max(700, vars.tiltMs)),
    pull: computed.number(() => reveal(computed.clock, computed.hold, computed.pullback)),
    scale: computed.number(() => wallScale(computed.clock, computed.hold, computed.pullback)),
    lean: computed.number(() => wallLean(computed.clock, computed.tiltStart, computed.tiltDuration)),
    squash: computed.number(() => isoSquash(computed.lean)),
    rotation: computed.number(() => wallRotation(computed.lean)),
    pan: computed.number(() => wallPan(computed.clock, computed.tiltStart, computed.tiltDuration, Math.max(0, Math.min(120, vars.panSpeed)))),
  },
  render({ vars, computed }) {
    return <Scene width={vars.width} height={vars.height}>
      <Rect id="background" width={vars.width} height={vars.height} fill={vars.background} />
      <Group id="square-stage" x={vars.width / 2} y={vars.height / 2} width={0} height={0}
        pivotX={0} pivotY={0} scaleX={computed.fit} scaleY={computed.fit}>
        <Group id="wall-camera" width={0} height={0} pivotX={0} pivotY={0}
          scaleX={computed.scale} scaleY={computed.scale * computed.squash}
          blur={0.55 * Math.sin(Math.PI * computed.pull)}>
          <Group id="isometric-rotation" width={0} height={0} pivotX={0} pivotY={0} rotation={computed.rotation}>
          <Group id="wall-plane" x={-219 * computed.pull}
            y={-159 * computed.pull - computed.pan} width={0} height={0} pivotX={0} pivotY={0}>
            <Repeat items={tokens.tiles} render={(tile, index) => (
              <Group id={"tool-" + index} x={tile.column * 438 - 210} y={tile.row * 318 - 150}
                width={420} height={300}>
                <Rect id={"shell-" + index} width={420} height={300} radius={12}
                  fill="#1B2538" stroke="#61708B" strokeWidth={0.7} />
                <Image id={"preview-" + index} x={0} y={38} width={420} height={262}
                  src={tile.kind === 0 ? tokens.editArt : tile.kind === 1 ? tokens.captionArt : tile.kind === 2 ? tokens.recordArt : tokens.motionArt} />
                <Rect id={"status-" + index} x={14} y={16} width={6} height={6} radius={3} fill="#C0C9DF" />
                <Text id={"tool-name-" + index} x={30} y={9} width={100} height={22}
                  text={tile.name} fontFamily="Inter" size={11} weight={600} letterSpacing={1.1} color="#F1F3FA" />
                <Text id={"project-name-" + index} x={145} y={9} width={255} height={22}
                  text={tile.caption} fontFamily="Inter" size={10} weight={400} align="right" color="#A8B5CD" />
                <Group id={"caption-copy-" + index} opacity={tile.kind === 1 ? 1 : 0}>
                  <Text id={"caption-a-" + index} text="MAKE ROOM" x={31} y={115} width={350} height={45}
                    fontFamily="Inter" size={35} weight={800} letterSpacing={-1.2} color="#292945" />
                  <Text id={"caption-b-" + index} text="for your story." x={31} y={163} width={350} height={44}
                    fontFamily="Inter" size={32} weight={500} letterSpacing={-1.1} color="#39364C" />
                </Group>
                <Text id={"record-timer-" + index} text="00:12" x={303} y={96} width={95} height={31}
                  opacity={tile.kind === 2 ? 1 : 0} fontFamily="Inter" size={22} weight={500} align="center" color="#4C5478" />
                <Text id={"motion-copy-" + index} text="Ease into motion." x={25} y={57} width={365} height={45}
                  opacity={tile.kind === 3 ? 1 : 0} fontFamily="Inter" size={27} weight={600} letterSpacing={-0.6} color="#E7EDDB" />
              </Group>
            )} />
          </Group>
          </Group>
        </Group>
        <Image id="edge-vignette" src={tokens.vignette} x={-480} y={-480} width={960} height={960} />
      </Group>
    </Scene>;
  },
});
