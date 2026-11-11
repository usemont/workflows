# Orbit carousel

Ten original posters orbit a tipped sphere using golden-angle spacing and depth-aware layering. The front-center **Soft Space** poster expands for a featured hold, returns to its orbit, then motion resumes.

**1920 × 1080 · seamless 14-second loop · 30 fps recommended.** Portable Mont scene primitives; embedded images, no browser or network dependency during rendering.

```sh
npx usemont templates preview orbit-carousel
```

## Edit

- `poster01`–`poster10`: image sources in collection order; defaults are embedded 600 × 800 JPEGs. Use 3:4 artwork to preserve the composition.
- `background`: canvas color.
- `width`, `height`: output dimensions; the composition scales uniformly and stays centered.
- `speed`: playback-rate multiplier for the entire choreography. Default `1`; use a positive value. For a seamless export at another speed, set the span duration to `14000 / speed` milliseconds, or an integer multiple.
- `carouselTravel`: integrated smoothstep velocity controls orbital travel, deceleration and acceleration.
- `carouselFeature`: quintic easing controls expansion and return. `orbitFeature` selects the featured poster (zero-based index `4`).

At default speed, travel decelerates at 3–3.8 s, settles until 4 s, expands at 4–4.9 s, holds until 6.7 s, shrinks at 6.7–7.5 s, settles until 7.75 s and accelerates until 8.65 s. The orbit stays stationary throughout the feature; the poster returns exactly before travel resumes. Position and cruise velocity match across the loop.

All ten artworks were generated for this collection with `gemini-3.1-flash-image`; see [original images and provenance](../../../assets/template-media/posters/provenance.json). The three carousel templates share the same image set.

Run the collection's motion and geometry checks from the repository root:

```sh
node templates/carousels/verify.mjs
```
