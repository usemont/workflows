// Standalone animated A/B preview. Use scene.tsx for live timeline host frames.
export default defineSpanScene({
  id: "cube-flip-transition",
  width: 1080,
  height: 1080,
  durationMs: 1000,
  editModeTimeMs: 500,
  background: "transparent",
  render() {
    return (
      <Scene background="transparent">
        <Group id="preview-clipA" textureSource width={1080} height={1080} sceneInput={{width: 1080, height: 1080}}>
          <Rect id="preview-clipA-review-background" x={0} y={0} width={1080} height={1080} fill="#1763CA" />
          <Rect id="preview-clipA-tile-0-0" x={0.0} y={0.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-2-0" x={360.0} y={0.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-4-0" x={720.0} y={0.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-1-1" x={180.0} y={180.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-3-1" x={540.0} y={180.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-5-1" x={900.0} y={180.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-0-2" x={0.0} y={360.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-2-2" x={360.0} y={360.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-4-2" x={720.0} y={360.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-1-3" x={180.0} y={540.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-3-3" x={540.0} y={540.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-5-3" x={900.0} y={540.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-0-4" x={0.0} y={720.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-2-4" x={360.0} y={720.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-4-4" x={720.0} y={720.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-1-5" x={180.0} y={900.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-3-5" x={540.0} y={900.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-5-5" x={900.0} y={900.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-0-6" x={0.0} y={1080.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-2-6" x={360.0} y={1080.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-tile-4-6" x={720.0} y={1080.0} width={180.0} height={180.0} fill="#2978D9" />
          <Rect id="preview-clipA-v-0" x={0.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-1" x={180.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-2" x={360.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-3" x={540.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-4" x={720.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-5" x={900.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-v-6" x={1080.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-0" x={0} y={0.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-1" x={0} y={180.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-2" x={0} y={360.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-3" x={0} y={540.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-4" x={0} y={720.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-5" x={0} y={900.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-h-6" x={0} y={1080.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipA-edge" x={5} y={5} width={1070} height={1070} fill="transparent" stroke="#ffffff" strokeWidth={10} />
          <Text id="preview-clipA-label" text="A" x={270.0} y={270.0} width={540.0} height={540.0} size={410.4} lineHeight={540.0} fontFamily="Archivo" weight={900} color="#ffffff" align="center" />
          <Circle id="preview-clipA-moving-dot" x={140.4} y={820.8} width={64.8} height={64.8} fill="#ffffff">
          <Animate prop="x" from={140.4} to={864.0} start="0ms" duration="2600.0ms" ease="linear" />
          </Circle>
        </Group>
        <Group id="preview-clipB" textureSource width={1080} height={1080} sceneInput={{width: 1080, height: 1080}}>
          <Rect id="preview-clipB-review-background" x={0} y={0} width={1080} height={1080} fill="#E87924" />
          <Rect id="preview-clipB-tile-0-0" x={0.0} y={0.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-2-0" x={360.0} y={0.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-4-0" x={720.0} y={0.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-1-1" x={180.0} y={180.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-3-1" x={540.0} y={180.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-5-1" x={900.0} y={180.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-0-2" x={0.0} y={360.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-2-2" x={360.0} y={360.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-4-2" x={720.0} y={360.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-1-3" x={180.0} y={540.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-3-3" x={540.0} y={540.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-5-3" x={900.0} y={540.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-0-4" x={0.0} y={720.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-2-4" x={360.0} y={720.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-4-4" x={720.0} y={720.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-1-5" x={180.0} y={900.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-3-5" x={540.0} y={900.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-5-5" x={900.0} y={900.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-0-6" x={0.0} y={1080.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-2-6" x={360.0} y={1080.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-tile-4-6" x={720.0} y={1080.0} width={180.0} height={180.0} fill="#F58F37" />
          <Rect id="preview-clipB-v-0" x={0.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-1" x={180.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-2" x={360.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-3" x={540.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-4" x={720.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-5" x={900.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-v-6" x={1080.0} y={0} width={2} height={1080} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-0" x={0} y={0.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-1" x={0} y={180.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-2" x={0} y={360.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-3" x={0} y={540.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-4" x={0} y={720.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-5" x={0} y={900.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-h-6" x={0} y={1080.0} width={1080} height={2} fill="#ffffff" opacity={0.2} />
          <Rect id="preview-clipB-edge" x={5} y={5} width={1070} height={1070} fill="transparent" stroke="#ffffff" strokeWidth={10} />
          <Text id="preview-clipB-label" text="B" x={270.0} y={270.0} width={540.0} height={540.0} size={410.4} lineHeight={540.0} fontFamily="Archivo" weight={900} color="#ffffff" align="center" />
          <Circle id="preview-clipB-moving-dot" x={140.4} y={820.8} width={64.8} height={64.8} fill="#ffffff">
          <Animate prop="x" from={140.4} to={864.0} start="0ms" duration="2600.0ms" ease="linear" />
          </Circle>
        </Group>
        <Rect id="void" x={0} y={0} width={1080} height={1080} fill="#020617" layer={0} />
        <Model3D
          id="cube-turn"
          src="data:model/gltf-binary;base64,Z2xURgIAAAAsEwAAyA8AAEpTT057ImFzc2V0Ijp7InZlcnNpb24iOiIyLjAiLCJnZW5lcmF0b3IiOiJNb250IGFkYXB0aXZlIGN1YmUgdHJhbnNpdGlvbiBnZW5lcmF0b3IifSwic2NlbmUiOjAsInNjZW5lcyI6W3sibm9kZXMiOlswXX1dLCJub2RlcyI6W3sibWVzaCI6MH1dLCJtZXNoZXMiOlt7Im5hbWUiOiJBZGFwdGl2ZUhvc3RGcmFtZUN1YmVUdXJuIiwicHJpbWl0aXZlcyI6W3siYXR0cmlidXRlcyI6eyJQT1NJVElPTiI6MCwiTk9STUFMIjoxLCJURVhDT09SRF8wIjoyfSwiaW5kaWNlcyI6MywibWF0ZXJpYWwiOjAsIm1vZGUiOjR9LHsiYXR0cmlidXRlcyI6eyJQT1NJVElPTiI6NCwiTk9STUFMIjo1LCJURVhDT09SRF8wIjo2fSwiaW5kaWNlcyI6NywibWF0ZXJpYWwiOjEsIm1vZGUiOjR9LHsiYXR0cmlidXRlcyI6eyJQT1NJVElPTiI6OCwiTk9STUFMIjo5LCJURVhDT09SRF8wIjoxMH0sImluZGljZXMiOjExLCJtYXRlcmlhbCI6MSwibW9kZSI6NH0seyJhdHRyaWJ1dGVzIjp7IlBPU0lUSU9OIjoxMiwiTk9STUFMIjoxMywiVEVYQ09PUkRfMCI6MTR9LCJpbmRpY2VzIjoxNSwibWF0ZXJpYWwiOjEsIm1vZGUiOjR9LHsiYXR0cmlidXRlcyI6eyJQT1NJVElPTiI6MTYsIk5PUk1BTCI6MTcsIlRFWENPT1JEXzAiOjE4fSwiaW5kaWNlcyI6MTksIm1hdGVyaWFsIjoxLCJtb2RlIjo0fSx7ImF0dHJpYnV0ZXMiOnsiUE9TSVRJT04iOjIwLCJOT1JNQUwiOjIxLCJURVhDT09SRF8wIjoyMn0sImluZGljZXMiOjIzLCJtYXRlcmlhbCI6MSwibW9kZSI6NH1dfV0sIm1hdGVyaWFscyI6W3sibmFtZSI6ImNsaXAtYS1mYWNlIiwicGJyTWV0YWxsaWNSb3VnaG5lc3MiOnsiYmFzZUNvbG9yRmFjdG9yIjpbMSwxLDEsMV0sIm1ldGFsbGljRmFjdG9yIjowLCJyb3VnaG5lc3NGYWN0b3IiOjF9LCJkb3VibGVTaWRlZCI6dHJ1ZX0seyJuYW1lIjoiY2xpcC1iLWZhY2UiLCJwYnJNZXRhbGxpY1JvdWdobmVzcyI6eyJiYXNlQ29sb3JGYWN0b3IiOlsxLDEsMSwxXSwibWV0YWxsaWNGYWN0b3IiOjAsInJvdWdobmVzc0ZhY3RvciI6MX0sImRvdWJsZVNpZGVkIjp0cnVlfV0sImJ1ZmZlcnMiOlt7ImJ5dGVMZW5ndGgiOjg0MH1dLCJidWZmZXJWaWV3cyI6W3siYnVmZmVyIjowLCJieXRlT2Zmc2V0IjowLCJieXRlTGVuZ3RoIjoxMjgsImJ5dGVTdHJpZGUiOjMyLCJ0YXJnZXQiOjM0OTYyfSx7ImJ1ZmZlciI6MCwiYnl0ZU9mZnNldCI6MTI4LCJieXRlTGVuZ3RoIjoxMiwidGFyZ2V0IjozNDk2M30seyJidWZmZXIiOjAsImJ5dGVPZmZzZXQiOjE0MCwiYnl0ZUxlbmd0aCI6MTI4LCJieXRlU3RyaWRlIjozMiwidGFyZ2V0IjozNDk2Mn0seyJidWZmZXIiOjAsImJ5dGVPZmZzZXQiOjI2OCwiYnl0ZUxlbmd0aCI6MTIsInRhcmdldCI6MzQ5NjN9LHsiYnVmZmVyIjowLCJieXRlT2Zmc2V0IjoyODAsImJ5dGVMZW5ndGgiOjEyOCwiYnl0ZVN0cmlkZSI6MzIsInRhcmdldCI6MzQ5NjJ9LHsiYnVmZmVyIjowLCJieXRlT2Zmc2V0Ijo0MDgsImJ5dGVMZW5ndGgiOjEyLCJ0YXJnZXQiOjM0OTYzfSx7ImJ1ZmZlciI6MCwiYnl0ZU9mZnNldCI6NDIwLCJieXRlTGVuZ3RoIjoxMjgsImJ5dGVTdHJpZGUiOjMyLCJ0YXJnZXQiOjM0OTYyfSx7ImJ1ZmZlciI6MCwiYnl0ZU9mZnNldCI6NTQ4LCJieXRlTGVuZ3RoIjoxMiwidGFyZ2V0IjozNDk2M30seyJidWZmZXIiOjAsImJ5dGVPZmZzZXQiOjU2MCwiYnl0ZUxlbmd0aCI6MTI4LCJieXRlU3RyaWRlIjozMiwidGFyZ2V0IjozNDk2Mn0seyJidWZmZXIiOjAsImJ5dGVPZmZzZXQiOjY4OCwiYnl0ZUxlbmd0aCI6MTIsInRhcmdldCI6MzQ5NjN9LHsiYnVmZmVyIjowLCJieXRlT2Zmc2V0Ijo3MDAsImJ5dGVMZW5ndGgiOjEyOCwiYnl0ZVN0cmlkZSI6MzIsInRhcmdldCI6MzQ5NjJ9LHsiYnVmZmVyIjowLCJieXRlT2Zmc2V0Ijo4MjgsImJ5dGVMZW5ndGgiOjEyLCJ0YXJnZXQiOjM0OTYzfV0sImFjY2Vzc29ycyI6W3siYnVmZmVyVmlldyI6MCwiYnl0ZU9mZnNldCI6MCwiY29tcG9uZW50VHlwZSI6NTEyNiwiY291bnQiOjQsInR5cGUiOiJWRUMzIiwibWluIjpbLTEuMCwtMSwxLjBdLCJtYXgiOlsxLjAsMSwxLjBdfSx7ImJ1ZmZlclZpZXciOjAsImJ5dGVPZmZzZXQiOjEyLCJjb21wb25lbnRUeXBlIjo1MTI2LCJjb3VudCI6NCwidHlwZSI6IlZFQzMifSx7ImJ1ZmZlclZpZXciOjAsImJ5dGVPZmZzZXQiOjI0LCJjb21wb25lbnRUeXBlIjo1MTI2LCJjb3VudCI6NCwidHlwZSI6IlZFQzIifSx7ImJ1ZmZlclZpZXciOjEsImJ5dGVPZmZzZXQiOjAsImNvbXBvbmVudFR5cGUiOjUxMjMsImNvdW50Ijo2LCJ0eXBlIjoiU0NBTEFSIn0seyJidWZmZXJWaWV3IjoyLCJieXRlT2Zmc2V0IjowLCJjb21wb25lbnRUeXBlIjo1MTI2LCJjb3VudCI6NCwidHlwZSI6IlZFQzMiLCJtaW4iOlsxLjAsLTEsLTEuMF0sIm1heCI6WzEuMCwxLDEuMF19LHsiYnVmZmVyVmlldyI6MiwiYnl0ZU9mZnNldCI6MTIsImNvbXBvbmVudFR5cGUiOjUxMjYsImNvdW50Ijo0LCJ0eXBlIjoiVkVDMyJ9LHsiYnVmZmVyVmlldyI6MiwiYnl0ZU9mZnNldCI6MjQsImNvbXBvbmVudFR5cGUiOjUxMjYsImNvdW50Ijo0LCJ0eXBlIjoiVkVDMiJ9LHsiYnVmZmVyVmlldyI6MywiYnl0ZU9mZnNldCI6MCwiY29tcG9uZW50VHlwZSI6NTEyMywiY291bnQiOjYsInR5cGUiOiJTQ0FMQVIifSx7ImJ1ZmZlclZpZXciOjQsImJ5dGVPZmZzZXQiOjAsImNvbXBvbmVudFR5cGUiOjUxMjYsImNvdW50Ijo0LCJ0eXBlIjoiVkVDMyIsIm1pbiI6Wy0xLjAsLTEsLTEuMF0sIm1heCI6WzEuMCwxLC0xLjBdfSx7ImJ1ZmZlclZpZXciOjQsImJ5dGVPZmZzZXQiOjEyLCJjb21wb25lbnRUeXBlIjo1MTI2LCJjb3VudCI6NCwidHlwZSI6IlZFQzMifSx7ImJ1ZmZlclZpZXciOjQsImJ5dGVPZmZzZXQiOjI0LCJjb21wb25lbnRUeXBlIjo1MTI2LCJjb3VudCI6NCwidHlwZSI6IlZFQzIifSx7ImJ1ZmZlclZpZXciOjUsImJ5dGVPZmZzZXQiOjAsImNvbXBvbmVudFR5cGUiOjUxMjMsImNvdW50Ijo2LCJ0eXBlIjoiU0NBTEFSIn0seyJidWZmZXJWaWV3Ijo2LCJieXRlT2Zmc2V0IjowLCJjb21wb25lbnRUeXBlIjo1MTI2LCJjb3VudCI6NCwidHlwZSI6IlZFQzMiLCJtaW4iOlstMS4wLC0xLC0xLjBdLCJtYXgiOlstMS4wLDEsMS4wXX0seyJidWZmZXJWaWV3Ijo2LCJieXRlT2Zmc2V0IjoxMiwiY29tcG9uZW50VHlwZSI6NTEyNiwiY291bnQiOjQsInR5cGUiOiJWRUMzIn0seyJidWZmZXJWaWV3Ijo2LCJieXRlT2Zmc2V0IjoyNCwiY29tcG9uZW50VHlwZSI6NTEyNiwiY291bnQiOjQsInR5cGUiOiJWRUMyIn0seyJidWZmZXJWaWV3Ijo3LCJieXRlT2Zmc2V0IjowLCJjb21wb25lbnRUeXBlIjo1MTIzLCJjb3VudCI6NiwidHlwZSI6IlNDQUxBUiJ9LHsiYnVmZmVyVmlldyI6OCwiYnl0ZU9mZnNldCI6MCwiY29tcG9uZW50VHlwZSI6NTEyNiwiY291bnQiOjQsInR5cGUiOiJWRUMzIiwibWluIjpbLTEuMCwxLC0xLjBdLCJtYXgiOlsxLjAsMSwxLjBdfSx7ImJ1ZmZlclZpZXciOjgsImJ5dGVPZmZzZXQiOjEyLCJjb21wb25lbnRUeXBlIjo1MTI2LCJjb3VudCI6NCwidHlwZSI6IlZFQzMifSx7ImJ1ZmZlclZpZXciOjgsImJ5dGVPZmZzZXQiOjI0LCJjb21wb25lbnRUeXBlIjo1MTI2LCJjb3VudCI6NCwidHlwZSI6IlZFQzIifSx7ImJ1ZmZlclZpZXciOjksImJ5dGVPZmZzZXQiOjAsImNvbXBvbmVudFR5cGUiOjUxMjMsImNvdW50Ijo2LCJ0eXBlIjoiU0NBTEFSIn0seyJidWZmZXJWaWV3IjoxMCwiYnl0ZU9mZnNldCI6MCwiY29tcG9uZW50VHlwZSI6NTEyNiwiY291bnQiOjQsInR5cGUiOiJWRUMzIiwibWluIjpbLTEuMCwtMSwtMS4wXSwibWF4IjpbMS4wLC0xLDEuMF19LHsiYnVmZmVyVmlldyI6MTAsImJ5dGVPZmZzZXQiOjEyLCJjb21wb25lbnRUeXBlIjo1MTI2LCJjb3VudCI6NCwidHlwZSI6IlZFQzMifSx7ImJ1ZmZlclZpZXciOjEwLCJieXRlT2Zmc2V0IjoyNCwiY29tcG9uZW50VHlwZSI6NTEyNiwiY291bnQiOjQsInR5cGUiOiJWRUMyIn0seyJidWZmZXJWaWV3IjoxMSwiYnl0ZU9mZnNldCI6MCwiY29tcG9uZW50VHlwZSI6NTEyMywiY291bnQiOjYsInR5cGUiOiJTQ0FMQVIifV19IEgDAABCSU4AAACAvwAAgL8AAIA/AAAAAAAAAAAAAIA/AAAAAAAAgD8AAIA/AACAvwAAgD8AAAAAAAAAAAAAgD8AAIA/AACAPwAAgD8AAIA/AACAPwAAAAAAAAAAAACAPwAAgD8AAAAAAACAvwAAgD8AAIA/AAAAAAAAAAAAAIA/AAAAAAAAAAAAAAEAAgAAAAIAAwAAAIA/AACAvwAAgD8AAIA/AAAAAAAAAAAAAAAAAACAPwAAgD8AAIC/AACAvwAAgD8AAAAAAAAAAAAAgD8AAIA/AACAPwAAgD8AAIC/AACAPwAAAAAAAAAAAACAPwAAAAAAAIA/AACAPwAAgD8AAIA/AAAAAAAAAAAAAAAAAAAAAAAAAQACAAAAAgADAAAAgD8AAIC/AACAvwAAAAAAAAAAAACAvwAAAAAAAIA/AACAvwAAgL8AAIC/AAAAAAAAAAAAAIC/AACAPwAAgD8AAIC/AACAPwAAgL8AAAAAAAAAAAAAgL8AAIA/AAAAAAAAgD8AAIA/AACAvwAAAAAAAAAAAACAvwAAAAAAAAAAAAABAAIAAAACAAMAAACAvwAAgL8AAIC/AACAvwAAAAAAAAAAAAAAAAAAgD8AAIC/AACAvwAAgD8AAIC/AAAAAAAAAAAAAIA/AACAPwAAgL8AAIA/AACAPwAAgL8AAAAAAAAAAAAAgD8AAAAAAACAvwAAgD8AAIC/AACAvwAAAAAAAAAAAAAAAAAAAAAAAAEAAgAAAAIAAwAAAIC/AACAPwAAgD8AAAAAAACAPwAAAAAAAAAAAACAPwAAgD8AAIA/AACAPwAAAAAAAIA/AAAAAAAAgD8AAIA/AACAPwAAgD8AAIC/AAAAAAAAgD8AAAAAAACAPwAAAAAAAIC/AACAPwAAgL8AAAAAAACAPwAAAAAAAAAAAAAAAAAAAQACAAAAAgADAAAAgL8AAIC/AACAvwAAAAAAAIC/AAAAAAAAAAAAAIA/AACAPwAAgL8AAIC/AAAAAAAAgL8AAAAAAACAPwAAgD8AAIA/AACAvwAAgD8AAAAAAACAvwAAAAAAAIA/AAAAAAAAgL8AAIC/AACAPwAAAAAAAIC/AAAAAAAAAAAAAAAAAAABAAIAAAACAAMA"
          x={0}
          y={0}
          width={1080}
          height={1080}
          cameraDistance={2.4}
          cameraFov={48}
          pitch={0}
          yaw={0}
          assetScale={1.2806145548}
          layer={4}
          textureBindings={[
            { textureSourceId: "preview-clipA", materialIndex: 0, textureFlipY: false, textureUseAlpha: true },
            { textureSourceId: "preview-clipB", materialIndex: 1, textureFlipY: false, textureUseAlpha: true }
          ]}
        >
          <Animate prop="yaw" from={0} to={-90} start="33ms" duration="934ms" ease="inOutCubic" />
        </Model3D>
        <Image id="outgoing-continuity-guard" src="texture-source://preview-clipA" x={0} y={0} width={1080} height={1080} fit="cover" opacity={1} layer={8}>
          <Animate prop="opacity" from={1} to={0} start="0ms" duration="33ms" ease="linear" />
        </Image>
        <Image id="incoming-continuity-guard" src="texture-source://preview-clipB" x={0} y={0} width={1080} height={1080} fit="cover" opacity={0} layer={9}>
          <Animate prop="opacity" from={0} to={1} start="967ms" duration="33ms" ease="linear" />
        </Image>
      </Scene>
    );
  }
});
