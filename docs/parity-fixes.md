# MaterialCraft parity fixes

Rolling record of fixes made while working through the [SBS root-node inventory](sbs-root-nodes.md).
Parity means exact pixel or typed-value agreement for recorded cases; pending validation is stated explicitly.
The first pass classifies all 546 roots: 166 Parity and 380 Too hard. These measured cases do not establish complete functional parity.

- **2026-11-13 — SBS swizzle defaults (orientation corrected):** Removed the importer's zero-channel overrides for omitted two-, three-, and four-component float/integer swizzle settings. Imports now use the existing identity-channel defaults from the function catalog. The first affected node, `3d_gradient.sbs/3d_linear_gradient`, previously repeated X instead of preserving XYZ, producing a horizontal gradient instead of the reference diagonal gradient. Compared both point-position modes with generated position maps at 256 and 512 pixels. Remaining float differences are recorded below; full node parity is not claimed.
- **2026-11-13 — Filtered function sampling (integer probes exact):** Added cooked-image storage precision to function samplers. Bilinear reads use eight fractional bits for filter weights. Normalized integer textures return 16-bit filtered values (including 8-bit sources); floating-point textures retain floating-point values. Input bridges retain the integer/floating-point distinction, and cache keys include it. Direct reference measurements on a generated position ramp showed a 16-bit half-step rounding to the next integer, explaining the remaining UV-gradient offset after the swizzle fix. All six measured integer-source sampler cases match all 65,536 float pixels exactly. Two of three floating-source cases are exact; one differs by 5.96e-8. Full-gradient rounding differences remain below.

- **2026-11-13 - SBS format inheritance (measured improvement):** Read and save the declared native graph output format instead of dropping it. An omitted format retains inheritance; comparison cases explicitly align the parent format to the reference preview (eight bits). Pixel Processors with omitted format settings now inherit the parent format instead of an image input format. This was exposed by `3d_perlin_noise.sbs/3D_perlin_noise`: the reference processor produces eight-bit output even though its position and lookup inputs are sixteen-bit. The Perlin comparison improved from 63,461/65,536 to 65,533/65,536 matching 8-bit pixels at 256 pixels; three additional 128-pixel cases match exactly. All four native format codes are serialized explicitly on save.

- **2026-11-13 - Floored floating-point modulo (randomized jump flood exact):** Replaced signed remainder with `a - b * floor(a / b)`. Direct reference probes confirm all sign combinations, including `-1 mod 4 = 3`, `1 mod -4 = -3`, and `-5.5 mod 4 = 2.5`. This corrects negative coordinate wrapping used by 3D texture jump flood. Both randomized 128x128 jump-flood cases (iterations 0 and 1, with disabled seed points) now match all 32,768 RGBA pixels exactly; the three ramp cases also match. All 131,072 RGBA pixels across five cases are exact.

- **2026-11-13 - Bound input storage and resizing:** Bound image inputs now store their declared output precision instead of passing unconverted floats downstream. Resizing clamps resource edges and proceeds through factor-two stored levels. A generated HDR 16x8 ramp resized to 32x32 now matches every RGBA8 input-bridge pixel; previously its center sample was 1.0 instead of 0.9843137. A randomized 64x32-to-32x32 downsample also matches every pixel. Three harder randomized multilevel cases retain single-level differences (4/1,024, 51/4,096, and 6/256 pixels), so general resampling parity is not claimed.

- **2026-11-13 - Half-float output storage:** Sixteen-bit floating-point outputs now convert to half precision, truncate toward zero, and saturate at +/-65,504. Previously they retained full 32-bit precision. All 2,304 pixels across nine direct reference probes match exactly, covering positive/negative values, rounding boundaries, a subnormal, maximum finite values, and positive/negative overflow. Uses the existing workspace half dependency (MIT OR Apache-2.0).

- **2026-11-13 - Volume-light function expansion:** Raised the bounded expanded-function allowance from 65,536 to 131,072 nodes after measuring the legitimate volume-light processor at 72,973 nodes and four call levels. The individual 4,096-node graph limit, 16-level call limit, cycle checks, and execution budget remain enforced. Error messages now report the actual limits. Three previously rejected light-render cases cook successfully; 760/768 RGBA16F pixels match, with the remaining differences recorded below.

- **2026-11-13 - Omitted function sampler settings:** Corrected Sample Color/Gray defaults to image zero with nearest filtering. Explicit bilinear settings are retained. Both original omitted-setting probes match all 32,768 float pixels. This removes the shifted edges of the 3D volume mask: default outputs at 128x128 and 256x256 and the changed-scale 128x128 case now match every sixteen-bit pixel.

- **2026-11-13 - Finite results after undefined pixel intermediates:** Pixel Processor arithmetic and vector operations now retain IEEE intermediate values so a later min/max can clamp division by zero to a finite result. Non-finite final pixels still return an actionable error. Parameter and pattern functions are checked at their public evaluation boundary. Four original clamp probes (positive infinity, negative infinity, NaN, and finite division) match all 1,024 float pixels. Box-mask cases previously failed at the exact sampled center; they now cook successfully, with the residual conversion differences recorded below.

- **2026-11-13 - Independent transform wrapping:** Transformation 2D now uses horizontal and vertical tiling flags independently, with matching per-axis image sampling. Previously a horizontal-only panorama lost horizontal wrapping because the two flags collapsed to one boolean. All 131,072 RGBA8 pixels across eight original transform probes match exactly, covering both one-axis modes, positive/negative offsets, matte edges, and varying alpha. The panorama helper improves, but its remaining minification/filtering differences are recorded below.

- **2026-11-13 - Blend alpha, copy, and HDR:** Corrected the Switch stages used by Copy and other graphs to interpolate RGBA without weighting foreground alpha twice. Copy, Add, and other color modes now combine alpha coverage when alpha blending is enabled; disabling alpha blending preserves background alpha while RGB still uses foreground coverage. Integer clipping happens after opacity; floating-point formats retain HDR output. All 19 independent blend graphs are being measured with opaque/varying-alpha inputs, masks, opacity changes, and alpha-channel blending disabled. Direct Add/Copy HDR cases match all 512 RGBA32F pixels. Fourteen blend graphs now match every pixel across four RGBA8 cases each (1,605,632 pixels); Add and Copy also pass their HDR cases. The blend properties label and all four alpha-interpretation menu entries were checked visually in the running native app.

- **2026-11-13 - Function validation at result boundaries:** Original clamp probes confirm that node-parameter and pattern functions also accept undefined intermediates when later operations produce a finite result. Deferred those intermediate checks and retained explicit final-result validation. The original four node-parameter and four pattern probes match all 2,048 output pixels; unclamped pixel and node-parameter probes still return clear non-finite-output errors. Ordered NaN comparisons now return false, as measured for all four relational operators. All 5,120 float pixels across the twenty NaN/infinity/finite/zero comparison cases match exactly.

- **2026-11-13 - Grayscale default precision:** Default grayscale rendering now uses one third per RGB channel instead of 0.333, which previously darkened equal channels. This makes Add Sub exact in all four RGBA8 cases and reduces Luminosity from 8,407 mismatching pixels to two. Original float probes establish the fused red/green/blue/alpha dot-product order; all 65,536 float pixels across default, custom, sum-three, and zero-weight cases match exactly. Explicit channel weights retain their authored values.

- **2026-11-13 - Declared image types:** Imported single-output filters with an omitted color-mode parameter now retain the color/grayscale type declared in their SBS output. The bevel debug helper previously dropped RGBA coordinate data into a grayscale pattern buffer. It now cooks in the correct four-channel format. Explicit settings and functions remain authoritative.

- **2026-11-13 - Pattern alpha and empty backgrounds:** Color pattern maps now start transparent. Scalar luminosity/opacity affects alpha as well as RGB, without multiplying RGB twice. Eight original empty/scalar/RGBA/HDR pattern probes match all 2,048 RGBA32F pixels across additive and both maximum blend modes. The existing White Noise result still matches every one of its 65,536 sixteen-bit pixels.

- **2026-11-13 - Color resources on grayscale input pins:** Bound color images now use equal RGB weights at grayscale input bridges. The general display-luminance utility retains its documented behavior. All nine eight-bit grayscale switch cases match 294,912 pixels; two additional float switch cases match all 32,768 pixels exactly, including varying source alpha. Previously only about one percent of the eight-bit pixels matched.

- **2026-11-13 - Transform matrix layout:** Transformation 2D now reads consecutive columns in the stored matrix and uses the same convention when selecting nearest mip levels. Previously rotations ran in the opposite direction and shears used the wrong axis. All 131,072 RGBA8 pixels across eight original shear/quarter-turn/diagonal/offset probes match exactly. Both unscaled rotated-checker cases also become exact. Scaled filtering remains open.

- **2026-11-13 - Legacy blend alpha interpretation:** Imported and implemented the older source-alpha/ignore-alpha/straight/premultiplied setting instead of dropping it. Direct runtime enum inspection confirms the four modes. Original Copy/Add/Max probes cover opacity changes, alpha-channel blending disabled, and 128x128/256x256 outputs. Ignore-alpha controls RGB weighting independently of output coverage; straight compositing includes background coverage, while premultiplied Copy consumes premultiplied foreground RGB. All four Clamp cases now match every one of 114,688 RGBA8 pixels. All 48 direct Copy/Add/Max cases cook; seven of the twelve mode/interpretation combinations match all 114,688 pixels, while the other five retain 11 one-level differences in total. Straight HDR output remains unclipped; its float differences reach 3.577e-7.

- **2026-11-13 - Integer options read as Boolean:** A legacy color-temperature function reads an integer option through Get Boolean. Original probes confirm that zero is false and positive/negative integers, including the minimum 32-bit value, are true. Added that conversion for user integer/enum variables while retaining strict system-variable and other getter types. All 1,280 float pixels across the five direct cases match exactly. Color temperature now cooks in all four cases; only two one-level pixels remain different.

- **2026-11-13 - Internal images down to one texel:** Removed the evaluator's 16-pixel floor for internal node sizes. The graph editor retains its own graph-size setting range. Internal reductions and lookup images can now use their declared 1..8,192 dimensions. Direct 1x1, 2x4, and 8x2 RGBA32F probes match all 25 pixels. The curvature helper's four small cases now use the correct 2x2/4x4 outputs and match all 28 pixels; two larger nonconstant cases also match all 16,640 RGBA32F pixels. All 655,360 pixels across 22 earlier Perlin-fractal, simplex, jump-flood, angle-vector, checker, and live White Noise cases still match exactly.

- **2026-11-13 - Integer Pixel Processor storage:** Original eight- and sixteen-bit probes confirm NaN converts to black, positive infinity to white, and negative infinity to black. Deferred Pixel Processor finite checks to integer storage for those formats, retaining final floating-point output validation. Integer storage now saturates infinities instead of treating every non-finite value as zero. All 2,560 pixels across ten direct integer probes match exactly. The zero-softness color mask and radial gradient now cook in all four cases each, with only one-level residual differences. Unclamped floating-point Pixel Processors still return a non-finite-output error.

- **2026-11-13 - Swizzle padding:** Vector swizzles now read components outside the source dimension as zero, including scalar inputs. Arithmetic scalar broadcasting remains available to arithmetic operations. All 1,792 RGBA32F pixels across seven original float/integer probes match exactly. Histogram computation now cooks in all four cases; its remaining numerical differences are recorded below.

- **2026-11-13 - Grayscale resources on color input pins:** Color input bridges now promote grayscale resources to RGB with opaque alpha. Both original 128x128 and 256x256 probes match all 81,920 RGBA8 pixels exactly.

- **2026-11-13 - Unassigned user variables:** Numeric user-variable getters now return typed zero, and Boolean getters return false, before the first assignment. Previously they raised an undefined-variable error. Original probes add a nonzero value after each getter so a failed black render cannot look like a match. All 2,304 float/RGBA32F pixels across Boolean, integer1..4, and float1..4 cases match exactly. System-variable availability, existing-variable type checks, and string errors remain strict. All four affected Voronoi graphs now cook across four 16x16/32x32 cases each; numerical differences remain. The 128x128 fractal case also cooks and matches 15,457/16,384 pixels. The previously failing matte and shadow helper now cook in all four cases each, with remaining output differences.

- **2026-11-13 - RGBA results in grayscale Pixel Processors:** Grayscale processors now convert a four-component result with equal RGB weights instead of taking red alone. Alpha does not contribute. Original HDR and negative-value probes establish the fused green/red/blue accumulation order; all 9,216 LUM32F pixels across 36 cases match exactly. The three affected grayscale rendering templates now match all 344,064 LUM8 pixels across twelve cases; the color template retains all 114,688 exact RGBA8 pixels.

- **2026-11-13 - Float-to-integer conversion:** Finite scalar and vector conversion now rounds down, correcting negative coordinates that previously truncated toward zero. All 6,144 float/RGBA32F pixels and 18 typed integer outputs match the original reference probes. Non-finite and out-of-range integer conversions retain actionable errors; parity for those inputs is not claimed. The 22 previously verified cases still match all 655,360 pixels after this change.

- **2026-11-13 - Unconnected output images:** Unconnected output bridges now produce transparent RGBA instead of grayscale. The importer separately retains each output's declared image type so saving an unconnected output preserves the graph interface. Both material templates match all 1,179,648 RGBA8 pixels across six recorded cases. All 2,560 direct grayscale/color declaration probe pixels also match, and both declared image types survive save/reload.

## 2026-11-18 root comparisons

### Pixel Processor products and sums

Use the signed RGBA32F field and position ramp specified under [3D linear gradient](#3d-linear-gradient). Each arithmetic probe has a nearest-sampled RGBA input, seed zero and a 128×128 float output. Compare the raw output in the 16.0.7 Direct3D 11 reference app with the MaterialCraft CPU cook. A pixel counts when all channels match. Integer errors in this log use stored levels. Floating errors use absolute differences.

| Pixel expression | Exact pixels / total | Maximum error |
|---|---:|---:|
| `R*G+B`, `B+R*G`, `R*G-B`, `B-R*G` | 16,384/16,384 each | 0 |
| `R*G+B*A`, `R*G-B*A` | 16,384/16,384 each | 0 |
| Runtime product plus .375, runtime channel times .3 minus B, literal product plus R | 16,384/16,384 each | 0 |
| Shared product read directly and in a sum, shared product used only in sums, vector times scalar plus vector | 16,384/16,384 each | 0 |

Rounding each multiplication separately matched 11,640 additions, 11,721 left subtractions, 12,098 two-product additions and 12,052 two-product subtractions out of 16,384 each. The largest difference was 1.907349e-6. The CPU evaluator now fuses the dynamic product into its immediate sum in pixel scope. A literal product or a product also read as a standalone value retains its rounded value. When both operands are products, the left product fuses.

Repeat the November 17 **3D linear gradient** cases with that change. At 256² and 512², the default UV, default world and oblique world gradients each match 65,536/65,536 and 262,144/262,144 LUM32F pixels, maximum error zero. Seed 42 at 128² matches all 16,384 default UV pixels and all 16,384 oblique pixels in each of LUM8, LUM16 and LUM16F. Contrast .6 still differs. Exact counts are 24,206/65,536 and 96,824/262,144 with a maximum error of 2.384186e-7. For the signed input above, a direct `1/G` probe matches 14,182/16,384 float pixels. All finite differences in that probe are one unit in the last place.

With the original shared lookup described under [basic and ridged 3D Perlin](#basic-and-ridged-3d-perlin), **basic 3D Perlin** now matches 65,536/65,536 pixels at 256² and 16,384/16,384 in each of the other six cases, including changed LUM16, LUM16F and LUM32F. All maximum errors are zero. With the applications' respective stock lookups, only 173/16,384 and 663/65,536 LUM8 pixels match. Both maxima are 203 levels.

### Runtime reciprocal and 3D gradient contrast

The twelve wired 3D linear gradient cases listed under [3D linear gradient](#3d-linear-gradient) now match **1,376,256/1,376,256 pixels**, maximum error zero. This includes all 65,536 and 262,144 LUM32F pixels at contrast .6 and all four measured output formats. An exposed contrast of .6 gives the float32 denominator `0x3ecccccc`. Its runtime reciprocal is 2.5. With a literal expression, the reciprocal is the next float above 2.5.

For a standalone Pixel Processor, nearest-sample a linear RGBA32F input and output the reciprocal of one channel at its original dimensions. Five fields match **671,744/671,744 LUM32F pixels**, maximum error zero. Use seed zero. For the 128² case take G from the signed field under [3D linear gradient](#3d-linear-gradient). Construct the remaining four fields from float32 bits at row-major pixel index `i`, repeated in RGB.

| Dimensions | Input bits | Exact pixels / total |
|---|---|---:|
| 256² | `0x3f800000 + 128*i` | 65,536/65,536 |
| 256² | `(125 << 23) + floor(3*2^23/5) - 32768 + i` | 65,536/65,536 |
| 512² | Use `h=(747796405*i+2891336453) mod 2^32` as below | 262,144/262,144 |
| 512² | Use `h=(1013904223*i+341937) mod 2^32` as below | 262,144/262,144 |

For the 512² fields, use the low 23 bits of `h` as the fraction, `7 + (37*((h >> 22) XOR i)) mod 241` as the biased exponent, and bit 2 of `i` as the sign. MaterialCraft derives a quadratic for each of 128 mantissa intervals from the mathematical reciprocal. The measured fixed-point square retains partial products through fractional bit 15. The literal division path retains its original rounding. A separate 32² RGBA32F probe emits contrast, denominator, reciprocal and half the reciprocal minus one half. All nine runs match 1,024/1,024 pixels each, including exposed values .3, .35, .51, .6, .625, .875 and .99, an image input and a literal.

With Position disconnected, both apps return zero in U8 and U16 and quiet NaN in floating storage. At 32² with seed zero, the 1,024 raw samples are respectively `0x00`, `0x0000`, `0x7fff` and `0x7fffffff`. At 64² with seed 42 the F32 result has the same NaN bits at all 4,096 pixels. The five disconnected cases match **8,192/8,192 bit patterns**. Maximum finite error is zero for the two integer runs. The float runs contain only NaNs, for which a numerical maximum error is undefined. Across the 12 wired and five disconnected cases the root matches **1,384,448/1,384,448 pixels**.

Four independent 32² image functions distinguish output storage from scalar parameter validation. Bind an RGBA32F field with `R=(x mod 7)-3`, `G=1` on every fourth row starting at zero and `G=0` elsewhere, `B=0` and `A=1`. Output, in turn, B/B, A/B, R/B and R/G. In U8, U16, F16 and F32 each run matches 1,024/1,024 stored samples, **16,384/16,384** overall. The mixed R/G float images each contain 256 finite values, 120 quiet NaNs, 288 positive infinities and 360 negative infinities. Every special value matches bit for bit and the maximum finite error is zero. Integer storage maps NaNs and negative infinities to zero and positive infinities to white.

### Resizing images bound to graph inputs

Bind a 128² linear image directly to a graph input, set the input bridge to the indicated storage format, and nearest-sample its output at pixel centers into RGBA32F. The CPU cook and the reference app match **4,534,272/4,534,272 pixels across 72 cases**, maximum component error zero. All four channels count toward each pixel.

| Source | Bridge | Output sides | Cases | Exact pixels / total |
|---|---|---|---:|---:|
| Original RGBA8 atlas field | Color in U8, U16, F16 and F32 | 64, 128, 256 | 12 | 344,064/344,064 |
| Original RGBA16 and signed RGBA32F fields | Color in U8, U16, F16 and F32 | 32, 64, 128, 256, 512 | 40 | 2,793,472/2,793,472 |
| Same RGBA16 and signed RGBA32F fields | Grayscale in U16 and F32 | 32, 64, 128, 256, 512 | 20 | 1,396,736/1,396,736 |

Use the RGBA8 formula under [atlas crop with wired value inputs](#atlas-crop-with-wired-value-inputs) and the signed RGBA32F field under [3D linear gradient](#3d-linear-gradient). For the RGBA16 field, map each signed input component `v` to the nearest integer to `65535*(v+4)/8`. Use seed 42 below 128² and zero otherwise. The grayscale bridge result was also read directly before expansion to RGBA.

Each factor-two step filters four clamped texels at the source storage precision, then stores the result in the bridge format. For grayscale, the first step filters the three color channels before computing the equal-weight mean. Further steps read stored grayscale. Before the fix, the 256² RGBA8 source case with an F32 bridge matched 4/65,536 pixels, maximum error 7.629395e-6. Filtering the same RGBA16 field after conversion to gray matched only 53,819/65,536 gray U16 pixels at 256².

The 256² atlas wrapper now matches 65,281/65,536 at default rotation, maximum .250003815. Its 255 differing pixels lie on the first row and column of the lower-right quadrant. Rotation .125 matches 65,089/65,536, maximum 1. The directly bound controls above and this wrapper exercise different graph connections.

With the inputs and settings under [panoramic LOD](#panoramic-lod), the four 64² seed-42 format runs now match 4,096/4,096 each, maximum zero. The 128² opaque, alpha and changed-seed outputs each match 16,128/16,384, maximum 125 byte levels. At 256² the opaque image matches 64,768/65,536, maximum 126. Both 128² flat controls match 16,128/16,384, maximum 116.

The rotated 3D planar projection runs still differ. At 128² the grayscale output matches 16,335/16,384 in LUM16, maximum 226 levels, and color matches 16,378/16,384 in RGBA8, maximum one level. At 64² both roots match 4,094/4,096 in U8 and 4,093/4,096 in each of U16, F16 and F32. The respective maxima are 1, 32, .00048828125 and .000488288701. All three default and changed-seed runs for each root match 98,304/98,304 pixels. With the [shared lookup](#basic-and-ridged-3d-perlin), ridged 3D Perlin matches all four eight-bit cases. Its changed U16, F16 and F32 runs match 16,337, 16,380 and 5,160 of 16,384 pixels, with maxima 1, .00048828125 and 1.78813935e-7.

### Wired graph textures and pixel sampling

An original two-graph package returns two outputs from the child. One child output returns its color input directly, the other reads it using a Pixel Processor. The parent supplies the 128² RGBA8 atlas field above at a fixed 128². Give the child input a parent-relative size and set its precision explicitly. Read the stored bridge from the direct output and the sampled value from an RGBA32F output. Each pixel comparison includes all four components and checks the reported dimensions and depth.

| Controls | Output comparisons | Exact pixels / total | Maximum error |
|---|---:|---:|---:|
| Nearest and bilinear at pixel centers, U8/U16/F16/F32 bridge, sides 64/128/256, seed zero | 48 | 1,376,256/1,376,256 | 0 |
| Bilinear at 32² seed 42 and 512² seed zero, same four bridge formats | 16 | 2,105,344/2,105,344 | 0 |
| At 256² and F32, change the tiling on the parent input, child input or processor to none, horizontal or vertical | 18 | 1,179,648/1,179,648 | 0 |

The directly returned bridge resizes through stored factor-two levels using its own horizontal and vertical tiling flags. The sampled color input retains the source dimensions and precision inside the graph instance. For example, at 64² the first red value is 48/255 when sampled nearest and 24/255 through the direct U16 bridge. At 256², both axes wrap for the bilinear sampler. The first atlas texel receives a green contribution of 16,384/65,535 from the opposite edge.

For a separate single-graph control, nearest- or bilinear-sample the same fixed 128² input at `$pos + (−.3125,.1875)`. Set the processor output to RGBA32F, side 256 and seed 42. Repeat with its tiling set to none, horizontal, vertical and both. All **524,288/524,288** pixels match, maximum zero. The explicit image read repeats both axes in all eight cases.

In the [atlas crop wrapper](#atlas-crop-with-wired-value-inputs), default rotation at 256² now matches **65,536/65,536** RGBA32F pixels, maximum zero. With .125 turns the 256² case matches 65,408/65,536, maximum .994125247. At 128² it matches 16,320/16,384 with seed zero or 42, maximum 1. At 64², 4,064/4,096 match, maximum .996078432 in the authored float output. The remaining rotation differences follow one edge of the sampled shape.

For the [spline distance controls](#height-distance-texture-mapping-skew-and-weave), both wired counts now match 1,024/1,024 at 32² seed 42 and 4,096/4,096 at 64² seed 137 in LUM32F. At 16² seed zero, count 1 matches 256/256 and count 2 matches 252/256, maximum 5.96046448e-8.

### Value Processor coordinates

Both maximum-coordinate roots return the same Float2 as the reference in **12/12 typed comparisons each**. All maximum component errors are zero. Before the fix, reading the position variable stopped both cooks. A Value Processor starts at (0,0) and passes this position to nested function calls.

| Root | Defaults, changed input and seed | Four requested output formats | Four isolated peaks |
|---|---:|---:|---:|
| `pbr_render.sbs/get_max_coords/get_max_coord` | 4/4 | 4/4 | 4/4 |
| `pbr_render2.sbs/get_max_coords/get_max_coord` | 4/4 | 4/4 | 4/4 |

Use the byte field (17x+31y) mod 256 at 128² and 256² with seed zero, and at 128² with seed 42. The changed field is 32+((17x+31y) mod 160) at 128² with seed zero. At 64² seed 42, requesting U8, U16, F16 and F32 at the upstream processor yields the Float2 (.1015625,.109375) in all four cases. These roots expose no ordinary parameters. For the original single-peak field use (11x+13y+3xy) mod 79, and set the specified pixel to 255.

| Size | Seed | Peak pixel | Typed result in both roots |
|---:|---:|---|---|
| 16² | 7 | (1,2) | (.09375,.15625) |
| 32² | 91 | (21,7) | (.6640625,.234375) |
| 64² | 113 | (57,48) | (.890625,.75) |
| 128² | 251 | (122,17) | (.953125,.125) |

Separate original value graphs read `$pos`, `$size`, `$sizelog2`, `$depth`, `$depthpow2` and `$number`. At 16² and 128² with seed zero and 64² with seed 42, the position and three depth or iteration values are zero. The size values follow the canvas and its base-two logarithm. Is Defined reports true for the position, size and three depth or iteration names and false for an unbound probe name. Unbound numeric and Boolean reads return a typed zero. These scalar and vector controls match **50/50 typed values** with maximum error zero.

A Value Processor also averages a sampled color’s three channels in the order measured under [grayscale sampling](#grayscale-sampling-of-color-inputs). Using the same four RGBA fields, sample at (39.5+.1875,77.5+.5625)/128. Repeat with a uniform (.25,.5,.75,1). Compare the single Float or Float4 as IEEE float32 components in nearest and bilinear modes. Before correcting normalization and filtering, 23/32 values matched across the two input fields, with maximum component error 5.84125519e-06.

### Value Processor texture filtering

The 32 scalar and color comparisons above now match **32/32 typed values**. Two independent 128² coordinate sets add **192/192 Float4 values**. Two more sets at 2² and 32² give **16/16 Float4 values**. Maximum component error is zero across all 240 comparisons. Both maximum-coordinate roots still match all 24 previously measured Float2 results.

A value function wraps both texture axes even when output tiling is disabled. For bilinear sampling, multiply the UV by the stored dimensions, subtract .5 and keep the full float32 fractional texel position. In each row compute a×(1−t)+b×t with two rounded products and a sum, then do the same vertically. For U8 or U16, interpolate stored integer channel levels as float32 and finally multiply by the float32 reciprocal of 255 or 65,535. A nearest read normalizes the selected stored level with the same reciprocal. Floating textures already carry their sample values.

Use the four original color fields described under [grayscale sampling](#grayscale-sampling-of-color-inputs), both filtering modes and output tiling disabled. For the first set choose (x,y,a,b) from (7,13,0,0), (83,127,48,144), (42,78,19,61), (17,93,128,128), (0,62,−93,181), (126,7,201,113), (77,0,−171,−35), (128,67,47,23), (−2,132,105,151), (31,40,1,255), (4,1,77,99), (76,30,45,85). The UV is (x+.5+a/256,y+.5+b/256)/128. For the second set use (((227j mod 1447)−153)/1009, ((281j mod 1867)−413)/1031) for j=0…11. In both sets alternate seeds 0 and 42.

For the small independent field, take row-major RGBA byte tuples (19,67,129,251), (85,222,17,9), (255,128,74,66) and (3,102,39,109). For U16, multiply each level by 251. For F16 and F32, use (level−97.125)/13.375. Sample at (−.317,1.093), seed 17, directly from a 2² bridge and again after the bridge resizes the field to 32². With the 2² byte field, the four bilinear component bit patterns are `3e8013a0 3f2bd646 3e0f43a5 3e744cb7`.

### Luminosity, axial gradients, color lookup and luminance merge

The five roots below each match **165,120/165,120 pixels in 15 cases**. Replaying the four first-pass captures gives 114,688 exact pixels per root. Four 64² producer formats at seed 42 give 16,384, and a separate 32² F32 case at seed 113 gives 1,024. Six further original cases give 33,024. Their sizes are 16² F32 at seed 17, 64² U8/U16/F16/F32 at seed 71 and 128² F32 at seed 137. Floating comparisons include the stored bits of every channel. Maximum finite error is zero throughout.

| Root | Seed 42 settings | Independent controls at seeds 17, 71 and 137 |
|---|---|---|
| `blend.sbs/luminosity` | Opacity .67 with foreground, background and mask | Disable alpha blending, set opacity to .375 at 16² and 64² or .8125 at 128², and use the fields below |
| `gradient_axial.sbs/gradient_axial` and `gradient_axial_reflected` | Endpoints (.15,.3), (.7,.85), expansion disabled | Enable expansion. Use (.8125,.21875) and (.09375,.78125). At 16² set both endpoints to the first point |
| `hald_clut.sbs/hald_clut` | Identity lookup, input alpha control disabled | Use a new nonlinear lookup and vary the input alpha. Enable alpha control at 16² and 64², disable it at 128² |
| `hdr_merge.sbs/average_luminance_merge` | Original HDR field, exposure delta 2 | Use the HDR field below. Set exposure delta to −.75 at 16² and 1.375 otherwise |

For Luminosity, let t be 1 for background, 2 for foreground or 3 for mask. Generate C=(11x+17y+3xy+31t) mod 256. The mask is C. The color inputs contain (C, (41x−19y+11t) mod 256, (3x+79y+t) mod 256, 42×((13x+7y+31t) mod 7)). For the 512² lookup, use the low, middle and high six bits of the row-major index as R, G and B indices. Map each to the nearest byte using 255/63. The lookup stores (floor((R+2G)/3), 255−B, floor(R/8) XOR floor(G/4) XOR floor(B/2), 255). Feed it RGBA bytes ((11x+7y+3xy) mod 256, (43x+17y+28) mod 256, (5x+11y) mod 256, 23×((7x+3y) mod 11)). The original HDR input stores RGBA32F (((7x+23y+5xy) mod 303)/53, .09375+((19x+2y) mod 187)/51, ((x+8y) mod 103)/31, 1).

With coincident endpoints both gradients return the same quiet NaN, bit pattern `0x7fffffff`, at every one of the 256 pixels. All other outputs are finite. At 64² the five independent F32 images have respectively 4,092, 2,210, 2,053, 4,096 and 4,096 distinct stored pixels.

### Skipped loops and path buffers

A loop runs its initializer before testing its exit condition. If the condition is already true, or the iteration allowance is zero, its result is the zero of its declared result type. A loop that enters the body returns its last body value. MaterialCraft previously returned the initializer on a skipped loop. A Boolean initializer followed by a float body consequently stopped both path roots with a type error.

Use a standalone loop with a scalar body of 9 and an initializer of 7, true, (3,5,7,9) or an unwired initializer. Set the exit condition to true, false or pixel X ≥ .5, with iteration allowances 0, 1 or 2. Eight pixel functions at 16² seed 7 and 32² seed 91 match **10,240/10,240 LUM32F pixels**. The eight matching value functions give **16/16** exact typed results. Changing the result type to Boolean, integer pair or float pair with a true or false condition gives another **12/12** exact typed results. Three 32² uniform-parameter image probes match **3,072/3,072 pixels**. All maxima are zero. The regression also reads a variable assigned by the initializer after the loop. Direct Boolean or vector outputs from a grayscale Pixel Processor are a separate open conversion case.

| Path root | Inputs and controls | Exact pixels / total | Maximum error |
|---|---|---:|---:|
| `Impl/sort_by_blue` | Four original 16² RGBA32F cases with seed 0 or 42, including changed settings | 1,024/1,024 | 0 |
| Same | 64² seed 42, passthrough enabled, count 32, U8/U16/F16/F32 | 16,384/16,384 | 0 |
| Same | Passthrough disabled, count 7 at 16² seed 17; count 19 at 32² seed 71 in four formats and 64² seed 137 F32 | 8,448/8,448 | 0 |
| `Impl/compute_paths_lengths` | Four original 16² cases, including the 1×1 passthrough result, plus four 1×1 results at requested 64² seed 42 | 773/773 | 0 |
| Same | Five-point transformed path at 16² seed 17 F32, intermediate lengths disabled | 256/256 | 0 |
| Same | Intermediate lengths enabled and passthrough disabled, five or seven points, seed 17, 71 or 137 | Execution budget reached | Unavailable |

The sort holdout uses RGBA bytes ((17x+13y+5) mod 256, (29x−13y) mod 256, 23×((3x+7y) mod 11), (7x+5y+11) mod 256). Its outputs contain seven or nineteen distinct RGBA tuples. Across the fourteen sort cases **25,856/25,856 pixels** match. The length holdout starts with the original five- or seven-point closed path buffer and replaces each vertex (x,y) by (.0625+.625x+.1875y, .125+.0625x+.6875y). Set scale to (1.25,.8125). Both the resized 16² path buffer and independently sized buffers with header texel spacing 1/side reach the local execution limit with intermediate lengths enabled. The reference app returns a grayscale image. An error image supplies no comparable samples.

### Grayscale sampling of color inputs

Feed an original 128² image to a Pixel Processor and read input zero with the grayscale sampler at pixel-center UV plus (.1875,.5625) texels. Use seed 42 and a LUM32F output. With nearest and bilinear filtering, RGBA8, RGBA16, RGBA16F and RGBA32F each match 16,384/16,384 pixels per mode. On the signed F32 input, reading only red previously matched zero pixels in both modes. Maximum errors were 4.95148039 and 3.83905315. Adding green and blue first, then red, and multiplying by the float32 reciprocal of three after sampling matches all eight color cases exactly.

For RGBA8 use ((17x+31y) mod 256, floor(255x/127), 255−floor(255y/127), (7x+13y) mod 256). For the floating formats use the signed four-channel input under [3D linear gradient](#3d-linear-gradient). For RGBA16 encode round((channel+4)×65535/8) from that field. The eight grayscale-bridge controls also match all pixels. Their U8 input is (61x+137y+7xy) mod 256. Their U16 input is q=(2029x+4889y+193xy) mod 65536. For floating storage, supply a linear RGB image with the same float32 value q/8191−3.25 in each channel and alpha one, and select a grayscale bridge. The single stored channel is sampled directly. The sixteen comparisons match **262,144/262,144 pixels**, maximum error zero. A separate luminance-only float EXR was readable in the reference app and the current MaterialCraft decoder returned an error for that file.

### Crop output and area

The color and grayscale Crop roots each match **229,376/229,376** pixels across both outputs in the four original cases. Use the default graph at 128² and 256² with seed zero, repeat 128² with seed 42, then disable Show Crop Area, set Input Size to (128,128), Matrix to (1.2,.25,−.125,.8), Offset to (.1,−.15), and Filtering to 1 at 128². The inputs are the original foreground or grayscale ramp from the first pass. At 64² seed 42, retaining the 128² original input, these altered settings match a further 32,768/32,768 pixels per root across the two outputs and all four storage formats. Every maximum is zero.

For the independent case, enable Show Crop Area, set Input Size to (64,128), Matrix to (.75,−.125,.375,1.25), Offset to (−.1875,.3125) and Filtering to 0. Use the original color or height field under [height distance, texture mapping and skew](#height-distance-texture-mapping-skew-and-weave) at the output dimensions. At 128² seed 71 F32 and 64² seed 137 in all four storage formats, each root’s Output matches 32,768/32,768 pixels. Area matches in U8, U16 and F32. At 64² F16, color Area matches 4,054/4,096 and grayscale Area matches 4,087/4,096. Both maxima are .00048828125. The last operation on both Area outputs is a blend.

### Runtime angles and circular gradient

The Circular Gradient root matches **163,840/163,840 pixels** in thirteen cases, maximum error zero. Its default outputs at 128² and 256² with seed zero and 128² with seed 42 match all 98,304 pixels. Set the two points to (.3,.6) and (.8,.7), angle .2 and non-square expansion off. At 128² seed zero the authored output matches 16,384/16,384. At 64² seed 42, U8, U16, F16 and F32 each match 4,096/4,096. For an independent rotation, use (.71875,.21875), (.0625,.8125), angle −.15625 and enable non-square expansion. The 128² F32 output at seed 71 and all four formats at 64² seed 137 match another 32,768/32,768 pixels. This root takes no image input.

Nearest-sample R and G from an RGBA32F input and emit the angle of the vector (R,G) in radians. The six runtime fields below match **230,400/230,400 raw LUM32F samples**. Maximum finite error is zero. All 186 nonfinite outputs in the axes field agree bit for bit. The signed field previously matched 5/16,384 with maximum error .000235080719.

| Field | Definition | Exact pixels / total |
|---|---|---:|
| Signed channels, 128² | R and G from the original field under [3D linear gradient](#3d-linear-gradient), seed zero | 16,384/16,384 |
| First octant, 128² | R=1, G=i/16,383, seed zero | 16,384/16,384 |
| One turn, 256² | R=float32(i/65,535) times float32(2π), G=1−R, seed 42 | 65,536/65,536 |
| Near π/4, 256² | R has bits `0x3f490fdb-32768+i`. G=float32(8h/2³²−4), h=(747796405i+2891336453) modulo 2³², seed 42 | 65,536/65,536 |
| Axes and exceptional values, 32² | At x and y independently cycle through +0, −0, 1, −1, +∞, −∞, quiet NaN, 2⁻¹⁴⁰, 2⁻¹²⁰ and 2¹²⁰, seed zero | 1,024/1,024 |
| Exponent range, 256² | R=2^(x−128), G=2^(y−128). Negate R when (x+y) mod 3 is zero and G when (x+2y) mod 5 is zero, seed 91 | 65,536/65,536 |

Here i is the row-major pixel index. The first-octant fit is quartic in the smaller-to-larger magnitude ratio. It uses the measured runtime reciprocal and the same fused accumulation order as a four-channel dot product. In the other quadrants, reflection across the vertical and horizontal axes requires different additions of π/2 or π. Four separate literal-only controls at 32², seed zero, with vectors (1,1), (−1,−1), (.2,−.6) and (0,0) match another 4,096/4,096 LUM32F pixels. The runtime (1,1) angle has bits `0x3f490fe1` and the literal result has bits `0x3f490fdb`.

### Pixel Processor random draws

Emit four independent calls to `rand(1)` as RGBA32F. In the reference app the four values recover a unique initial 24-bit state at each of **27,968/27,968 pixels**. Advance the state with `s=(3626811*s+51137) mod 2^24` and return `floor(s/2)/2^23`. The four outputs then match at every pixel, maximum error zero. Use square canvases of sides 8, 16, 32, 64 and 128 at seed zero, also 32² at seeds 1, 2, 42 and 137, and 32×16 and 16×32 at seeds zero and 42. The first 32² pixel starts from 9,091,073 at seed zero and 3,342,178 at seed 42.

Four additional original graphs replicate a single draw to RGBA, reuse two draws in alternating channels, double a reused draw, or apply ranges 1, 1, −2 and .375 to successive draws. At 16², 32² and 64² with seed zero and 32² with seed 42, all **25,600/25,600 reference pixels** agree with the corresponding channel expressions, maximum error zero. The MaterialCraft evaluator now matches the progression given the recovered initial state. Its mapping from pixel position and graph seed to that state remains open. For example, the native and CPU four-draw images at 32² still match 0/1,024 entire pixels at seeds zero and 42, with maximum component errors .995849967 and .989675045.

### Color conversion, checker and clone

The ACEScg-to-linear-sRGB root matches **131,072/131,072 RGBA pixels across eight cases**, maximum component error zero. Bind the following original linear RGBA8 fields directly to its image input. At integer texel `(x,y)` on an `N×N` field, let `U=floor(255*x/(N-1))` and `V=floor(255*y/(N-1))`. Arithmetic in the four stored byte channels wraps modulo 256.

- Foreground RGBA is `(17*x+31*y, 3*x+7*y, 11*x+5*y, 5*x+11*y)`.
- Background RGBA is `(29*x+13*y, 255-U, V, 13*x+3*y)`.
- The scalar mask is `7*x+9*y`.

For the first four cases, use the foreground field at 128² and 256² with seed zero, repeat 128² with seed 42, then substitute the background at 128² with seed zero. The RGBA8 outputs match 16,384, 65,536, 16,384 and 16,384 pixels respectively. For the four remaining cases, keep the 128² background input, cook at 64² with seed 42, and select U8, U16, F16 and F32 on the final processor. Each format matches 4,096/4,096 pixels. The float32 output contains 4,096 distinct RGBA values and spans −0.53148961 to 1.51964462.

Two other roots matched the original four eight-bit cases at 128²/256², including seed 42 and the changed settings below. Each matched 114,688/114,688 eight-bit pixels, maximum error zero. The first 64² seed-42 format sweep exposed the differences below.

| Root and changed setting | U8 exact / max | U16 exact / max | F16 exact / max | F32 exact / max |
|---|---:|---:|---:|---:|
| `checker_1.sbs/checker_1`, tiling 3, 45-degree rotation enabled | 4,096 / 0 | 4,096 / 0 | 3,740 / 4.8828125e-4 | 4,096 / 0 |
| `clone.sbs/clone`, offset (.17, −.25), mask blur 0, original foreground and mask | 4,096 / 0 | 4,073 / 1 | 4,096 / 0 | 994 / 1.192093e-7 |

Each format column compares all channels of 4,096 pixels. Clone uses 128² input fields in the 64² cases.

### Clone mask and alpha rounding

In the 64² Clone graph the transformed foreground, source image and blurred mask each matched all 4,096 source pixels. The final normal-copy Blend produced the entire U16 and F32 discrepancy. After correcting that operation, all eight cases listed above match **131,072/131,072 pixels**, maximum error zero. This includes 4,096/4,096 each in RGBA8, RGBA16, RGBA16F and RGBA32F at 64².

For an independent 64² Blend control, form byte-valued foreground `(17*x+31*y, 7*x+3*y, 11*x+5*y, 5*x+11*y)`, background `(29*x+13*y, 128+7*x, 5+5*y, 13*x+3*y)` and mask `7*x+9*y`, each modulo 256. Use default normal-copy blending and seed zero. Test each as linear U8 and as float32 normalized by 255. The grayscale float fields may be encoded as equal RGB channels with an opaque alpha and connected to a grayscale input.

For each input format, compare the varying fields, an opaque foreground, a white mask, white and black foreground RGB, white and black background RGB, zero background alpha, and constant foreground alpha 127 with mask 159. Repeat the first three controls with grayscale source and background. Each of these 24 float-output cases matches 4,096/4,096 pixels, maximum error zero.

With float inputs, repeat the varying field, black background RGB and zero background alpha at global opacity .3 and .625. Repeat at .3 with alpha blending disabled. For signed fields replace each normalized input component `v` with `3*v-1` and run at opacity .3 and 1. Finally test opacity .07 with and without the mask, and opacity 1e-8 with the mask. All 24 controls match 4,096/4,096 pixels each, maximum error zero.

The measured operation first takes the complement of mask transparency in float32, then multiplies by the global opacity. Color weights take the complement of foreground transparency. Output coverage uses the original foreground alpha. Fusing the background contribution into the weighted foreground reproduces the measured RGB rounding, including negative inputs and non-opaque coverage.

### Height to normal in world units

Both roots in `height_to_normal_world_units.sbs` match **131,072/131,072 RGBA pixels each** across eight cases. For a square grayscale height input, store `(17*x+31*y) mod 256` at each integer coordinate. Use default settings at 128² and 256², repeat 128² with seed 42, then set `surface_size=100`, `height_depth=8`, `normal_format=0` and `sampling=1` at 128² with seed zero. Each root matches 16,384, 65,536, 16,384 and 16,384 RGBA8 pixels respectively. Keep those changed settings and the 128² input, then cook at 64² with seed 42. U8, U16, F16 and F32 each match 4,096/4,096 pixels for both roots. Every maximum error is zero. Each 64² output contains 49 distinct normals.

### Height distance, texture mapping, skew and weave

Twelve roots match **163,840/163,840 pixels each**, maximum component error zero, across thirteen cases per root. The total is **1,966,080/1,966,080 pixels**. Both versions of the render helpers were cooked separately. Read the final output bridge at its native dimensions and storage precision. The F32 controls below contain 36 distinct outputs for the height-distance root, 42 for Weave 3 and at least 3,682 for each mapping, shift or skew root.

| Package and graph | First changed case at 128² | Independent field and changed settings |
|---|---|---|
| `normal_to_height_hq.sbs/base_algorithm_height_distance` | Intensity .6, light angle .3, light offset .4, detail balance .6 | Height field, respectively .875, .125, .375 and .5 |
| `pbr_render.sbs` and `pbr_render2.sbs`, `PBR_render_mapping_color` | Substitute foreground for background texture | Color and UV fields, background color (.25,.5,.75,.375) |
| Both render packages, `PBR_render_mapping_grayscale` | Substitute full-range gray for limited-range gray | Height and UV fields, same background color |
| Both render packages, `chroma_shift` | Radius 2, angle .125 turns | Color field, radius 3.5, angle .3125 turns |
| `skew.sbs/skew` and `skew_grayscale` | Axis 1, amount .35, align 0 | Color or height field, axis 0, amount −.3125, align 2 |
| `uv_mapper.sbs/uv_mapper_color_sub` and `uv_mapper_grayscale_sub` | Filtering mode 0 | Color or height and UV fields, filtering mode 1. Color background (.25,.5,.75,.375) |
| `weave_3.sbs/weave_3` | Tiling 2, rotation enabled | Tiling 3, rotation disabled |

For each root the first three cases use defaults at 128² and 256² with seed zero, then 128² with seed 42. The fourth uses the first changed setting in the table at 128² and seed zero. Each root matches 16,384, 65,536, 16,384 and 16,384 pixels respectively. Height distance and Weave 3 use LUM16 here. The other roots use eight-bit output. At 64² use seed 42, the same changed control and U8, U16, F16 and F32 output. Each matches 4,096/4,096. Inputs keep their 128² source dimensions, except the limited-range gray fixture, which has a 64² counterpart.

Use the foreground and background byte fields specified under [color conversion, checker and clone](#color-conversion-checker-and-clone). The full-range gray is `(17*x+31*y) mod 256` and the limited-range gray is `32+((17*x+31*y) mod 160)`. At side N the original UV input is `(floor(255*x/(N−1)),floor(255*y/(N−1)),128,255)`. Height distance and grayscale skew take the full-range gray, chroma shift and color skew take the foreground. UV mapping takes the background or limited-range gray. Weave 3 has no image input.

For the five independent cases, use F32 at 128² with seed 71 and all four formats at 64² with seed 137. Inputs match the canvas size. Generate byte channels modulo 256 at integer (x,y). Color is `(11*x+37*y+29,41*x−19*y,3*x+79*y+1,13*x+7*y)`, height is its red channel and UV is `(5*x+3*y,9*y−7*x,round(255*y/(N−1)),A)`, where A is zero on `(x+y) mod 7 = 0` and 255 elsewhere. Each root matches all 16,384 float pixels and 4,096 pixels in each 64² format.

The same independent checks exposed remaining differences in five additional roots. All four 64² formats at seed 137 are exact. At 128² and seed 71 their F32 outputs differ as follows.

| Root | Changed control | Exact pixels / total | Maximum component error |
|---|---|---:|---:|
| `pattern_stripes.sbs/stripes` | Stripes 9, width .28125, softness .125, shift 3, align 2, filtering disabled | 16,341/16,384 | 1 |
| `quad_transform.sbs/quad_transform` | Color field, tiling enabled, four corners (.1,.125), (.2,.9), (.9,.2), (.8,.85) in parameter order p00, p01, p10, p11 | 16,378/16,384 | .00387579203 |
| `quad_transform.sbs/quad_transform_grayscale` | Height field, same corners | 16,378/16,384 | .000564604998 |
| `quantize_color.sbs/modify_color_palette_sub` | Color field, 16-bit ID equal to 4,369 times `(7*x+13*y) mod 16` | 254/256 | 1.19209290e-7 |
| `shape_splatter.sbs/shape_splatter_blend_color` | Two active patterns, color input opacity .625, HSL adjustment (.125,−.25,.0625) | 16,381/16,384 | 8.94069672e-8 |

For the palette control use selection mode 1, index 6, spread .125, contrast .25, distance color space 0, mode 1, opacity .375, override (.8,.125,.3) and HSL (.125,−.25,.0625). For the splatter control, bind Color to the background and second data input, UV to the color input and first data input. For pattern number j from 1 through 8, its RGBA bytes are `(13*x+11*y+23*j,5*x+31*j,17*y+13*j,19*x+7*y+29*j)` modulo 256.

For `spline_tools.sbs/internal_tools/compute_bridge_distance`, wire a Float2 value processor returning (2,1) into parent size and an Integer value processor into spline amount. Enable non-square correction. Bind the original 16² coordinate circle described under [spline preprocessing with wired counts](#spline-preprocessing-with-wired-counts). With count 1, the LUM32F outputs at 16² seed zero, 32² seed 42 and 64² seed 137 match 256/256, 1,024/1,024 and 4,096/4,096, maximum zero. With count 2 the counts are 252/256, 992/1,024 and 4,032/4,096. The respective maxima are 5.96046448e-8, .228374884 and .0570636913.

### Alveolus quadrant placement and falloff

In an original blank 64² pattern graph, follow the lower-left quadrant one, two or three times and draw a square of size (.25, .25) at the leaf. With zero offsets its centers are (16, 48), (8, 56) and (4, 60) in pixel-edge coordinates. Applying (.25, .125) to either the branch offset or the pattern offset moves the center by (16, 8) texels at every depth, with tiling at the image bounds. The size still shrinks at each depth. A separate unit Gaussian stamp shows a circular cutoff and a falloff of approximately `max(0, exp(-11.1*r²) - exp(-11.1))` in local coordinates. At 64², unit opacity and F32, the maximum difference between that analytic profile and the observed red channel is 3.5e-7.

The corrected offsets and Gaussian profile bring the default Alveolus pattern stage from **0/8,192** LUM16 pixels (maximum error 55,100) to **8,096/8,192** (maximum error 1). The following table compares the final output bridge. This generator uses no input image.

| Case | Size | Seed | Output format | Exact pixels / total | Maximum error |
|---|---:|---:|---|---:|---:|
| Defaults | 128² | 0 and 42 | LUM16 | 16,192/16,384 each | 2 |
| Defaults | 256² | 0 | LUM16 | 64,896/65,536 | 2 |
| `Gradient_Filled_Cells=true`, `Interstice_Width=.1` | 128² | 0 | LUM16 | 16,264/16,384 | 1,000 |
| Same changed parameters | 64² | 42 | LUM8, LUM16, LUM16F, LUM32F | 4,096/4,096 each | 0 |

### Anisotropic blur

For the grayscale root bind `(17*x+31*y) mod 256`. For the color root use the same value in R, `U` in G, `255-V` in B and 255 in A. The definitions of `U` and `V` are under [color conversion, checker and clone](#color-conversion-checker-and-clone). At 128² the color input bridge matches all 16,384 pixels. The first directional pass already differs.

| Case | Color exact / total | Gray exact / total | Maximum error, color / gray |
|---|---:|---:|---:|
| Defaults, 128², eight-bit | 11,526/16,384 | 11,526/16,384 | 1 / 1 |
| Angle .125 turns, anisotropy .8, 128², eight-bit | 3,228/16,384 | 3,310/16,384 | 16 / 16 |
| Quality 1, intensity 4, 256², eight-bit | 65,536/65,536 | 65,536/65,536 | 0 / 0 |
| Same 256² input and changed settings at 64², seed 42, U8, U16, F16, F32 | 4,096/4,096 each | 4,096/4,096 each | 0 / 0 |

At the first directional pass the default color case matches 11,944/16,384 pixels with maximum error 1. With angle .125 it matches 2,997/16,384 with maximum error 21. A separate F32 impulse at (64,64) on a 128² field, intensity 10 and angle .125, has nonzero samples on both sides of the diagonal.

### Fixed graph and instance output sizes

Both `pattern_tile_sampler.sbs/arrow` and `tile_sampler.sbs/arrow` specify an absolute 512×512 graph output. MaterialCraft now imports and saves that size separately from the default parent canvas. Each output of a graph instance also retains the dimensions and storage precision of its source. Downstream connections and cache hits use the selected output port.

For a size control, draw a centered white square half the canvas width and height on black. Give its graph an absolute output size of (6,5) in base-two exponents. With parent sizes 64²/U8, 128²/U16, 256²/F16 and 128²/F32, the stored output is 64×32. All four cases match 2,048/2,048 pixels, maximum error zero. Use seed zero for the first two cases and 42 for the last two.

A second control fixes only the final processor to 64×32 F32. Place the child in an original wrapper with a 128² or 64² parent. Exercise inherited instance size, parent-relative (0,0), and absolute (5,4). For each setting also request U8, U16, F16 and F32 on the instance. Each of these 18 cases produces a 64×32 LUM32F image and matches 2,048/2,048 pixels, maximum error zero. A two-output evaluator regression separately exercises a 64×32 F32 port, a 4×8 U8 port, their downstream consumers and a cached cook.

Neither Arrow root has input pins or exposed parameters. Restoring the fixed size brought each to 261,338/262,144 LUM8 pixels at its default 512² size, including seed 42, with maximum error 131. With a 64² parent and seed 42 the stored dimensions remained 512². The following counts precede the transform filtering correction.

| Stored format | Exact pixels / total | Maximum error |
|---|---:|---:|
| LUM8 | 261,338/262,144 | 131 |
| LUM16 | 261,338/262,144 | 33,667 |
| LUM16F | 260,826/262,144 | 0.513916015625 |
| LUM32F | 260,826/262,144 | 0.5137255192 |

### Transform edges and floating-point filtering

Both Arrow roots now match **1,835,008/1,835,008 pixels each**, maximum error zero. Repeat the 512²/U8 default twice at seed zero and once at seed 42. Then use a 64² parent and seed 42 with LUM8, LUM16, LUM16F and LUM32F on the final Blend. Every run returns a 512² image and matches 262,144/262,144 pixels. Checker 1 also matches **131,072/131,072 pixels**, maximum error zero, across the eight cases under [color conversion, checker and clone](#color-conversion-checker-and-clone). Its rotated 64² half-float result went from 3,740/4,096 to 4,096/4,096.

The first two mismatching Arrow stages transform a uniform white image with tiling off. A smaller control uses an original white 128² RGBA8 input with a transparent matte and bilinear filtering. Translate by (.25,.375) texels. The last column, last row and bottom-right corner of its RGBA32F output contain 49,151/65,535, 40,959/65,535 and 30,720/65,535 respectively. Each outside sample contributes the matte according to its own weight. Two further controls use matrix (1.23654997,0,0,4) with UV offset (.118276998,0), or matrix (1.01874995,−.982617021,1.01874995,.982617021) with offset (0,.367500007). These three transforms match 16,384/16,384 pixels in each of four output formats, maximum error zero.

To isolate matte precision, move a 128² image two UV units along both axes with tiling off. Set the matte to (.5,.1,−.312345,1.144444). Use U8, U16, F16 and F32 input storage and repeat with each of the four output formats. All 16 cases match 16,384/16,384 pixels each. In the F32 outputs the red channel retains the source-format levels 127/255, 32,767/65,535, .5 and .5 respectively.

For floating-point filtering, use the original signed 128² RGBA field specified under [3D linear gradient](#3d-linear-gradient). Store it first as F16 and then as F32. Use a tiled identity transform, F32 output, seed zero and offsets in texels of (.37,.18), (−.129,.678), (.601,0), (0,0), (.0039,.496) and (.501953125,.376953125). Each of these 12 cases matches 16,384/16,384 pixels, maximum error zero. The measured filter quantizes the four weights to a sum of 256. Per component it aligns contributing texels to their largest exponent with four extra significand bits, truncates, accumulates the weighted sum, and rounds to the source precision with halfway cases away from zero.

The same sampler was measured independently inside a Pixel Processor. Bind a 128² RGBA field to input zero, add a texel offset divided by 128 to the pixel-center UV, then read the input with bilinear filtering into an RGBA32F output. Run offsets (.25,.375), (.37,.18) and (−.129,.678) for each of U16, F16 and F32 input storage. All nine runs match 16,384/16,384 pixels each, **147,456/147,456** overall, maximum component error zero. The F16 and F32 cases use the same signed input field and produce different filtered values.

### Linear gradient 2 and multiangle albedo

Each root matches **131,072/131,072 pixels in eight cases**, maximum component error zero. Use defaults at 128² and 256² with seed zero, repeat 128² with seed 42, then run the changed settings at 128² with seed zero. Those four runs match 16,384, 65,536, 16,384 and 16,384 pixels respectively. Keep the changed settings and render at 64² with seed 42. Each of U8, U16, F16 and F32 matches 4,096/4,096 pixels for both roots.

- `gradient_linear_2` generates its own input. The first four outputs are LUM16. For the changed cases set `Tiling=3` and `rotation=1`.
- `multiangle_to_albedo` uses the foreground field on input 1 and the background field on inputs 2 through 8, as specified under [color conversion, checker and clone](#color-conversion-checker-and-clone). The first four outputs are RGBA8. For the changed cases set `samples_amount=4` and keep the 128² inputs for the 64² format sweep.

Six additional albedo cases use a different original image on each of the eight pins. For input number `j=1..8` the RGBA bytes are `((17+j)*x+(11+2*j)*y+29*j, (7*j+1)*x+5*y+13*j, 3*x+(13*j+1)*y+41*j, (5+j)*x+(11-j)*y+23*j)` modulo 256. Match the input and output dimensions. The default at 64² with seed zero matches 4,096/4,096 RGBA8 pixels. Four samples at 128² and seed 137 match 16,384/16,384 RGBA8 pixels. At 32² and seed 42 the same setting matches 1,024/1,024 pixels in each of U8, U16, F16 and F32. Maximum error is zero throughout.

### Atlas crop with wired value inputs

For `atlas_spliter.sbs/crop_and_transform_sub`, connect constant value processors to `shape_count` and `shape_data_table_size`. Use count 1, table size (1,1), and a single RGBA32F table texel (0,0,1,1). Bind a transparent 16² background and solid white 16² shape-index and opacity images. For the 128² color image, use linear RGBA8 texels `((17*x+31*y)%256, floor(255*x/127), 255-floor(255*y/127), (7*x+13*y)%256)`. Give the instance a parent-relative (0,0) size. The reference app returns RGBA32F at its authored precision.

| Rotation | 128² seed 0 or 42, exact / max | 256² seed 0, exact / max | 64² seed 42, exact / max |
|---|---:|---:|---:|
| Default | 16,384/16,384 · 0 | 49,152/65,536 · 0.250003815 | 4,096/4,096 · 0 |
| .125 turns | 16,320/16,384 · 1 | 49,091/65,536 · 1 | 4,064/4,096 · 0.996078431 |

The 16,384 differences at default rotation and 256² occupy exactly the lower-right 128² quadrant. At 128², three further controls each match 16,384/16,384, maximum error zero. Enable grid view, set count and table dimensions to zero, or set count to 2 and table dimensions to (1,2), respectively. All three use the same single table texel.

Set the final Pixel Processor's storage explicitly, keep the 128² input, and cook at 64² with seed 42. At default rotation, RGBA8, RGBA16, RGBA16F and RGBA32F each match 4,096/4,096 pixels, maximum error zero. At .125 turns, each matches 4,064/4,096. The respective maximum errors are 254, 65,278, 0.995605469 and 0.996078431.

### Spline preprocessing with wired counts

For `spline_tools.sbs/internal_tools/scatter_splines_on_splines_preprocess`, wire integer value processors to `input_spline_amount` and `custom_spline_amount`. Use input counts 1 and 2, custom count 1, `side=3`, `amount_mode=0` and `spline_amount_per_spline=1`. The 16² grayscale mask stores `(11*x+7*y)%256`. An original 16² circle coordinate buffer and its tangent buffer supply the other two images.

At 16² with seed zero or 42 and at 32² with seed zero, the two input counts return the exact integers 1 and 2 in all six comparisons, maximum error zero. With count 1 the image has two distinct RGBA8 values. All three images match 32,768/32,768 pixels after the instance retains the 2048×16 source dimensions. Before the fix MaterialCraft returned a 16² or 32² image through the wrapper.

To reproduce with a fresh pair of circle buffers, initialize two 16×16 RGBA32F images to zero. For `x=0..15`, let `a=2π(x+.5)/16`. In row zero, store `(.375+.1875*cos(a), .625+.1875*sin(a), 1, -1)` and `(-sin(a), cos(a), 0, 1)`. A separate run with these fields also matches all six typed integers and all 98,304 RGBA8 pixels through the wrapper, maximum error zero.

With the 16² parent, seed zero and count 1, set the output FX-Map storage explicitly to RGBA8, RGBA16, RGBA16F and RGBA32F. Each matches 32,768/32,768 pixels, maximum error zero. The reference app twice failed to return an image for the disconnected zero-count default within six minutes. That case has no pixel count or error measurement.

## 2026-11-17 root comparisons

The clean-room comparisons below use the 16.0.7 reference app with the Direct3D 11 backend and a release MaterialCraft CPU cook. They compare the raw output bridge at its stored precision. A pixel counts only when every channel agrees. Integer errors are in stored levels. Floating errors are absolute. Each case starts from the graph defaults. Where a row selects another storage format, set the final processor's format to absolute in both apps before cooking.

### 3D linear gradient

Bind a linear RGBA8 position map at the output size `N`. At zero-based texel `(x,y)`, set R and G to the nearest integers to `255*x/(N-1)` and `255*y/(N-1)`, with ties to even. Set B to 128 and A to 255. Use the default UV points, or set `points_position_mode=1` for world mode. The oblique case sets the world points to `(.2,.1,.3)` and `(.8,.9,.6)` and contrast to .35. The float output includes values outside the unit interval.

| Case | Size | Seed | Format | Exact pixels / total | Maximum error |
|---|---:|---:|---|---:|---:|
| Default UV | 256² / 512² | 0 | LUM32F | 65,536/65,536 and 262,144/262,144 | 0 |
| Default world points | 256² / 512² | 0 | LUM32F | 65,536/65,536 and 262,144/262,144 | 0 |
| Default UV | 128² | 42 | LUM32F | 16,384/16,384 | 0 |
| UV, contrast .6 | 256² / 512² | 0 | LUM32F | 26,043/65,536 and 104,172/262,144 | 2.384186e-7 |
| Oblique | 256² / 512² | 0 | LUM32F | 38,634/65,536 and 154,536/262,144 | 1.192093e-7 |
| Oblique | 128² | 42 | LUM8 | 16,384/16,384 | 0 |
| Oblique | 128² | 42 | LUM16 | 16,379/16,384 | 1 |
| Oblique | 128² | 42 | LUM16F | 16,367/16,384 | 2.44140625e-4 |

At 256² the two default cases previously matched 46,763 and 32,755 float pixels. A separate pair of 128² probes for each of float2, float3 and float4 isolated the dot product. Use a varying signed RGBA32F input and dot its leading lanes with themselves or with the corresponding prefix of `(.2126,.7152,.0722,-.39)`. The measured accumulation starts with the rounded Y product and fuses X, then Z and W when present. All 98,304 sampled dot and squared-length results now agree exactly. Literal-only dot, division and interpolation expressions retain their separately measured rounding.

For the signed test image, number RGBA components in row-major order starting at zero. Compute `v = i*747796405 + 2891336453`, then `v = ((v >> ((v >> 28)+4)) ^ v)*277803737` with unsigned 32-bit wrap at both steps. Store `8*((v ^ (v >> 22))/2^32)-4` as float32. Interpolating R to G with B as the factor and dividing R by the float32 value 1.7320508 each match 16,384/16,384 pixels with zero error. With G as the divisor, 14,372/16,384 match and the maximum error is 4.8828125e-4. Multiplying R by a separately captured reference reciprocal of G reproduces all 16,384 reference quotients. A standalone `R*G-B` probe matches 11,721/16,384 with a maximum error of 1.907349e-6. Fusing that product and subtraction reproduces every reference pixel. At 256² the contrast-.6 reference output is exactly a fused `gradient*2.5-.75` on the zero-contrast reference image. Separate graph operations and reciprocal approximation remain open. The unconnected Position case from the first pass also remains open.

### Basic and ridged 3D Perlin

Use the same position ramp. For a controlled comparison, bind an original 1024×16 grayscale lookup to the internal bitmap in both apps. With one Python `random.Random(104729)` instance, shuffle a fresh list of integers 0 through 1023 for each successive row. Write each entry as the nearest integer to `65535*entry/1023` in a linear 16-bit PNG. Both internal lookup outputs matched all 16,384 levels with zero error. The changed parameter is `scale=6` for basic Perlin and `roughness=.7` for ridged Perlin. Use an 8-bit parent format.

| Case | Size | Seed | Format | Basic exact / total | Basic max | Ridged exact / total | Ridged max |
|---|---:|---:|---|---:|---:|---:|---:|
| Default | 128² | 0 | LUM8 | 16,383/16,384 | 1 | 16,384/16,384 | 0 |
| Default | 256² | 0 | LUM8 | 65,534/65,536 | 1 | 65,536/65,536 | 0 |
| Default | 128² | 42 | LUM8 | 16,384/16,384 | 0 | 16,384/16,384 | 0 |
| Changed | 128² | 0 | LUM8 | 16,384/16,384 | 0 | 16,384/16,384 | 0 |
| Changed | 128² | 42 | LUM16 | 16,360/16,384 | 1 | 16,304/16,384 | 1 |
| Changed | 128² | 42 | LUM16F | 16,384/16,384 | 0 | 16,378/16,384 | 4.8828125e-4 |
| Changed | 128² | 42 | LUM32F | 12,105/16,384 | 5.066395e-7 | 3,119/16,384 | 4.172326e-7 |

The changed float case previously matched 10,399 pixels for basic Perlin and 3,926 for ridged Perlin. Whole-graph rounding remains visible in both. A separate seed-zero comparison used each app's shipped lookup, with MaterialCraft's tracked CC0 image read from Git. Basic Perlin matched 173/16,384 at 128² and 662/65,536 at 256², with maximum errors of 203 levels at both sizes. Ridged Perlin matched 176/16,384 and 768/65,536, with maximum errors of 139 and 154 levels. The shared lookup isolates an asset-dependent difference from the arithmetic.

### Grayscale and color 3D planar projection

Bind the position ramp above and a constant world-space normal of RGBA8 `(128,128,255,255)`. The grayscale texture is `T=(17*x+31*y) mod 256`. The color texture is RGBA8 `(T,R,255-G,(251+3*x+5*y) mod 256)`, where R and G are the position ramp bytes. Keep all input maps linear and the same size as the graph. The rotated case sets `rotation=.125` and `scale=.38`. Each format pair below gives the grayscale and color output formats respectively.

| Case | Size | Seed | Formats | Grayscale exact / total | Gray max | Color exact / total | Color max |
|---|---:|---:|---|---:|---:|---:|---:|
| Default | 128² | 0 | LUM16, RGBA8 | 16,384/16,384 | 0 | 16,384/16,384 | 0 |
| Default | 256² | 0 | LUM16, RGBA8 | 65,536/65,536 | 0 | 65,536/65,536 | 0 |
| Default | 128² | 42 | LUM16, RGBA8 | 16,384/16,384 | 0 | 16,384/16,384 | 0 |
| Rotated | 128² | 0 | LUM16, RGBA8 | 16,334/16,384 | 226 | 16,378/16,384 | 1 |
| Rotated | 64² | 42 | LUM8, RGBA8 | 4,094/4,096 | 1 | 4,094/4,096 | 1 |
| Rotated | 64² | 42 | LUM16, RGBA16 | 4,093/4,096 | 32 | 4,093/4,096 | 32 |
| Rotated | 64² | 42 | LUM16F, RGBA16F | 4,093/4,096 | 4.8828125e-4 | 4,093/4,096 | 4.8828125e-4 |
| Rotated | 64² | 42 | LUM32F, RGBA32F | 4,093/4,096 | 4.882888e-4 | 4,093/4,096 | 4.882888e-4 |

Before the sampler change the default 256² cases matched 65,048 grayscale pixels and 65,492 RGBA pixels. An independent one-processor graph isolates the integer texture read. In a 256² source, use the 2×2 block starting at (12,15). Light one corner at a time, then repeat with corner levels `(.37,.13,.91,.68)` rounded to the input precision, in row-major order. For output pixel (x,y), sample at `((12.5+x/256)/256,(15.5+y/256)/256)`. Run all five patterns with RGBA8 and RGBA16 input and read the red channel into LUM32F. All 655,360 pixels match exactly.

In units of 1/256 texel, the rounded horizontal and vertical fractions are `dx` and `dy`. The bottom-right weight is `q=floor((dx*dy+128)/256)`. The other three weights in row-major order are `256-dx-dy+q`, `dx-q` and `dy-q`. Accumulate the four normalized 16-bit levels with these integer weights, divide by 256 and round half up. A separate 128² high-frequency texture sweep at seven fixed offsets matched all 114,688 pixels, including wrapped edges. The rotated graph cases still differ with this sampler. Floating-source filtering has separate rounding behavior and remains open.

### 3D surface and volume render passes

The four rows of the inventory preceding `position_debug` share one package. The small output side `N` is 16 for irradiance, volume IBL and volume light, and 32 for surface position. The large case doubles both dimensions. The surface position parent is 8-bit. The other parents are float32. At the small and large sizes, generate new input ramps with byte values `U=floor(255*(x+.5)/N)` and `V=floor(255*(y+.5)/N)`. Bind all images as linear.

- Surface irradiance uses a constant float SDF of 128, position `(U,V,128,255)`, normal `(128,128,255,255)`, sample table `(64,64,255,255)`, environment `(U,V,192,255)` and grayscale noise `(17*x+31*y) mod 256`. The changed case sets `i=16` and albedo to `(.3,.6,.9)`.
- Surface position uses a constant white SDF. The changed case enables the ground plane and sets camera UV to `(.35,.6)` and field of view to 65 degrees.
- The volume passes use constant float SDF −.1 and density .001, the same position ramp, and the noise formula evaluated at the noise input's native 256². IBL also uses the irradiance sample table and environment. Its changed case sets `i=1` and absorption to 2. Volume light sets light intensity to 2 and light UV to `(.3,.6)` for its changed case.

Each cell gives **exact pixels / total (maximum error)**. The native output formats in column order are RGBA32F, RGBA8, RGBA16F and RGBA16F.

| Case | Surface irradiance | Surface position | Volume IBL | Volume light |
|---|---:|---:|---:|---:|
| Small, defaults, seed 0 | 0/256 (8.940697e-8) | 1,024/1,024 (0) | 9/256 (.00634765625) | 250/256 (6.103516e-5) |
| Large, defaults, seed 0 | 0/1,024 (7.450581e-8) | 4,096/4,096 (0) | 44/1,024 (.00634765625) | 994/1,024 (.0001220703125) |
| Small, defaults, seed 42 | 0/256 (8.940697e-8) | 1,024/1,024 (0) | 8/256 (.00537109375) | 250/256 (6.103516e-5) |
| Small, changed, seed 0 | 0/256 (8.940697e-8) | 1,023/1,024 (1) | 9/256 (.009765625) | 250/256 (6.103516e-5) |
| Small, changed, seed 42, RGBA8 | 256/256 (0) | 1,023/1,024 (1) | 69/256 (3) | 255/256 (1) |
| Small, changed, seed 42, RGBA16 | 256/256 (0) | 1,023/1,024 (246) | 0/256 (701) | 250/256 (2) |
| Small, changed, seed 42, RGBA16F | 256/256 (0) | 410/1,024 (4) | 3/256 (.0107421875) | 250/256 (6.103516e-5) |
| Small, changed, seed 42, RGBA32F | 0/256 (8.940697e-8) | 0/1,024 (.67333984375) | 0/256 (.010691524) | 0/256 (4.068762e-5) |

**Irradiance.** Replace noise by zero, environment by `(64,128,192,255)` and position by `(128,128,128,255)`. Both renders become spatially constant and all six input bridges match. RGB still differs by at most 8.940697e-8 and alpha is exact. Changing only the sample table from byte `(64,64,255,255)` to float `(.25,.25,1,1)` makes all 256 RGBA32F pixels exact. Replacing only the normal by `(.5,.5,1,1)` gives 0/256 and maximum error 1.192093e-7. The remaining difference depends on the sampled direction even with flat lighting.

**Surface position.** A black SDF with defaults at 16² matched 256/256 RGBA8 pixels. The changed camera at 64² still matches 4,092/4,096 with maximum error 2. At 32² and RGBA32F, changing only field of view gives 0/1,024 with maximum error 1.490116e-7. Changing only camera UV gives 0/1,024 with maximum error 1.269579e-5. With both changes the largest error occurs at (0,2), where X is −7683.423 in MaterialCraft and −7684.096 in the reference. Alpha agrees at every pixel. Integer storage hides much of this coordinate difference.

**Volume integration.** At 16² and RGBA32F, replace noise by zero, position by `(128,128,128,255)` and the IBL environment by `(64,128,192,255)`. All six IBL inputs and all four light inputs match exactly. IBL still gives 0/256 with maximum error .00578630. Disabling dithering reduces this to 3.039837e-6 with 16/256 exact at both seeds 0 and 42. With nonconstant inputs and dithering disabled the maximum is .0002391934 and 12/256 match. Light gives 0/256 on the flat inputs with maximum error .0001328364, or .0001164377 with dithering disabled. Setting density to zero makes both flat, undithered renders exact at all 256 pixels. Nonzero-density integration remains open.

### Position debug

All **622,592/622,592 RGBA pixels** match across 14 cases, with maximum component error zero. The two previously failing 256² sphere cases match the original reference captures, including all 17 colored texels in each. Changing `debug_pos` from `(.5,.5)` to `(.25,.75)` changes four pixels between reference images. Both internal 4096² processors cook at full size.

Use a float32 parent, the RGBA8 position ramp above and an original signed-distance atlas. For side `N=128` or 256, let `s=N/16`, `X=((x mod s)+.5)/s-.5`, `Y=((y mod s)+.5)/s-.5`, and `Z=(floor(x/s)+16*floor(y/s)+.5)/256-.5`. Compute `sqrt(X²+Y²+Z²)-.3` and store the result in each RGB channel of a float32 EXR with alpha 1.

| Probe | Size | Seed | Output formats | Exact pixels / total | Maximum error |
|---|---:|---:|---|---:|---:|
| Default debug point | 128² | 0 and 42 | RGBA32F | 32,768/32,768 | 0 |
| Default and changed debug point | 256² | 0 | RGBA32F | 131,072/131,072 | 0 |
| Changed debug point | 128² | 42 | RGBA8, RGBA16, RGBA16F, RGBA32F | 65,536/65,536 | 0 |
| Changed debug point | 256² | 42 | Same four formats | 262,144/262,144 | 0 |
| Changed point, plane and shifted sphere | 256² | 42 | RGBA32F | 131,072/131,072 | 0 |

The plane uses `.55*X+.3*Y-.7*Z+.06`. The shifted sphere uses `sqrt((X-.125)²+(Y+.1)²+(Z-.125)²)-.36`. Both produced the same sparse marker as the centered sphere for the changed debug point. The 128² cases had zero colored texels. The parity mark covers these output images and settings.

### Panoramic LOD

The panorama helper exposes one image input. For its first-pass fixtures, let `U=floor(255*x/(N-1))`, `V=floor(255*y/(N-1))` and `T=(17*x+31*y) mod 256`. Bind linear RGBA8 `(T,U,255-V,255)`. The alpha case uses `(7*x+13*y) mod 256` for A. The new reference outputs for all three first-pass cases match their archived raw pixels exactly. The fractional-alpha input and opaque input produce identical output here. The four 64² cases use the varying-alpha color texture from the planar projection comparison.

| Input | Size | Seed | Format | Exact pixels / total | Maximum error |
|---|---:|---:|---|---:|---:|
| Opaque | 128² | 0 and 42 | RGBA8 | 15,883/16,384 each | 125 |
| Opaque | 256² | 0 | RGBA8 | 63,309/65,536 | 126 |
| Varying alpha | 128² | 0 | RGBA8 | 15,883/16,384 | 125 |
| Flat (23,139,211), alpha 75 or 255 | 128² | 0 | RGBA8 | 16,128/16,384 each | 116 |
| Varying alpha | 64² | 42 | RGBA8 / RGBA16 | 3,964/4,096 each | 1 / 257 |
| Same input | 64² | 42 | RGBA16F / RGBA32F | 3,964/4,096 each | .00439453125 / .0039215982 |

MaterialCraft previously sampled the full-resolution input for every bilinear transform. The 128² and 256² opaque cases then matched 719 and 5,015 pixels. An isolated transform uses a 2048×1024 RGBA8 input `(T,x mod 256,(255-y) mod 256,255)`, a 128² output, matrix `diag(-1,1.99309003)`, offset `(0,.5)`, horizontal-only wrap and opaque black matte. Its input bridge matches all 2,097,152 texels. The transform improved from 8,216/16,384 (maximum 117) to 16,384/16,384 (0). Changing the vertical scale to 2 improved from 8,192/16,384 (103) to 16,384/16,384 (0). When the first-pass 128² input is resized to the same 2048×1024 bridge, 1,923,823/2,097,152 input texels match (maximum 2). The two transforms give 16,287 and 16,284 exact output pixels, both with maximum 1. The full graph still differs along rows 0 and 47 with a flat input.

For an independent mip-level check, generate a 128² RGBA grid using the row-major unsigned hash from the gradient probe. Use the low 8 or 16 bits of the final hash for each integer channel. For a signed float input store `4*hash/2^32-2` in float32. Run an identity transform at 128², 64², 32² and 16². With automatic mip selection, both RGBA8-to-RGBA8 and RGBA16-to-RGBA16 match 21,760/21,760 pixels with zero error. For RGBA8 input and output, setting the manual mip level to 0, 1 or 2 also gives 21,760/21,760 at each level. An automatic uniform scale of 1.5 gives the same exact count. For the signed RGBA32F input and output, the four sizes match 16,384, 2,781, 308 and 29 pixels respectively, with maxima 0, 1.192093e-7, 5.960464e-8 and 2.980232e-8. Float filtering remains open.

### 3D volume mask and Worley noise

Use the first-pass linear position texture. For side `N`, store RGBA8 `(floor(255*x/(N-1)),floor(255*y/(N-1)),128,255)`. The mask's rotated case uses box shape 1, rotation `(.1,.2,.3)`, size `(.8,.6,1)` and feather width .1. The Worley border case sets style 1, border width .2 and roundness .1. For its 64² output-format sweep, round the two position channels to nearest instead of flooring. Each cell gives exact pixels / total (maximum error).

| Case | Volume mask | Worley noise |
|---|---:|---:|
| 128², defaults, seed 0, LUM16 | 16,384/16,384 (0) | 1/16,384 (44,494) |
| 256², defaults, seed 0, LUM16 | 65,536/65,536 (0) | 3/65,536 (44,494) |
| 128², defaults, seed 42, LUM16 | 16,384/16,384 (0) | 1/16,384 (43,865) |
| 128², scale .4 for mask or 6 for Worley, seed 0, LUM16 | 16,384/16,384 (0) | 2/16,384 (40,476) |
| 128², rotated mask or Worley border, seed 0, LUM16 | 16,326/16,384 (1) | 1/16,384 (32,729) |
| Same settings, seed 42, LUM8 (mask 128², Worley 64²) | 16,383/16,384 (1) | 45/4,096 (138) |
| Same sizes and seed, LUM16 | 16,326/16,384 (1) | 0/4,096 (35,289) |
| Same sizes and seed, LUM16F | 16,349/16,384 (.0078125) | 4/4,096 (.53833008) |
| Same sizes and seed, LUM32F | 664/16,384 (.000015258789) | 0/4,096 (.53847277) |

**Mask.** The 128² box with default rotation, size and feathering gives 16,379/16,384 exact LUM16 pixels, with maximum error 1. The RGBA16 position bridge matches all 16,384 pixels. In LUM32F, changing only box rotation gives 564/16,384 (1.907349e-6), only size gives 14,088/16,384 (9.536743e-7), and only feather width gives 13,716/16,384 (3.814697e-6). A sphere with `position_input=1` and `position_worldspace=(.375,.375,.5)` matches 16,384/16,384 float pixels exactly.

**Worley.** With a constant input position of `(0,0,0,255)`, no LUM32F pixel matches in the 32² default case at seed 0 (maximum .02120566) or seed 42 (.07068205). The graph's internal random table uses four draws. A standalone Pixel Processor writing four successive `rand(1)` values to RGBA16 reproduces a discrepancy before any cell-distance calculation. It gives 0/1,024 exact pixels at 32², seed 0 (maximum 65,122), 0/16,384 at 128², seed 0 (65,136), and 0/16,384 at 128², seed 42 (65,348). At seed 0 the first reference pixel changes with resolution, from `(47238,35719,5497,27913)` to `(37730,56747,64327,4184)`. MaterialCraft returns `(42015,40251,56435,61318)` at both sizes. The random sequence and any residual differences in the distance calculation remain open.

### November 17 validation

All six new regression tests failed with their corresponding fixes disabled or absent. The release suites for nodes, evaluation and formats passed 246 tests. Two opt-in format interoperability tests were skipped. Workspace formatting, strict all-target Clippy, dependency layers and WebAssembly checks passed. Raw captures and generated input images were kept outside the repository.

## Remaining differences

The following entries record the first pass. The dated comparisons above give the newer results for revisited roots.

- **`3d_gradient.sbs/3d_linear_gradient` - Too hard for this pass:** Corrected the wrong channel mapping and normalized-integer bilinear sampling offset. Compared generated position maps in UV and world modes, with zero and nonzero contrast, at 256 and 512 pixels (eight cases). Remaining 32-bit differences reach 7.153e-7; some values straddle a 16-bit rounding boundary by one level. The default world-mode case matches every pixel after 16-bit conversion at both sizes, but full float parity is not established. Further work needs matching the reference shader arithmetic rounding. An unconnected Position input yields NaNs in the reference app; MaterialCraft retains its actionable non-finite arithmetic error. Moving to the next node.

- **`3d_perlin_noise.sbs/3D_perlin_noise` - Too hard for this pass:** Exact at 128x128 for defaults, seed 42, and scale 6. At 256x256, 3 of 65,536 eight-bit pixels remain one level different: (x=246,y=155), (25,179), and (154,227). Input maps are generated linear XY ramps with Z=128/255 and alpha=1. The output-format error is fixed; residual arithmetic/conversion rounding still prevents a parity mark.

- **`3d_perlin_noise.sbs/3D_ridged_noise_fractal` - Too hard for this pass:** Default and roughness-0.7 cases at 128x128 are exact. The default 256x256 case has two differing eight-bit pixels; seed 42 at 128x128 has one. Maximum difference is one level. Full parity is not claimed.

- **`3d_planar_projection.sbs/3d_planar_projection` - Too hard for this pass:** With generated position, normal, and high-frequency grayscale texture inputs, 16,240/16,384 sixteen-bit pixels match at 128x128 and 65,048/65,536 at 256x256. Maximum differences are 125 and 128 levels respectively, concentrated at texture sampling boundaries. Independent sampler half-step and nearest-read probes agree; remaining projection arithmetic is a likely contributor.

- **`3d_planar_projection.sbs/3d_planar_projection_color` - Too hard for this pass:** With generated position, normal, and RGBA texture inputs, 16,370/16,384 pixels match at 128x128 and 65,498/65,536 at 256x256. All four eight-bit channels, including alpha, were compared. Remaining differences are at most one level.

- **`3d_texture_render.sbs/3d_texture_surface_render_irradiance` - Too hard for this pass:** Eight 16x16 RGBA32F cases use original position/normal maps, constant positive HDR distance fields, directional sample tables, and a colored environment ramp. Four nonzero lighting cases (iteration 0, 1, 16, and 128) differ by at most 8.941e-8 in RGB; alpha is exact. Four opposite-hemisphere cases match exactly but produce black lighting. All cooks succeed. Full float parity needs matching shader arithmetic rounding; black-image agreement alone is not used for a parity mark.

- **`3d_texture_render.sbs/3d_texture_volume_render_ibl_pass` - Too hard for this pass:** Six 16x16 RGBA16F cases use original position/noise/environment/sample maps and constant negative distance fields, with dense and thin media, iteration changes, and absorption changes. Thin-media cases produce visible nonzero lighting but only 9/256, 10/256, and 4/256 pixels match exactly; maximum component differences are 0.006348, 0.005372, and 0.011719. Three additional thin-media cases bind an original 256x256 noise map at the required input size; they still differ, reaching 0.012208. All cooks succeed. The volume-integration discrepancy needs further isolation; half-float storage is separately verified.

- **`3d_texture_render.sbs/3d_texture_volume_render_light_pass` - Too hard for this pass:** Fixed the function-expansion rejection. All three 16x16 thin-media cases now cook successfully, including changed light position/intensity and changed albedo. Exact RGBA16F pixels: 252/256, 254/256, and 254/256. Maximum component differences are 6.104e-5, 3.052e-5, and 1.526e-5. Eight pixels in total prevent a parity mark.

- **`3d_texture_render.sbs/3d_texture_surface_render_position` - Too hard for this pass:** All four initial 16x16 RGBA8 cases match exactly (solid/positive fields, ground plane, changed camera). Solid-field 64x64 and ground-plane 128x128 cases also match exactly. A 64x64 case with a changed camera direction and 65-degree field of view has four differing pixels, at most two levels apart. All cooks succeed. Full camera/geometry parity is not claimed.

- **`3d_texture_render.sbs/position_debug` - Too hard for this pass:** Initial zero/positive/midpoint fields and checker fields produce constant images and are insufficient for a parity mark. A generated signed sphere-distance atlas and original position map produce nonconstant 256x256 RGBA32F outputs. Center and changed debug-selection cases each match 65,519/65,536 pixels; 17 pixels per case differ by up to 1.0. Both internal 4096x4096 processors cook at full native size.

- **`3d_volume_mask.sbs/3d_volume_mask` - Too hard for this pass:** Corrected omitted nearest sampling and finite clamping after division by zero. Default 128x128, default 256x256, and changed-scale 128x128 outputs are exact (98,304 sixteen-bit pixels). Box mode has four one-level differences; rotated, resized, narrow-feather box mode has 58 one-level differences. All five cases cook successfully.

- **`3d_worley_noise.sbs/3D_worley_noise` - Too hard for this pass:** All cooks succeed, but the noise field differs substantially. Defaults at 128x128/256x256, seed 42, scale 6, and border style have only 1, 3, 1, 2, and 1 matching sixteen-bit pixels respectively; maximum difference is 44,494 levels. Changing omitted sampler defaults does not resolve this.

- **Pixel Processor random stream - Unresolved:** Three original scalar-random probes (16x16 seeds 0/42 and 32x32 seed 0) disagree at all 1,536 float pixels. The reference stream also depends on output resolution. No guessed replacement sequence was introduced. A causal link to the remaining noise or volume-lighting differences is not established.

- **`alveolus.sbs/alveolus` - Too hard for this pass:** Defaults at 128x128/256x256 and seed 42 have no exactly matching sixteen-bit pixels. Gradient-filled cells with interstice width 0.1 match 13,880/16,384 pixels, but differ by up to 65,535 levels. All four cooks succeed.

- **`anisotropic_blur.sbs/anisotropic_blur` and `anisotropic_blur_grayscale` - Too hard for this pass:** Compared default 128x128, rotated high-anisotropy 128x128, and quality/intensity changes at 256x256. All cooks succeed. Color matches 224/16,384, 109/16,384, and 977/65,536 RGBA8 pixels; grayscale matches 252/16,384, 125/16,384, and 1,024/65,536 pixels. Maximum difference reaches 87 eight-bit levels. Blur sampling/kernel parity remains open.

- **`atlas_spliter.sbs/crop_and_transform_sub` - Too hard for this pass:** Its two typed value pins are outside the reference graph's exposed-parameter API. The direct comparison harness cannot set shape count/table size through that API. A future comparison needs an original wrapper that supplies these typed value inputs; the regular image-pin bindings are available.

- **`3d_viewer.sbs/ibl_specular_contrib_anisotropic/panoramic_lod` - Too hard for this pass:** Corrected one-axis wrapping. For 128x128 and 256x256 opaque color inputs, exact RGBA8 pixels improve from 576/16,384 and 4,391/65,536 to 719/16,384 and 5,015/65,536. Maximum component differences drop from 252/254 to 128/127. A varying-alpha 128x128 case still has no fully matching pixels and a maximum difference of 239. All cooks succeed. Input resizing and transform minification/filtering remain open.

- **`bevel_smooth.sbs/blur_gaussian_grayscale` - Too hard for this pass:** Defaults at 128x128 and 256x256 are exact but perform no visible blur at these settings. Intensity 5 at 128x128 matches only 158/16,384 eight-bit pixels and differs by up to 23 levels. All cooks succeed; nonzero-kernel parity remains open.

- **`bevel_smooth.sbs/jump_flood_segment_iteration` - Too hard for this pass:** All cooks succeed. Default 128x128, iteration 1, changed distance mode/scale, and default 256x256 match 16,366/16,384, 16,376/16,384, 16,379/16,384, and 65,466/65,536 RGBA8 pixels. A total of 101 segment selections differ; maximum component difference is 192. Input segment endpoints are original synthetic color fields.

- **`bevel_smooth.sbs/debug/bevel_smooth_debug_position` - Too hard for this pass:** Corrected undefined function intermediates, ordered NaN comparisons, omitted color-mode imports, and pattern alpha. All four cases now cook successfully in RGBA32F. Default bounded-position maps at 128x128/256x256 match 16,359/16,384 and 65,415/65,536 pixels. Offset and pixel-selection cases match 14,326/16,384 and 15,362/16,384. Remaining stamp placement/coverage differences reach 19.165; a parity mark is withheld.

- **`blackbody.sbs/blackbody` - Too hard for this pass:** Compared the default 3,000 K at 128x128 and 1,000/6,500/12,000 K at 16x16, retaining RGBA32F HDR output. Alpha is exact, but no complete RGBA pixel matches; RGB differences reach 4.769e-7. All cooks succeed.

- **Remaining blend cases:** All four RGBA8 cases cook for every graph. `blend.sbs/color` matches 114,111/114,688 pixels, with differences up to nine levels. `color_burn` and `color_dodge` each have seven differing pixels, reaching 16 and 144 levels; these include degenerate black/white endpoints and one-level arithmetic differences. `dissolve_2` matches 75,035/114,688 pixels, with stochastic differences up to 254 levels. After the grayscale default fix, `luminosity` has two one-level differences; the fused-dot recheck retains those two differences. These graphs are not marked parity.

- **`blur_hq.sbs/blur_hq` and `blur_hq_grayscale` - Too hard for this pass:** All four cases cook. Intensity 1 at 128x128 is exact for both output types but has no visible blur at that scale. Default 128x128/256x256 and high-quality intensity-5 color cases match 2/16,384, 0/65,536, and 12/16,384 RGBA8 pixels; grayscale matches 125/16,384, 15,168/65,536, and 144/16,384 pixels. Differences reach 52 color levels and 30 grayscale levels. Directional blur kernel/sampling remains open.

- **`bnw_spots_1.sbs/bnw_spots_1` - Too hard for this pass:** All four cases cook. Exact pixels for defaults at 128x128/256x256, seed 42, and changed parameters: 0/16384, 5/65536, 0/16384, 0/16384. Maximum component difference is 62677 integer levels. Pattern/noise behavior remains open.

- **`bnw_spots_3.sbs/bnw_spots_3` - Too hard for this pass:** All four cases cook. Exact pixels for defaults at 128x128/256x256, seed 42, and changed parameters: 0/16384, 0/65536, 0/16384, 0/16384. Maximum component difference is 63995 integer levels. Pattern/noise behavior remains open.

- **`brick_1.sbs/brick_1` - Too hard for this pass:** All four cases cook. Exact pixels for defaults at 128x128/256x256, seed 42, and changed parameters: 14992/16384, 59968/65536, 14992/16384, 8352/16384. Maximum component difference is 65535 integer levels. Pattern/noise behavior remains open.

- **`brick_2.sbs/brick_2` - Too hard for this pass:** All four cases cook. Exact pixels for defaults at 128x128/256x256, seed 42, and changed parameters: 15264/16384, 61360/65536, 15264/16384, 8896/16384. Maximum component difference is 65535 integer levels. Pattern/noise behavior remains open.

- **`brick_3.sbs/brick_3` - Too hard for this pass:** All four cases cook. Exact pixels for defaults at 128x128/256x256, seed 42, and changed parameters: 4616/16384, 24922/65536, 5408/16384, 2880/16384. Maximum component difference is 65535 integer levels. Pattern/noise behavior remains open.

- **`cartesian_polar_transformation.sbs/cartesian_to_polar` - Too hard for this pass:** All four cases cook. Exact pixels for defaults at 128x128/256x256, seed 42, and a changed input: 14130/16384, 52081/65536, 14130/16384, 14081/16384. Maximum component difference is 4 integer levels. Polar sampling/coordinate precision remains open.

- **`cartesian_polar_transformation.sbs/polar_to_cartesian` - Too hard for this pass:** All four cases cook. Exact pixels for defaults at 128x128/256x256, seed 42, and a changed input: 15330/16384, 61564/65536, 15330/16384, 15842/16384. Maximum component difference is 1 integer levels. Polar sampling/coordinate precision remains open.

- **`cartesian_polar_transformation.sbs/cartesian_to_polar_grayscale` - Too hard for this pass:** All four cases cook. Exact pixels for defaults at 128x128/256x256, seed 42, and a changed input: 15216/16384, 58474/65536, 15216/16384, 15281/16384. Maximum component difference is 4 integer levels. Polar sampling/coordinate precision remains open.

- **`cartesian_polar_transformation.sbs/polar_to_cartesian_grayscale` - Too hard for this pass:** All four cases cook. Exact pixels for defaults at 128x128/256x256, seed 42, and a changed input: 15804/16384, 63310/65536, 15804/16384, 15794/16384. Maximum component difference is 1 integer levels. Polar sampling/coordinate precision remains open.

- **`cells_1.sbs/cells_1` - Too hard for this pass:** All four cases cook. Exact pixels for defaults at 128x128/256x256, seed 42, and changed parameters: 5/16384, 12/65536, 1/16384, 2/16384. Maximum component difference is 64385 integer levels. Pattern/noise behavior remains open.

- **`channel_mixer.sbs/channel_mixer` - Too hard for this pass:** Default 128x128/256x256 and seed-42 cases fail final validation of the dynamic `in_high` parameter. The changed-channel, monochrome case cooks but matches 0/16384 RGBA16 pixels, with differences up to 62259 levels. Parameter endpoint semantics and channel mixing remain open.

- **`chrominance_extract.sbs/chrominance_extract` - Too hard for this pass:** All cases cook. Exact pixels in case order: 10557/16384, 42010/65536, 10557/16384, 10515/16384; maximum component difference 1 integer levels.

- **`clone.sbs/clone` - Too hard for this pass:** All cases cook. Exact pixels in case order: 16384/16384, 65536/65536, 16384/16384, 16155/16384; maximum component difference 1 integer levels.

- **`clouds_1.sbs/clouds_1` - Too hard for this pass:** All cases cook. Exact pixels in case order: 1/16384, 12/65536, 4/16384, 3/16384; maximum component difference 51526 integer levels.

- **`clouds_2.sbs/clouds_2` - Too hard for this pass:** All cases cook. Exact pixels in case order: 1/16384, 2/65536, 1/16384; maximum component difference 42679 integer levels.

- **`color_space_conversion.sbs/acescg_to_linear_srgb` - Too hard for this pass:** All cases cook. Exact pixels in case order: 16384/16384, 65536/65536, 16384/16384, 16383/16384; maximum component difference 1 integer levels.

- **`color_space_conversion.sbs/srgb_to_acescg` - Too hard for this pass:** All cases cook. Exact pixels in case order: 16383/16384, 65534/65536, 16383/16384, 16382/16384; maximum component difference 1 integer levels.

- **`color_to_mask.sbs/color_to_mask` - Too hard for this pass:** All cases cook. Exact pixels in case order: 16384/16384, 65536/65536, 16384/16384, 16293/16384; maximum component difference 255 integer levels.

- **`color_to_mask_v2.sbs/color_to_mask` - Too hard for this pass:** Corrected integer storage after undefined final pixel arithmetic. All four cases now cook. Defaults at 128x128/256x256 and seed 42 match exactly. Changed input-color selection has one one-level difference (16,383/16,384 pixels match).

- **`checker_1.sbs/checker_1` - Too hard for this pass:** Corrected the transposed transform matrix. Defaults at 128x128/256x256, seed 42, and rotated tiling 1/2 now match exactly. Tiling 3 still has minification differences: 14,400/16,384 pixels without rotation and 13,476/16,384 with rotation; maxima 56 and 119 eight-bit levels. A parity mark is withheld.

- **`constant_values.sbs/integer/constant_integer4` - Too hard for this pass:** Zero and mixed signed-vector cases match. At the positive 32-bit integer limit, the reference returns -2,147,483,648 for a component set to 2,147,483,647; MaterialCraft retains 2,147,483,647. No overflow behavior was introduced to imitate this boundary result.

- **`color_temperature_adjustment.sbs/color_temperature_adjustment` - Too hard for this pass:** Fixed the integer option read that previously rejected every cook. Default 128x128/256x256 and seed-42 cases are exact. Temperature 0.25, magenta/green -0.2, and alternate color space at 128x128 have two one-level RGBA8 differences. All four cases cook.

- **`composite.sbs/composite/filters/filter_transformation` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16320/16384, 65280/65536, 16320/16384, 16318/16384; maximum component difference 255 integer levels.

- **`composite.sbs/composite/blends/blend_content` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16288/16384, 65152/65536, 16288/16384, 16288/16384; maximum component difference 255 integer levels.

- **`composite.sbs/composite/blends/blend_content_pre_post_composite` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16288/16384, 65152/65536, 16288/16384, 16287/16384; maximum component difference 255 integer levels.

- **`creased.sbs/creased` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 1/65536, 0/16384, 0/16384; maximum component difference 55711 integer levels.

- **`crop.sbs/crop` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16340/16384, 16384/16384, 65350/65536, 65536/65536, 16340/16384, 16384/16384, 16384/16384, 16384/16384; maximum component difference 1 integer levels.

- **`crop.sbs/crop_grayscale` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16340/16384, 16384/16384, 65350/65536, 65536/65536, 16340/16384, 16384/16384, 16384/16384, 16384/16384; maximum component difference 1 integer levels.

- **`crystal_1.sbs/crystal_1` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 2/16384, 5/65536, 2/16384, 3/16384; maximum component difference 64433 integer levels.

- **`crystal_2.sbs/crystal_2` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 1/16384, 2/65536, 0/16384, 1/16384; maximum component difference 55976 integer levels.

- **`curvature.sbs/curvature` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 31/16384, 660/65536, 31/16384, 20/16384; maximum component difference 962 integer levels. Inputs include an original curved, normalized normal field.

- **`curvature_smooth.sbs/curvature_smooth` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 23/16384, 167/65536, 23/16384, 25/16384; maximum component difference 1602 integer levels. Inputs include an original curved, normalized normal field.

- **`diffusion.sbs/impl/pass_normal` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 16383/16384, 65527/65536, 16383/16384, 1304/16384; maximum component difference 94 integer levels.

- **`diffusion.sbs/impl/pass_RGBA` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 15919/16384, 63936/65536, 15919/16384, 458/16384; maximum component difference 154 integer levels.

- **`diffusion.sbs/impl/pass_gray` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 16178/16384, 64736/65536, 16178/16384, 2192/16384; maximum component difference 104 integer levels.

- **`diffusion.sbs/impl/pass_UV` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 9704/16384, 40270/65536, 9720/16384, 9704/16384; maximum component difference 192 integer levels.

- **`directional_distance.sbs/directional_distance` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 16378/16384, 16384/16384, 65510/65536, 65536/65536, 16378/16384, 16384/16384, 16384/16384, 16384/16384; maximum component difference 1 integer levels. All output bridges were compared.

- **`directionnal_noise.sbs/directionnal_noise` - Too hard for this pass:** All 3 cases cook. Exact pixels by case/output: 0/16384, 0/65536, 0/16384; maximum component difference 60592 integer levels.

- **`dirt_1.sbs/dirt_1` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 79/16384, 334/65536, 55/16384, 100/16384; maximum component difference 65535 integer levels.

- **`dirt_3.sbs/dirt_3` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 6/16384, 546/65536, 2/16384, 5/16384; maximum component difference 64872 integer levels.

- **`dirt_5.sbs/dirt_5` - Too hard for this pass:** All 3 cases cook. Exact pixels by case/output: 2514/16384, 9716/65536, 2587/16384; maximum component difference 65535 integer levels.

- **`dirt_6.sbs/dirt_6` - Too hard for this pass:** All 3 cases cook. Exact pixels by case/output: 51/16384, 188/65536, 60/16384; maximum component difference 64915 integer levels.

- **`dirt_gradient.sbs/dirt_gradient` - Too hard for this pass:** All 3 cases cook. Exact pixels by case/output: 0/16384, 1/65536, 1/16384; maximum component difference 65399 integer levels.

- **`fibers_1.sbs/fibers_1` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 1/65536, 0/16384, 0/16384; maximum component difference 48873 integer levels.

- **`flood_fill.sbs/flood_fill_to_random_color` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum component difference 65071 integer levels.

- **`flood_fill.sbs/flood_fill_to_random_grayscale` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 8/65536, 0/16384, 0/16384; maximum component difference 64844 integer levels.

- **`flood_fill_2.sbs/flood_fill_to_random_color` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum component difference 65071 integer levels.

- **`flood_fill_2.sbs/flood_fill_to_random_grayscale` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum component difference 65169 integer levels.

- **`flood_fill_2.sbs/flood_fill_to_color` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 0/16384; maximum component difference 61956 integer levels.

- **`flood_fill_2.sbs/flood_fill_to_grayscale` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 2/16384; maximum component difference 5503 integer levels.

- **`fluid.sbs/fluid` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum component difference 65454 integer levels.

- **`fractal_sum_base.sbs/fractal_sum_base` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 3/16384, 8/65536, 5/16384, 5/16384; maximum component difference 10237 integer levels.

- **`fur_1.sbs/fur_1` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 23/16384, 193/65536, 30/16384, 25/16384; maximum component difference 65535 integer levels.

- **`fur_2.sbs/fur_2` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum component difference 65523 integer levels.

- **`fur_3.sbs/fur_3` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 1/65536, 0/16384, 0/16384; maximum component difference 65535 integer levels.

- **`fur_4.sbs/fur_4` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 1134/16384, 4416/65536, 1205/16384, 1200/16384; maximum component difference 65535 integer levels.

- **`gaussian_1.sbs/gaussian_1` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum component difference 40180 integer levels.

- **`gaussian_2.sbs/gaussian_2` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum component difference 23969 integer levels.

- **`gaussian_spots_1.sbs/gaussian_spots_1` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 4/16384, 15/65536, 7/16384, 4/16384; maximum component difference 63216 integer levels.

- **`gaussian_spots_2.sbs/gaussian_spots_2` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 3/16384, 16/65536, 2/16384, 3/16384; maximum component difference 65535 integer levels.

- **`gradient_axial.sbs/gradient_axial` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 16379/16384; maximum component difference 1 integer levels.

- **`gradient_axial.sbs/gradient_axial_reflected` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 16366/16384; maximum component difference 1 integer levels.

- **`gradient_circular.sbs/gradient_circular` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 5531/16384, 21951/65536, 5531/16384, 5591/16384; maximum component difference 4 integer levels.

- **`gradient_linear_1.sbs/gradient_linear_1` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 15456/16384; maximum component difference 24576 integer levels.

- **`gradient_linear_2.sbs/gradient_linear_2` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 0/16384; maximum component difference 1984 integer levels.

- **`gradient_linear_3.sbs/gradient_linear_3` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 15495/16384, 61975/65536, 15495/16384, 4096/16384; maximum component difference 1830 integer levels.

- **`gradient_radial.sbs/gradient_radial` - Too hard for this pass:** Corrected integer storage so the changed shape/points case cooks instead of failing on a non-finite pixel. All four cases now cook and match 114,624/114,688 sixteen-bit pixels in total; 64 one-level differences remain.

- **`hald_clut.sbs/hald_clut` - Too hard for this pass:** Exact pixels by case/output: 16118/16384, 64478/65536, 16118/16384, 15905/16384; maximum stored-component difference 1.

- **`hbao.sbs/hbao_mip` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 22/16384; maximum stored-component difference 24094.

- **`hbao_2.sbs/hbao_subsample` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 16377/16384; maximum stored-component difference 32.

- **`hbao_2.sbs/hbao_mip` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 143/16384; maximum stored-component difference 94.

- **`hdr_merge.sbs/average_luminance_merge` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 1/16384; maximum stored-component difference 26228.

- **`hdr_range_viewer.sbs/hdr_range_viewer_grayscale` - Too hard for this pass:** Exact pixels by case/output: 14532/16384, 58173/65536, 14532/16384, 11670/16384; maximum stored-component difference 1.

- **`height_blend.sbs/height_blend` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 16384/16384, 65536/65536, 65536/65536, 16384/16384, 16384/16384, 7573/16384, 8768/16384; maximum stored-component difference 23426.

- **`height_to_normal_world_units.sbs/height_to_normal_world_units` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 10198/16384; maximum stored-component difference 50.

- **`height_to_normal_world_units.sbs/height_to_normal_world_units_2` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 10198/16384; maximum stored-component difference 49.

- **`highpass.sbs/highpass` - Too hard for this pass:** Exact pixels by case/output: 223/16384, 556/65536, 223/16384, 6967/16384; maximum stored-component difference 59.

- **`highpass.sbs/highpass_grayscale` - Too hard for this pass:** Exact pixels by case/output: 223/16384, 3712/65536, 223/16384, 9926/16384; maximum stored-component difference 66.

- **`histogram_scan.sbs/histogram_scan` - Too hard for this pass:** Exact pixels by case/output: 16319/16384, 65280/65536, 16319/16384, 16384/16384; maximum stored-component difference 255.

- **`histogram_shift.sbs/histogram_shift` - Too hard for this pass:** Exact pixels by case/output: 16319/16384, 65280/65536, 16319/16384, 15935/16384; maximum stored-component difference 1.

- **`histogram_tools.sbs/histogram_compute` - Too hard for this pass:** Exact pixels by case/output: 4/256, 54/256, 1/256, 256/256, 4/256, 54/256, 1/128, 0/128; maximum stored-component difference 7.

- **`material_transforms.sbs/material_transforms` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 16384/16384, 16384/16384, 16384/16384, 16384/16384, 65536/65536, 65536/65536, 65536/65536, 65536/65536, 65536/65536, 16384/16384, 16384/16384, 16384/16384, 16384/16384, 16384/16384, 14559/16384, 13711/16384, 15603/16384, 15493/16384, 12192/16384; maximum stored-component difference 1.

- **`me_water_level.sbs/scratches_2` - Too hard for this pass:** Exact pixels by case/output: 3/16384, 29/65536, 2/16384, 9/16384; maximum stored-component difference 103.

- **`mean_of_least_variance.sbs/mlv_color_iteration` - Too hard for this pass:** Exact pixels by case/output: 4/16384, 0/65536, 4/16384, 658/16384; maximum stored-component difference 167.

- **`mean_of_least_variance.sbs/mlv_grayscale_iteration` - Too hard for this pass:** Exact pixels by case/output: 6292/16384, 33605/65536, 6292/16384, 1762/16384; maximum stored-component difference 88.

- **`median_filter.sbs/median_filter_color` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 12077/16384; maximum stored-component difference 236.

- **`mesh_1.sbs/mesh_1` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 16/16384; maximum stored-component difference 65535.

- **`mesh_2.sbs/mesh_2` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 160/16384; maximum stored-component difference 65535.

- **`mesh_adaptive_material_blender.sbs/mesh_material_blender_8_inputs` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 16384/16384, 89/16384, 65536/65536, 65536/65536, 345/65536, 16384/16384, 16384/16384, 89/16384, 3157/16384, 14385/16384, 76/16384; maximum stored-component difference 255.

- **`mesh_adaptive_material_blender.sbs/mesh_material_blender_4_inputs` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 16384/16384, 86/16384, 65536/65536, 65536/65536, 358/65536, 16384/16384, 16384/16384, 86/16384, 1595/16384, 14388/16384, 74/16384; maximum stored-component difference 255.

- **`mesh_adaptive_material_blender.sbs/mesh_material_blender_2_inputs` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 16384/16384, 20/16384, 65536/65536, 65536/65536, 81/65536, 16384/16384, 16384/16384, 20/16384, 231/16384, 2761/16384, 12/16384; maximum stored-component difference 255.

- **`messy_fibers_1.sbs/messy_fibers_1` - Too hard for this pass:** Exact pixels by case/output: 173/16384, 1893/65536, 145/16384, 104/16384; maximum stored-component difference 65535.

- **`messy_fibers_2.sbs/messy_fibers_2` - Too hard for this pass:** Exact pixels by case/output: 34/16384, 165/65536, 53/16384, 46/16384; maximum stored-component difference 65535.

- **`messy_fibers_3.sbs/messy_fibers_3` - Too hard for this pass:** Exact pixels by case/output: 6383/16384, 25656/65536, 6157/16384, 6471/16384; maximum stored-component difference 255.

- **`messy_fibers_4.sbs/messy_fibers_4` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384; maximum stored-component difference 64037.

- **`microscope_view.sbs/microscope_view` - Too hard for this pass:** Exact pixels by case/output: 1/16384, 23/65536, 0/16384, 1/16384; maximum stored-component difference 65535.

- **`moisture_noise.sbs/moisture_noise` - Too hard for this pass:** Exact pixels by case/output: 7/16384, 34/65536, 3/16384, 10/16384; maximum stored-component difference 45639.

- **`mosaic.sbs/mosaic` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 190.

- **`mosaic.sbs/mosaic_grayscale` - Too hard for this pass:** Exact pixels by case/output: 198/16384, 630/65536, 198/16384, 258/16384; maximum stored-component difference 143.

- **`multi_directional_warp.sbs/multi_directional_warp_grayscale` - Too hard for this pass:** Exact pixels by case/output: 121/16384, 3395/65536, 121/16384, 9394/16384; maximum stored-component difference 158.

- **`multi_directional_warp.sbs/multi_directional_warp_color` - Too hard for this pass:** Exact pixels by case/output: 66/16384, 286/65536, 66/16384, 1215/16384; maximum stored-component difference 250.

- **`multiangle_to_albedo.sbs/multiangle_to_albedo` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65534/65536, 16384/16384, 16384/16384; maximum stored-component difference 1.

- **`noise_anisotropic_noise.sbs/anisotropic_noise` - Too hard for this pass:** Exact pixels by case/output: 5/16384, 25/65536, 8/16384, 3612/16384; maximum stored-component difference 65214.

- **`noise_anisotropic_noise_v2.sbs/anisotropic_noise_v2` - Too hard for this pass:** Exact pixels by case/output: 5422/16384, 14285/65536, 5547/16384, 13676/16384; maximum stored-component difference 15.

- **`min_max.sbs/min_max` - Too hard for this pass:** All four cases cook and seven of eight scalar outputs match exactly. The alternate-input minimum differs by 5.9604645e-8 (0.1254901885986328 versus 0.12549012899398804).

- **`make_it_tile_patch.sbs/make_it_tile_patch` - Too hard for this pass:** Exact pixels by case/output: 596/16384, 2423/65536, 596/16384, 41/16384; maximum stored-component difference 255.

- **`make_it_tile_patch_grayscale.sbs/make_it_tile_patch_grayscale` - Too hard for this pass:** Exact pixels by case/output: 937/16384, 5840/65536, 937/16384, 90/16384; maximum stored-component difference 252.

- **`non_square_transform.sbs/non_square_transform` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 13939/16384; maximum stored-component difference 2.

- **`non_square_transform.sbs/non_square_transform_greyscale` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 15078/16384; maximum stored-component difference 2.

- **`non_uniform_blur.sbs/non_uniform_blur_sample` - Too hard for this pass:** Exact pixels by case/output: 8397/16384, 25549/65536, 8397/16384, 3980/16384; maximum stored-component difference 4.

- **`non_uniform_blur.sbs/non_uniform_blur_grayscale_sample` - Too hard for this pass:** Exact pixels by case/output: 13146/16384, 52479/65536, 13146/16384, 8120/16384; maximum stored-component difference 4.

- **`non_uniform_rotation.sbs/non_uniform_rotation_grayscale` - Too hard for this pass:** Exact pixels by case/output: 15829/16384, 63348/65536, 15829/16384, 15865/16384; maximum stored-component difference 1.

- **`non_uniform_rotation.sbs/non_uniform_rotation_color` - Too hard for this pass:** Exact pixels by case/output: 15888/16384, 63498/65536, 15888/16384, 15885/16384; maximum stored-component difference 2.

- **`normal_normalise.sbs/normal_normalise` - Too hard for this pass:** Exact pixels by case/output: 16356/16384, 65420/65536, 16356/16384, 16356/16384; maximum stored-component difference 1.

- **`normal_sobel.sbs/normal_sobel` - Too hard for this pass:** Exact pixels by case/output: 10198/16384, 40960/65536, 10198/16384, 10198/16384; maximum stored-component difference 2399.

- **`normal_sobel.sbs/normal_sobel_2` - Too hard for this pass:** Exact pixels by case/output: 10198/16384, 40960/65536, 10198/16384, 10198/16384; maximum stored-component difference 9.

- **`normal_to_height_2.sbs/normal_to_height` - Too hard for this pass:** Exact pixels by case/output: 10956/16384, 44049/65536, 10956/16384, 11389/16384; maximum stored-component difference 19614.

- **`normal_to_height_hq.sbs/base_algorithm_height_distance` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 14374/16384; maximum stored-component difference 347.

- **`normal_transform.sbs/normal_transform` - Too hard for this pass:** Exact pixels by case/output: 16356/16384, 65420/65536, 16356/16384, 16356/16384; maximum stored-component difference 1.

- **`panorama_gradients.sbs/panorama_gradient_linear` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 2.384186e-07.

- **`panorama_gradients.sbs/panorama_gradient_2_points` - Too hard for this pass:** Exact pixels by case/output: 445/16384, 1667/65536, 445/16384, 137/16384; maximum stored-component difference 4.172325e-07.

- **`panorama_nadir_extract.sbs/nadir_extract` - Too hard for this pass:** Exact pixels by case/output: 354/16384, 337/65536, 354/16384, 3312/16384; maximum stored-component difference 0.07189722.

- **`panorama_nadir_patch.sbs/panorama_nadir_patch` - Too hard for this pass:** Exact pixels by case/output: 15109/16384, 59874/65536, 15109/16384, 15569/16384; maximum stored-component difference 0.01915002.

- **`panorama_position.sbs/panorama_3d_position` - Too hard for this pass:** Exact pixels by case/output: 16183/16384, 64379/65536, 16183/16384, 16183/16384; maximum stored-component difference 1.

- **`panorama_rotation.sbs/panorama_rotation` - Too hard for this pass:** Exact pixels by case/output: 13885/16384, 48478/65536, 13885/16384, 11046/16384; maximum stored-component difference 7.

- **`panorama_shape.sbs/panorama_shape` - Too hard for this pass:** Exact pixels by case/output: 15802/16384, 63195/65536, 15802/16384, 15177/16384; maximum stored-component difference 9.202957e-05.

- **`panorama_shape.sbs/panorama_shape_2` - Too hard for this pass:** Exact pixels by case/output: 15374/16384, 61494/65536, 15374/16384, 10926/16384; maximum stored-component difference 6.747246e-05.

- **`panorama_shape.sbs/panorama_sphere_light` - Too hard for this pass:** Exact pixels by case/output: 16356/16384, 65425/65536, 16356/16384, 16269/16384; maximum stored-component difference 0.001916409.

- **`panorama_shape.sbs/panorama_plane_light` - Too hard for this pass:** Exact pixels by case/output: 15918/16384, 63690/65536, 15918/16384, 16384/16384; maximum stored-component difference 4.464388e-05.

- **`panorama_shape.sbs/panorama_spot` - Too hard for this pass:** Exact pixels by case/output: 16378/16384, 65508/65536, 16378/16384, 11303/16384; maximum stored-component difference 0.1226807.

- **`panorama_straighten_horizon.sbs/panorama_straighten_horizon` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 0/16384; maximum stored-component difference 250.

- **`paper_path.sbs/paper_path` - Too hard for this pass:** Exact pixels by case/output: 7664/16384, 27377/65536, 7766/16384, 6530/16384; maximum stored-component difference 65535.

- **`parquet_1.sbs/parquet_1` - Too hard for this pass:** Exact pixels by case/output: 16240/16384, 64996/65536, 16288/16384, 16096/16384; maximum stored-component difference 255.

- **`panorama_physical_sun_sky.sbs/Impl/GetOneParam` - Too hard for this pass:** Typed outputs match in 0/4 comparisons. output: [3.0352942943573, 3.1215686798095703, 3.1450982093811035] versus [3.0352942943573, 3.1215689182281494, 3.1450982093811035]; output: [2.8705883026123047, 3.3490195274353027, 2.462745428085327] versus [2.870588541030884, 3.349020004272461, 2.462745428085327]; output: [3.0352942943573, 3.1215686798095703, 3.1450982093811035] versus [3.0352942943573, 3.1215689182281494, 3.1450982093811035]; output: [0.9883579015731812, 0.8544117212295532, 1.0778186321258545] versus [0.9883579015731812, 0.8544118404388428, 1.0778186321258545].

- **`panorama_shape.sbs/panorama_line_light` - Too hard for this pass:** Exact pixels by case/output: 15172/16384, 60708/65536, 15172/16384, 14428/16384; maximum stored-component difference 0.0001544952.

- **`noise_bnw_spots_1.sbs/bnw_spots_1` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget. Exact pixels by case/output: 1/16384, 0/65536, 1/16384; maximum stored-component difference 64390.

- **`noise_bnw_spots_1_v2.sbs/bnw_spots_1` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget. Exact pixels by case/output: 131/16384, 541/65536, 141/16384; maximum stored-component difference 205.

- **`noise_bnw_spots_3.sbs/bnw_spots_3` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget. Exact pixels by case/output: 0/16384, 1/65536, 0/16384; maximum stored-component difference 62911.

- **`noise_bnw_spots_3_v2.sbs/bnw_spots_3` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget. Exact pixels by case/output: 49/16384, 162/65536, 48/16384; maximum stored-component difference 227.

- **`noise_cells_1.sbs/cells_1` - Too hard for this pass:** Exact pixels by case/output: 2/16384, 5/65536, 0/16384, 1/16384; maximum stored-component difference 56592.

- **`noise_cells_1_v2.sbs/cells_1` - Too hard for this pass:** Exact pixels by case/output: 236/16384, 1029/65536, 261/16384, 0/16384; maximum stored-component difference 223.

- **`noise_cells_4.sbs/cells_4` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 23/16384; maximum stored-component difference 65462.

- **`noise_cells_4_v2.sbs/cells_4` - Too hard for this pass:** Exact pixels by case/output: 98/16384, 379/65536, 33/16384, 1970/16384; maximum stored-component difference 253.

- **`noise_clouds_1.sbs/clouds_1` - Too hard for this pass:** Exact pixels by case/output: 2/16384, 5/65536, 0/16384, 0/16384; maximum stored-component difference 48017.

- **`noise_clouds_1_v2.sbs/clouds_1` - Too hard for this pass:** Exact pixels by case/output: 229/16384, 792/65536, 186/16384, 242/16384; maximum stored-component difference 156.

- **`noise_clouds_2.sbs/clouds_2` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 4/65536, 2/16384, 0/16384; maximum stored-component difference 44350.

- **`noise_clouds_2_v2.sbs/clouds_2` - Too hard for this pass:** Exact pixels by case/output: 201/16384, 832/65536, 211/16384, 245/16384; maximum stored-component difference 123.

- **`noise_creased.sbs/creased` - Too hard for this pass:** Exact pixels by case/output: 1/16384, 0/65536, 2/16384, 1/16384; maximum stored-component difference 42493.

- **`noise_crystal_1.sbs/crystal_1` - Too hard for this pass:** Exact pixels by case/output: 1/16384, 2/65536, 2/16384, 1/16384; maximum stored-component difference 61334.

- **`noise_crystal_2.sbs/crystal_2` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 3/65536, 0/16384, 1/16384; maximum stored-component difference 51894.

- **`noise_directional_noise_1.sbs/directional_noise_1` - Too hard for this pass:** Exact pixels by case/output: 1/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 64802.

- **`noise_directional_noise_1_v2.sbs/directional_noise_1` - Too hard for this pass:** Exact pixels by case/output: 8/16384, 54/65536, 0/16384, 2/16384; maximum stored-component difference 240.

- **`noise_directional_noise_2.sbs/directional_noise_2` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget. Exact pixels by case/output: 0/16384, 0/65536, 0/16384; maximum stored-component difference 65535.

- **`noise_directional_noise_3.sbs/directional_noise_3` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget.

- **`noise_directional_noise_4.sbs/directional_noise_4` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 1/65536, 0/16384, 0/16384; maximum stored-component difference 47884.

- **`noise_directional_noise_4_v2.sbs/directional_noise_4` - Too hard for this pass:** Exact pixels by case/output: 161/16384, 713/65536, 161/16384, 148/16384; maximum stored-component difference 207.

- **`noise_directional_scratches.sbs/directional_scratches` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 5334/16384; maximum stored-component difference 62981.

- **`noise_directional_scratches_v2.sbs/directional_scratches` - Too hard for this pass:** Exact pixels by case/output: 62/16384, 250/65536, 74/16384, 4096/16384; maximum stored-component difference 239.

- **`noise_dirt_1.sbs/dirt_1` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget. Exact pixels by case/output: 1037/16384, 4038/65536, 992/16384; maximum stored-component difference 65535.

- **`noise_fluid.sbs/fluid` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 63791.

- **`noise_fractal_sum_base.sbs/fractal_sum_base` - Too hard for this pass:** Exact pixels by case/output: 2/16384, 6/65536, 1/16384, 0/16384; maximum stored-component difference 21118.

- **`noise_fractal_sum_base.sbs/fractal_sum_base_2` - Too hard for this pass:** Exact pixels by case/output: 2/16384, 6/65536, 1/16384, 0/16384; maximum stored-component difference 20137.

- **`noise_fractal_sum_base_v2.sbs/noise_upscale` - Too hard for this pass:** Exact pixels by case/output: 389/16384, 1536/65536, 389/16384, 279/16384; maximum stored-component difference 15.

- **`noise_fractal_sum_base_v2.sbs/noise_blend` - Too hard for this pass:** Exact pixels by case/output: 134/16384, 668/65536, 134/16384, 75/16384; maximum stored-component difference 100.

- **`noise_fur_1.sbs/fur_1` - Too hard for this pass:** Exact pixels by case/output: 13/16384, 75/65536, 16/16384, 10/16384; maximum stored-component difference 65535.

- **`noise_fur_2.sbs/fur_2` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 65530.

- **`noise_fur_3.sbs/fur_3` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 65529.

- **`noise_gaussian_noise.sbs/gaussian_noise` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 64187.

- **`noise_gaussian_noise_v2.sbs/gaussian_noise` - Too hard for this pass:** Exact pixels by case/output: 4/16384, 16/65536, 6/16384, 0/16384; maximum stored-component difference 251.

- **`noise_gaussian_spots_1.sbs/gaussian_spots_1` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 1/65536, 1/16384, 0/16384; maximum stored-component difference 58938.

- **`noise_gaussian_spots_1_v2.sbs/gaussian_spots_1` - Too hard for this pass:** Exact pixels by case/output: 99/16384, 434/65536, 118/16384, 121/16384; maximum stored-component difference 235.

- **`noise_gaussian_spots_2.sbs/gaussian_spots_2` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 1/65536, 0/16384, 0/16384; maximum stored-component difference 45111.

- **`noise_gaussian_spots_2_v2.sbs/gaussian_spots_2` - Too hard for this pass:** Exact pixels by case/output: 21/16384, 257/65536, 33/16384, 62/16384; maximum stored-component difference 255.

- **`noise_liquid.sbs/liquid` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 61102.

- **`noise_messy_fibers_1.sbs/messy_fibers_1` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 1/65536, 0/16384, 0/16384; maximum stored-component difference 65295.

- **`noise_messy_fibers_1_v2.sbs/messy_fibers_1` - Too hard for this pass:** Exact pixels by case/output: 37/16384, 153/65536, 58/16384, 46/16384; maximum stored-component difference 255.

- **`noise_messy_fibers_2.sbs/messy_fibers_2` - Too hard for this pass:** Exact pixels by case/output: 55/16384, 282/65536, 116/16384, 87/16384; maximum stored-component difference 65535.

- **`noise_messy_fibers_2_v2.sbs/messy_fibers_2` - Too hard for this pass:** Exact pixels by case/output: 163/16384, 686/65536, 95/16384, 213/16384; maximum stored-component difference 255.

- **`noise_messy_fibers_3.sbs/messy_fibers_3` - Too hard for this pass:** Exact pixels by case/output: 78/16384, 304/65536, 77/16384, 63/16384; maximum stored-component difference 65535.

- **`noise_messy_fibers_3_v2.sbs/messy_fibers_3` - Too hard for this pass:** Exact pixels by case/output: 128/16384, 515/65536, 138/16384, 3/16384; maximum stored-component difference 255.

- **`noise_microscope_view.sbs/microscope_view` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 65528.

- **`noise_perlin_noise.sbs/perlin_noise` - Too hard for this pass:** Exact pixels by case/output: 2/16384, 6/65536, 4/16384, 4/16384; maximum stored-component difference 44958.

- **`noise_perlin_noise_v2.sbs/perlin_noise` - Too hard for this pass:** Exact pixels by case/output: 107/16384, 448/65536, 130/16384, 168/16384; maximum stored-component difference 215.

- **`noise_plasma.sbs/plasma` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 3/65536, 1/16384, 0/16384; maximum stored-component difference 41315.

- **`noise_upscale_1.sbs/noise_upscale_1` - Too hard for this pass:** Exact pixels by case/output: 47496/65536, 189800/262144, 47496/65536, 46630/65536; maximum stored-component difference 42.

- **`noise_upscale_2.sbs/noise_upscale_2` - Too hard for this pass:** Exact pixels by case/output: 44458/65536, 177124/262144, 44458/65536, 44586/65536; maximum stored-component difference 23.

- **`noise_upscale_3.sbs/noise_upscale_3` - Too hard for this pass:** Exact pixels by case/output: 54720/65536, 223780/262144, 54720/65536, 55992/65536; maximum stored-component difference 2.

- **`pattern_alveolus.sbs/alveolus` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 13568/16384; maximum stored-component difference 65535.

- **`pattern_arc_pavement.sbs/arc_pavement` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65532/65536, 16384/16384, 12457/16384; maximum stored-component difference 65535.

- **`pattern_brick_1.sbs/brick_1` - Too hard for this pass:** Exact pixels by case/output: 14592/16384, 58368/65536, 14592/16384, 6528/16384; maximum stored-component difference 65535.

- **`pattern_brick_2.sbs/brick_2` - Too hard for this pass:** Exact pixels by case/output: 15040/16384, 61056/65536, 15040/16384, 6656/16384; maximum stored-component difference 65535.

- **`pattern_cube_3d.sbs/cube_3d` - Too hard for this pass:** Exact pixels by case/output: 16375/16384, 65369/65536, 16375/16384, 16328/16384; maximum stored-component difference 1.

- **`pattern_cube_3d.sbs/cube_3d_gbuffers` - Too hard for this pass:** Exact pixels by case/output: 16350/16384, 16384/16384, 16375/16384, 16352/16384, 65244/65536, 65536/65536, 65369/65536, 65322/65536, 16350/16384, 16384/16384, 16375/16384, 16352/16384, 16107/16384, 16384/16384, 16328/16384, 16253/16384; maximum stored-component difference 1.

- **`pattern_fibers_1.sbs/fibers_1` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 1/65536, 0/16384, 0/16384; maximum stored-component difference 48873.

- **`pattern_fibers_2.sbs/fibers_2` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 2/65536, 0/16384, 0/16384; maximum stored-component difference 62612.

- **`pattern_gaussian_1.sbs/gaussian_1` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 40180.

- **`pattern_gaussian_2.sbs/gaussian_2` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 23969.

- **`pattern_mesh_1.sbs/mesh_1` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 16/16384; maximum stored-component difference 65535.

- **`pattern_mesh_2.sbs/mesh_2` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 160/16384; maximum stored-component difference 65535.

- **`pattern_polygon_2.sbs/polygon_2` - Too hard for this pass:** Exact pixels by case/output: 14203/16384, 56174/65536, 14203/16384, 13032/16384; maximum stored-component difference 4.

- **`pattern_shape.sbs/shape_base` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 16217/16384; maximum stored-component difference 1.

- **`pattern_shape.sbs/hemisphere` - Too hard for this pass:** Exact pixels by case/output: 16368/16384, 65536/65536, 16368/16384, 16361/16384; maximum stored-component difference 1.

- **`pattern_splatter_circular.sbs/splatter_circular` - Too hard for this pass:** Exact pixels by case/output: 15941/16384, 63805/65536, 15941/16384, 15401/16384; maximum stored-component difference 57311.

- **`pattern_splatter_circular.sbs/splatter_circular_color` - Too hard for this pass:** Exact pixels by case/output: 15529/16384, 62023/65536, 15529/16384, 15401/16384; maximum stored-component difference 65535.

- **`pattern_stripes.sbs/stripes` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 45056/65536, 16384/16384, 16384/16384; maximum stored-component difference 9557.

- **`pattern_tile_generator.sbs/tile_generator` - Too hard for this pass:** Exact pixels by case/output: 13097/16384, 52436/65536, 13097/16384, 16384/16384; maximum stored-component difference 18088.

- **`pattern_tile_generator.sbs/tile_generator_color` - Too hard for this pass:** Exact pixels by case/output: 10819/16384, 41641/65536, 10819/16384, 16384/16384; maximum stored-component difference 23978.

- **`pattern_tile_random.sbs/tile_random` - Too hard for this pass:** Exact pixels by case/output: 13532/16384, 53254/65536, 13610/16384, 11762/16384; maximum stored-component difference 57311.

- **`pattern_tile_random.sbs/tile_random_color` - Too hard for this pass:** Exact pixels by case/output: 13532/16384, 53254/65536, 13610/16384, 11762/16384; maximum stored-component difference 65535.

- **`pattern_tile_random_2.sbs/tile_random_2` - Too hard for this pass:** Exact pixels by case/output: 16378/16384, 4788/16384, 65497/65536, 18310/65536, 16368/16384, 5261/16384, 16353/16384, 12087/16384; maximum stored-component difference 1.

- **`pattern_weave_1.sbs/weave_1` - Too hard for this pass:** Exact pixels by case/output: 928/16384, 3872/65536, 928/16384, 848/16384; maximum stored-component difference 65535.

- **`pattern_weave_2.sbs/weave_2` - Too hard for this pass:** Exact pixels by case/output: 256/16384, 2304/65536, 256/16384, 39/16384; maximum stored-component difference 65535.

- **`pbr_render.sbs/PBR_render_mapping_color` - Too hard for this pass:** Exact pixels by case/output: 16223/16384, 65081/65536, 16223/16384, 16123/16384; maximum stored-component difference 1.

- **`pbr_render.sbs/PBR_render_mapping_grayscale` - Too hard for this pass:** Exact pixels by case/output: 15835/16384, 63262/65536, 15835/16384, 16170/16384; maximum stored-component difference 1.

- **`pbr_render.sbs/environment_map_rescale` - Too hard for this pass:** Exact pixels by case/output: 1/16384, 2/65536, 1/16384, 0/16384; maximum stored-component difference 2.551079e-05.

- **`pbr_render.sbs/position_to_normal` - Too hard for this pass:** Exact pixels by case/output: 3119/16384, 4234/65536, 3119/16384, 1153/16384; maximum stored-component difference 0.1367188.

- **`pbr_render.sbs/chroma_shift` - Too hard for this pass:** Exact pixels by case/output: 16083/16384, 64304/65536, 16083/16384, 15570/16384; maximum stored-component difference 1.

- **`pbr_render.sbs/get_max_coords/get_max_coord` - Too hard for this pass:** Cooks report: Get Float2 (1379828382): undefined variable `$pos` in this context.

- **`pbr_render.sbs/ibl_specular_contrib_anisotropic/panoramic_lod` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 0.98973.

- **`pbr_render2.sbs/PBR_render_mapping_color` - Too hard for this pass:** Exact pixels by case/output: 16223/16384, 65081/65536, 16223/16384, 16123/16384; maximum stored-component difference 1.

- **`pbr_render2.sbs/PBR_render_mapping_grayscale` - Too hard for this pass:** Exact pixels by case/output: 15835/16384, 63262/65536, 15835/16384, 16170/16384; maximum stored-component difference 1.

- **`pbr_render2.sbs/environment_map_rescale` - Too hard for this pass:** Exact pixels by case/output: 1/16384, 2/65536, 1/16384, 0/16384; maximum stored-component difference 2.551079e-05.

- **`pbr_render2.sbs/position_to_normal` - Too hard for this pass:** Exact pixels by case/output: 8615/16384, 31600/65536, 8615/16384, 6737/16384; maximum stored-component difference 0.0004882812.

- **`pbr_render2.sbs/chroma_shift` - Too hard for this pass:** Exact pixels by case/output: 16083/16384, 64304/65536, 16083/16384, 15570/16384; maximum stored-component difference 1.

- **`pbr_render2.sbs/get_max_coords/get_max_coord` - Too hard for this pass:** Cooks report: Get Float2 (1379828382): undefined variable `$pos` in this context.

- **`pbr_render2.sbs/ibl_specular_contrib_anisotropic/panoramic_lod` - Too hard for this pass:** Exact pixels by case/output: 42/16384, 1690/65536, 42/16384, 741/16384; maximum stored-component difference 155.

- **`pbr_render2.sbs/lod` - Too hard for this pass:** Exact pixels by case/output: 12328/16384, 49408/65536, 12328/16384, 12632/16384; maximum stored-component difference 127.

- **`pbr_render_ortho.sbs/mipmap_generator` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 2.687117.

- **`pbr_render_ortho.sbs/rotate_environment` - Too hard for this pass:** Exact pixels by case/output: 13885/16384, 48478/65536, 13885/16384, 11791/16384; maximum stored-component difference 7.

- **`pbr_render_ortho.sbs/blue_noise` - Too hard for this pass:** Exact pixels by case/output: 133/16384, 489/65536, 133/16384, 144/16384; maximum stored-component difference 3.874302e-07.

- **`pbr_render_ortho.sbs/environment_map_rescale` - Too hard for this pass:** Exact pixels by case/output: 1/16384, 2/65536, 1/16384, 0/16384; maximum stored-component difference 2.551079e-05.

- **`pbr_render_ortho.sbs/vector_shadow_pass` - Too hard for this pass:** Exact pixels by case/output: 14758/16384, 60408/65536, 14758/16384, 14762/16384; maximum stored-component difference 1.149848.

- **`noise_directional_noise_2_v2.sbs/directional_noise_2` - Too hard for this pass:** The release CPU comparison did not finish its four-case batch within the 180-second per-graph allowance. Reference captures completed; no pixel-parity claim is made for this timed-out run.

- **`noise_directional_noise_3_v2.sbs/directional_noise_3` - Too hard for this pass:** The release CPU comparison did not finish its four-case batch within the 180-second per-graph allowance. Reference captures completed; no pixel-parity claim is made for this timed-out run.

- **`noise_dirt_1_v2.sbs/dirt_1` - Too hard for this pass:** The release CPU comparison did not finish its four-case batch within the 180-second per-graph allowance. Reference captures completed; no pixel-parity claim is made for this timed-out run.

- **`noise_moisture_noise.sbs/moisture_noise` - Too hard for this pass:** The release CPU comparison did not finish its four-case batch within the 180-second per-graph allowance. Reference captures completed; no pixel-parity claim is made for this timed-out run.

- **`noise_moisture_noise_1_v2.sbs/moisture_noise_1` - Too hard for this pass:** The release CPU comparison did not finish its four-case batch within the 180-second per-graph allowance. Reference captures completed; no pixel-parity claim is made for this timed-out run.

- **`noise_moisture_noise_2.sbs/moisture_noise_2` - Too hard for this pass:** The release CPU comparison did not finish its four-case batch within the 180-second per-graph allowance. Reference captures completed; no pixel-parity claim is made for this timed-out run.

- **`pattern_triangle_grid.sbs/triangle_grid_base` - Too hard for this pass:** The release CPU comparison did not finish its four-case batch within the 180-second per-graph allowance. Reference captures completed; no pixel-parity claim is made for this timed-out run.

- **`noise_voronoi.sbs/3d_voronoi_fractal` - Too hard for this pass:** All four 16x16/32x32 cases now cook after the unassigned-counter fix. Exact pixels by case/output: 232/256, 950/1024, 245/256, 253/256; maximum stored-component difference 75.

- **`noise_voronoi.sbs/3d_voronoi` - Too hard for this pass:** All four 16x16/32x32 cases now cook after the unassigned-counter fix. Exact pixels by case/output: 238/256, 968/1024, 240/256, 255/256; maximum stored-component difference 1.

- **`noise_voronoi.sbs/voronoi` - Too hard for this pass:** All four 16x16/32x32 cases now cook after the unassigned-counter fix. Exact pixels by case/output: 225/256, 888/1024, 175/256, 185/256; maximum stored-component difference 178.

- **`noise_voronoi.sbs/voronoi_fractal` - Too hard for this pass:** All four 16x16/32x32 cases now cook after the unassigned-counter fix. Exact pixels by case/output: 124/256, 502/1024, 87/256, 179/256; maximum stored-component difference 36916.

- **`pattern_tile_sampler.sbs/arrow` - Too hard for this pass:** The importer drops the explicit 512x512 graph-level output size. Manually aligning both graph sizes still leaves rendering differences. These three captures use 512x512 output. Exact pixels by case/output: 259850/262144, 259850/262144, 259850/262144; maximum stored-component difference 155.

- **`pbr_render_ortho.sbs/ibl_shadow_pass` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 31.45882.

- **`composite.sbs/composite/blends/blend_matte` - Too hard for this pass:** Exact pixels by case/output: 90/16384, 359/65536, 90/16384, 118/16384; maximum stored-component difference 196.

- **`noise_dirt_2.sbs/dirt_2` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget. Exact pixels by case/output: 14450/16384, 57939/65536, 14517/16384; maximum stored-component difference 65535.

- **`noise_dirt_3.sbs/dirt_3` - Too hard for this pass:** Exact pixels by case/output: 375/16384, 0/65536, 326/16384, 259/16384; maximum stored-component difference 65535.

- **`noise_dirt_3_v2.sbs/dirt_3` - Too hard for this pass:** Exact pixels by case/output: 363/16384, 0/65536, 427/16384, 94/16384; maximum stored-component difference 255.

- **`noise_dirt_5.sbs/dirt_5` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget. Exact pixels by case/output: 2184/16384, 6519/65536, 2184/16384; maximum stored-component difference 61798.

- **`noise_dirt_5_v2.sbs/dirt_5` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget. Exact pixels by case/output: 1988/16384, 5879/65536, 1997/16384; maximum stored-component difference 218.

- **`noise_dirt_gradient.sbs/dirt_gradient` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 2/65536, 0/16384, 0/16384; maximum stored-component difference 62840.

- **`perlin_noise_zoom.sbs/perlin_noise_zoom` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 62227.

- **`pre_multiply_straight.sbs/premult_to_straight` - Too hard for this pass:** Exact pixels by case/output: 16376/16384, 65504/65536, 16376/16384, 16381/16384; maximum stored-component difference 255.

- **`quad_transform.sbs/quad_transform` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 15477/16384; maximum stored-component difference 1.

- **`quad_transform.sbs/quad_transform_grayscale` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 15846/16384; maximum stored-component difference 1.

- **`quantize_color.sbs/modify_color_palette_sub` - Too hard for this pass:** Exact pixels by case/output: 240/256, 240/256, 240/256, 240/256; maximum stored-component difference 0.7294117.

- **`quantize_color.sbs/kmeans_iteration` - Too hard for this pass:** Exact pixels by case/output: 4080/4096, 4080/4096, 4080/4096, 4080/4096; maximum stored-component difference 6.023682.

- **`quantize_color.sbs/get_min_max/GetMaxFromAlpha_v2` - Too hard for this pass:** Exact pixels by case/output: 0/1, 1/1, 0/1, 0/1; maximum stored-component difference 128.

- **`quantize_color.sbs/color_conversions/convert_RGB_to_Lab` - Too hard for this pass:** Exact pixels by case/output: 16163/16384, 64702/65536, 16163/16384, 16163/16384; maximum stored-component difference 1.

- **`quantize_color.sbs/color_conversions/convert_Lab_to_RGB` - Too hard for this pass:** Exact pixels by case/output: 16377/16384, 65504/65536, 16377/16384, 16383/16384; maximum stored-component difference 255.

- **`radial_blur.sbs/radial_blur_grayscale` - Too hard for this pass:** Exact pixels by case/output: 48/16384, 200/65536, 48/16384, 19/16384; maximum stored-component difference 2355.

- **`replace_color.sbs/replace_color` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 1/16384; maximum stored-component difference 64.

- **`rt_caustics.sbs/rt_caustics_base` - Too hard for this pass:** Exact pixels by case/output: 282/16384, 1219/65536, 282/16384, 185/16384; maximum stored-component difference 242.

- **`rt_irradiance.sbs/panoramic_lod` - Too hard for this pass:** Exact pixels by case/output: 4979/16384, 20210/65536, 4979/16384, 5976/16384; maximum stored-component difference 155.

- **`safe_transform.sbs/safe_transform` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 3649/16384; maximum stored-component difference 145.

- **`safe_transform.sbs/safe_transform_grayscale` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 6679/16384; maximum stored-component difference 132.

- **`scratches_1.sbs/scratches_1` - Too hard for this pass:** Exact pixels by case/output: 3025/16384, 12465/65536, 2995/16384, 2178/16384; maximum stored-component difference 64929.

- **`scratches_2.sbs/scratches_2` - Too hard for this pass:** Exact pixels by case/output: 431/16384, 567/65536, 461/16384, 291/16384; maximum stored-component difference 65534.

- **`scratches_4.sbs/scratches_4` - Too hard for this pass:** Exact pixels by case/output: 97/16384, 1946/65536, 96/16384; maximum stored-component difference 65535.

- **`shape.sbs/shape_base` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 16217/16384; maximum stored-component difference 1.

- **`shape_mapper.sbs/shape_mapper` - Too hard for this pass:** Exact pixels by case/output: 7688/16384, 28865/65536, 7688/16384, 5078/16384; maximum stored-component difference 30712.

- **`shape_splatter.sbs/shape_splatter_blend_color` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 16383/16384; maximum stored-component difference 1.

- **`shape_splatter.sbs/shape_splatter_blend` - Too hard for this pass:** Exact pixels by case/output: 16379/16384, 65532/65536, 16379/16384, 16319/16384; maximum stored-component difference 36.

- **`shape_splatter.sbs/shape_splatter_data_extract` - Too hard for this pass:** Exact pixels by case/output: 12131/16384, 4638/16384, 16384/16384, 16384/16384, 16384/16384, 16384/16384, 16384/16384, 48527/65536, 19321/65536, 65536/65536, 65536/65536, 65536/65536, 65536/65536, 65536/65536, 12131/16384, 4638/16384, 16384/16384, 16384/16384, 16384/16384, 16384/16384, 16384/16384, 12151/16384, 4862/16384, 16384/16384, 16384/16384, 16384/16384, 16384/16384, 16384/16384; maximum stored-component difference 1.192093e-07.

- **`shape_splatter_v2.sbs/mipmap_tools/Mipmap_N_from_O_color` - Too hard for this pass:** Exact pixels by case/output: 28672/32768, 114688/131072, 28672/32768, 15872/16384; maximum stored-component difference 255.

- **`shape_splatter_v2.sbs/shape_splatter_compute_normal` - Too hard for this pass:** Exact pixels by case/output: 128/16384, 256/65536, 128/16384, 128/16384; maximum stored-component difference 255.

- **`skew.sbs/skew` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 14625/16384; maximum stored-component difference 1.

- **`skew.sbs/skew_grayscale` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 15263/16384; maximum stored-component difference 1.

- **`slope_blur.sbs/slope_blur` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 160.

- **`slope_blur.sbs/slope_blur_grayscale` - Too hard for this pass:** Exact pixels by case/output: 296/16384, 2588/65536, 296/16384, 217/16384; maximum stored-component difference 124.

- **`slope_blur.sbs/slope_blur_grayscale_2` - Too hard for this pass:** Exact pixels by case/output: 296/16384, 2588/65536, 296/16384, 217/16384; maximum stored-component difference 124.

- **`smart_auto_tile.sbs/pattern_find_edges_step` - Too hard for this pass:** Exact pixels by case/output: 1048574/1048576, 1048575/1048576, 1048574/1048576, 1024/1024; maximum stored-component difference 255.

- **`splatter_circular.sbs/splatter_circular` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 15750/16384; maximum stored-component difference 57311.

- **`splatter_circular.sbs/splatter_circular_color` - Too hard for this pass:** Exact pixels by case/output: 14564/16384, 57648/65536, 14564/16384, 13974/16384; maximum stored-component difference 65535.

- **`spot_emboss.sbs/spot_emboss` - Too hard for this pass:** Exact pixels by case/output: 460/16384, 2680/65536, 460/16384, 163/16384; maximum stored-component difference 255.

- **`starburst.sbs/starburst` - Too hard for this pass:** Exact pixels by case/output: 8952/16384, 35696/65536, 8952/16384, 12412/16384; maximum stored-component difference 58561.

- **`stripes.sbs/stripes` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 45056/65536, 16384/16384, 16350/16384; maximum stored-component difference 65535.

- **`swirl.sbs/swirl` - Too hard for this pass:** Exact pixels by case/output: 16186/16384, 64854/65536, 16186/16384, 15722/16384; maximum stored-component difference 2.

- **`swirl.sbs/swirl_grayscale` - Too hard for this pass:** Exact pixels by case/output: 16274/16384, 65132/65536, 16274/16384, 16007/16384; maximum stored-component difference 2.

- **`symmetry_slice.sbs/symmetry_slice` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 11619/16384; maximum stored-component difference 7.

- **`tile_generator.sbs/tile_generator` - Too hard for this pass:** Exact pixels by case/output: 12821/16384, 51231/65536, 12821/16384, 14592/16384; maximum stored-component difference 18088.

- **`tile_generator_color.sbs/tile_generator_color` - Too hard for this pass:** Exact pixels by case/output: 10819/16384, 41641/65536, 10819/16384, 13363/16384; maximum stored-component difference 23978.

- **`tile_random.sbs/tile_random` - Too hard for this pass:** Exact pixels by case/output: 13932/16384, 53844/65536, 13189/16384, 11424/16384; maximum stored-component difference 57311.

- **`tile_random.sbs/tile_random_color` - Too hard for this pass:** Exact pixels by case/output: 13932/16384, 53844/65536, 13189/16384, 11424/16384; maximum stored-component difference 65535.

- **`tile_sampler.sbs/arrow` - Too hard for this pass:** Exact pixels by case/output: 259850/262144, 259850/262144, 259850/262144; maximum stored-component difference 155.

- **`tileable_material_to_mesh_material.sbs/tileable_material_to_mesh_material` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 16384/16384, 65536/65536, 65536/65536, 16384/16384, 16384/16384, 16384/16384, 4460/16384; maximum stored-component difference 4.

- **`trapezoid_transform.sbs/trapezoid_transform` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 16382/16384; maximum stored-component difference 1.

- **`trapezoid_transform.sbs/trapezoid_transform_grayscale` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 16382/16384; maximum stored-component difference 1.

- **`uv_mapper.sbs/uv_mapper_grayscale_sub` - Too hard for this pass:** Exact pixels by case/output: 15835/16384, 63262/65536, 15835/16384, 16384/16384; maximum stored-component difference 1.

- **`uv_mapper.sbs/uv_mapper_color_sub` - Too hard for this pass:** Exact pixels by case/output: 16223/16384, 65081/65536, 16223/16384, 16384/16384; maximum stored-component difference 1.

- **`vector_morph.sbs/vector_morph` - Too hard for this pass:** Exact pixels by case/output: 2840/16384, 15375/65536, 2840/16384, 3262/16384; maximum stored-component difference 250.

- **`vector_morph.sbs/vector_morph_grayscale` - Too hard for this pass:** Exact pixels by case/output: 1277/16384, 4578/65536, 1277/16384, 1540/16384; maximum stored-component difference 146.

- **`vertical_noise.sbs/vertical_noise` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 1/65536, 0/16384, 0/16384; maximum stored-component difference 51100.

- **`waveform_01_v2.sbs/waveform_01_v2` - Too hard for this pass:** Exact pixels by case/output: 12757/16384, 57538/65536, 12257/16384, 15190/16384; maximum stored-component difference 255.

- **`waveform_1.sbs/waveform_1` - Too hard for this pass:** Exact pixels by case/output: 10166/16384, 44500/65536, 10112/16384, 16046/16384; maximum stored-component difference 65535.

- **`weave_1.sbs/weave_1` - Too hard for this pass:** Exact pixels by case/output: 928/16384, 3872/65536, 928/16384, 848/16384; maximum stored-component difference 65535.

- **`weave_2.sbs/weave_2` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 2/16384; maximum stored-component difference 64614.

- **`weave_3.sbs/weave_3` - Too hard for this pass:** Exact pixels by case/output: 15760/16384, 63040/65536, 15760/16384, 14816/16384; maximum stored-component difference 64255.

- **`weave_4.sbs/weave_4` - Too hard for this pass:** Exact pixels by case/output: 10240/16384, 40960/65536, 10240/16384, 9728/16384; maximum stored-component difference 64511.

- **`white_noise.sbs/white_noise_fast` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 2/65536, 1/16384; maximum stored-component difference 65348.

- **`white_noise_v2.sbs/white_noise_v2` - Too hard for this pass:** Exact pixels by case/output: 68/16384, 298/65536, 70/16384, 153/16384; maximum stored-component difference 255.

- **`wood_fibers_1.sbs/wood_fibers_1` - Too hard for this pass:** Exact pixels by case/output: 3/16384, 10/65536, 0/16384, 4/16384; maximum stored-component difference 65495.

- **`wood_fibers_2.sbs/wood_fibers_2` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget.

- **`paths_tools.sbs/paths_polygon` - Too hard for this pass:** Exact pixels by case/output: 252/256, 252/256, 252/256, 250/256; maximum stored-component difference 5.602837e-06.

- **`paths_tools.sbs/paths_trace` - Too hard for this pass:** Exact pixels by case/output: 184/256, 182/256, 184/256, 138/256; maximum stored-component difference 145.

- **`paths_tools.sbs/vertex_processor` - Too hard for this pass:** Exact pixels by case/output: 250/256, 248/256, 250/256, 248/256; maximum stored-component difference 1.

- **`paths_tools.sbs/vertex_processor_simple` - Too hard for this pass:** Exact pixels by case/output: 250/256, 248/256, 250/256, 248/256; maximum stored-component difference 1.

- **`paths_tools.sbs/Impl/compute_paths_lengths` - Too hard for this pass:** Cooks report: function returned bool, expected native type 256.

- **`paths_tools.sbs/Impl/sort_by_blue` - Too hard for this pass:** Cooks report: function returned bool, expected native type 256. Exact pixels by case/output: 256/256, 256/256, 256/256; maximum stored-component difference 0.

- **`paths_tools.sbs/Impl/mask_to_facets` - Too hard for this pass:** Exact pixels by case/output: 256/256, 240/256, 256/256, 247/256; maximum stored-component difference 2.980232e-08.

- **`spline_tools.sbs/internal_tools/2dtransform_spline_sub` - Too hard for this pass:** Its connectable spline-count input defaults to zero. Nonempty image buffers are available, but a nonzero value-pin case is not verified through the reference API in this pass.

- **`spline_tools.sbs/internal_tools/compute_bridge_distance` - Too hard for this pass:** Its connectable spline-count input defaults to zero. A meaningful nonempty case needs a configured value input; that case is not verified in this pass.

- **`spline_tools.sbs/internal_tools/points_preview` - Too hard for this pass:** Its connectable point-count input defaults to zero. Empty-preview agreement cannot establish parity for drawing a nonempty point list.

- **`spline_tools.sbs/internal_tools/scatter_splines_on_splines_preprocess` - Too hard for this pass:** The reference compute did not finish after more than six minutes with the default zero input-spline count and original curve data. The comparison session was recovered; nonzero value-input behavior is not verified.

- **`spline_tools.sbs/internal_tools/compute_spline_tangents` - Too hard for this pass:** Exact pixels by case/output: 247/256, 249/256, 247/256, 248/256; maximum stored-component difference 0.1875924.

- **`spline_tools.sbs/internal_tools/normalize_spline_distance` - Too hard for this pass:** Exact pixels by case/output: 193/256, 193/256, 193/256, 193/256; maximum stored-component difference 1.907349e-06.

- **`spline_tools.sbs/internal_tools/compute_spline_length_init` - Too hard for this pass:** Exact pixels by case/output: 251/256, 250/256, 251/256, 250/256; maximum stored-component difference 1.490116e-08.

- **`spline_tools.sbs/internal_tools/resample_spline_data` - Too hard for this pass:** Exact pixels by case/output: 220/256, 208/256, 220/256, 208/256, 220/256, 208/256, 220/256, 208/256; maximum stored-component difference 0.5.

- **`spline_tools.sbs/internal_tools/spline_warp_sub` - Too hard for this pass:** Exact pixels by case/output: 253/256, 253/256, 253/256, 240/256; maximum stored-component difference 0.1294118.

- **`spline_tools.sbs/internal_tools/select_spline_sub` - Too hard for this pass:** Exact pixels by case/output: 30720/32768, 30720/32768, 30720/32768, 30720/32768; maximum stored-component difference 0.5240468.

- **`spline_tools.sbs/internal_tools/modify_spline_property_sub` - Too hard for this pass:** Exact pixels by case/output: 15/256, 15/256, 15/256, 240/256; maximum stored-component difference 0.2784314.

- **`paths_tools.sbs/Impl/facets_to_paths_impl_link_template` - Too hard for this pass:** Exact pixels by case/output: 0/256, 0/256, 0/256, 256/256; maximum stored-component difference 1.

- **`paths_tools.sbs/Impl/facets_to_paths_impl_stitch_template` - Too hard for this pass:** Exact pixels by case/output: 0/256, 0/256, 0/256, 0/256; maximum stored-component difference 1.

- **`pavement_1.sbs/pavement_1` - Too hard for this pass:** Exact pixels by case/output: 2160/16384, 9712/65536, 2160/16384, 800/16384; maximum stored-component difference 65535.

- **`splatter.sbs/splatter` - Too hard for this pass:** Exact pixels by case/output: 2048/16384, 4096/65536, 2048/16384, 966/16384; maximum stored-component difference 44975.

- **`splatter_color.sbs/splatter_color` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 924/16384; maximum stored-component difference 47536.

- **`noise_dirt_4.sbs/dirt_4` - Too hard for this pass:** The MaterialCraft measurement exceeded the 180-second limit. Reference captures exist; exact pixel parity remains unverified.

- **`noise_dirt_4_v2.sbs/dirt_4` - Too hard for this pass:** The MaterialCraft measurement exceeded the 180-second limit. Reference captures exist; exact pixel parity remains unverified.

- **`noise_dirt_gradient_v2.sbs/dirt_gradient` - Too hard for this pass:** The MaterialCraft measurement exceeded the 180-second limit. Reference captures exist; exact pixel parity remains unverified.

- **`pbr_render.sbs/raytrace_position` - Too hard for this pass:** The MaterialCraft measurement exceeded the 180-second limit. Reference captures exist; exact pixel parity remains unverified.

- **`pbr_render.sbs/raytraced_specular_pass_8` - Too hard for this pass:** The MaterialCraft measurement exceeded the 180-second limit. Reference captures exist; exact pixel parity remains unverified.

- **`pbr_render.sbs/raytraced_irradiance_pass_8` - Too hard for this pass:** The MaterialCraft measurement exceeded the 180-second limit. Reference captures exist; exact pixel parity remains unverified.

- **`pbr_render2.sbs/raytrace_position` - Too hard for this pass:** The MaterialCraft measurement exceeded the 180-second limit. Reference captures exist; exact pixel parity remains unverified.

- **`pbr_render2.sbs/raytraced_specular_pass_8` - Too hard for this pass:** The MaterialCraft measurement exceeded the 180-second limit. Reference captures exist; exact pixel parity remains unverified.

- **`pbr_render2.sbs/raytraced_irradiance_is_pass_8` - Too hard for this pass:** The MaterialCraft measurement exceeded the 180-second limit. Reference captures exist; exact pixel parity remains unverified.

- **`rt_irradiance.sbs/mip_max` - Too hard for this pass:** The reference output is 128x128 while MaterialCraft produces 64x64 for the same 128x128 case. Pixel parity cannot be claimed with different output dimensions.

- **`rt_shadows.sbs/mip_max` - Too hard for this pass:** The reference output is 128x128 while MaterialCraft produces 64x64 for the same 128x128 case. Pixel parity cannot be claimed with different output dimensions.

- **`pbr_render_ortho.sbs/sphere_displace` - Too hard for this pass:** The MaterialCraft measurement exceeded the 180-second limit. Reference captures exist; exact pixel parity remains unverified.

- **`rt_irradiance.sbs/compute_ray_direction` - Too hard for this pass:** Exact pixels by case/output: 16368/16384, 65414/65536, 16368/16384; maximum stored-component difference 0.0004882812. Connectable value pins retain their defaults; nondefault values require a wired reference fixture and remain unverified.

- **`rt_irradiance.sbs/compute_irradiance` - Too hard for this pass:** Exact pixels by case/output: 7010/16384, 27432/65536, 7010/16384; maximum stored-component difference 0.001953125. Connectable value pins retain their defaults; nondefault values require a wired reference fixture and remain unverified.

- **`rt_irradiance.sbs/debug_hit_uv` - Too hard for this pass:** Exact pixels by case/output: 67/16384, 260/65536, 67/16384; maximum stored-component difference 159. Connectable value pins retain their defaults; nondefault values require a wired reference fixture and remain unverified.

## Verified cases

- **`3d_perlin_noise.sbs/3D_perlin_noise_fractal` - Parity:** All 114,688 eight-bit output pixels match across four cases: defaults at 128x128 and 256x256, seed 42 at 128x128, and roughness 0.7 at 128x128. Generated linear XY position ramps use Z=128/255 and alpha=1. Parent precision is eight bits. Other parameters retain defaults. This validates these cases, not every parameter combination.

- **`white_noise.sbs/white_noise` - Parity:** All 98,304 sixteen-bit pixels match across defaults at 128x128/256x256 and seed 42. This preserves the original result after the shared importer, sampler, and function changes.

- **`3d_simplex_noise.sbs/3d_simplex_noise` - Parity:** All 114,688 eight-bit pixels match for defaults at 128x128 and 256x256, seed 42 at 128x128, and scale 4 at 128x128. Generated position maps match the other 3D-noise comparisons.

- **`3d_texture_jump_flood.sbs/3d_texture_jump_flood_iteration` - Parity:** All 131,072 eight-bit RGBA pixels match across ramp inputs at 128x128 and 256x256, iteration 1 at 128x128, and two randomized 128x128 candidate-point cases (iterations 0 and 1, with disabled seeds). Floored modulo fixes the previously incorrect negative-coordinate wrapping.

- **`3d_texture_position.sbs/3d_texture_position` - Parity:** Both native 4096x4096 RGBA16 atlases match exactly: default grid 16x16 and grid 8x4. All 33,554,432 pixels (134,217,728 components) were compared at the graph's actual fixed output size, with no resizing.

- **`3d_texture_transform.sbs/3d_texture_offset_color` - Parity:** All 131,072 RGBA8 pixels match across five cases: default 128x128, fractional signed offsets at 128x128, voxel offsets at 256x256, changed grid 8x4 with offsets at 128x128, and varying input alpha with fractional offsets. All four channels were compared.

- **`3d_texture_transform.sbs/3d_texture_offset_grayscale` - Parity:** All 114,688 eight-bit pixels match across four corresponding default, fractional-offset, voxel-offset, and changed-grid cases at 128x128 and 256x256.

- **`angle_to_vector.sbs/angle_to_vector` - Parity:** All 114,688 RGBA8 pixels match across defaults at 128x128/256x256, signed-vector format, and angle offset 0.25. Inputs are original scalar angle maps; all four output channels were compared.

- **`atlas_spliter.sbs/max_ba` - Parity:** All 98,304 eight-bit output pixels match across 128x128/256x256 color inputs and a varying-alpha 128x128 case. The varying-alpha output is nonconstant.

- **`auto_crop.sbs/min_max` - Parity:** All 24,576 RGBA8 output pixels match for original rectangular masks at parent sizes 128 and 256 and a varying-alpha color case. Actual outputs are 64x64, 128x128, and 64x64 respectively.

- **`auto_levels.sbs/auto_levels` - Parity:** All 86,016 eight-bit pixels match for restricted-range inputs at 128x128/256x256 and a constant 64x64 image. The nonconstant inputs span levels 32 through 191; the flat input is level 96.

- **`auto_levels.sbs/min_max` - Parity:** All 102,400 eight-bit pixels match across minimum/maximum selection at 128x128, minimum selection at 256x256, and a flat 64x64 input.

- **Exact blend family - Parity:** `blend.sbs/add`, `add_sub`, `copy`, `difference`, `exclusion`, `hard_light`, `linear_burn`, `max`, `min`, `multiply`, `overlay`, `screen`, `soft_light`, and `subtract` each match all 114,688 RGBA8 pixels across four cases: opaque 128x128, varying foreground/background alpha at 128x128, opacity 0.37 with alpha-channel blending disabled, and 256x256 varying-alpha inputs with opacity 0.67 and seed 42. Original masks and all four output channels were compared. Add and Copy additionally match their 16x16 RGBA32F HDR cases (512 pixels total).

- **`blend_switch.sbs/multi_switch` - Parity:** All 131,072 RGBA8 pixels match across default selection, twentieth input, eleventh of twelve inputs, out-of-range selection with three active inputs, and second input at 256x256. Original twenty-input maps include varying alpha.

- **`blend_switch.sbs/switch` - Parity:** Both boolean selections at 128x128 and 256x256 match all 163,840 RGBA8 pixels, including varying alpha.

- **Grayscale switches - Parity:** `blend_switch.sbs/multi_switch_grayscale` matches all 131,072 eight-bit pixels in the same five selection cases as its color counterpart. `switch_grayscale` matches both boolean selections at 128x128/256x256 (163,840 eight-bit pixels) and both selections at 128x128 in float storage (32,768 pixels). Original color resources exercise the corrected bridge conversion.

- **`brick_generator.sbs/resources/Tile` - Parity:** Defaults at 128x128/256x256 and seed 42 are exact but constant. A seven-by-three grid with scale 0.8 and offset 0.5 produces a nonconstant result and also matches exactly. All 114,688 sixteen-bit pixels across four cases agree.

- **`checker_2.sbs/checker_2` - Parity:** All 114,688 eight-bit pixels match for defaults at 128x128/256x256, seed 42, and changed tiling/luminance variation/disorder at 128x128.

- **`clamp.sbs/clamp_grayscale` - Parity:** All 114,688 eight-bit pixels match across defaults at 128x128/256x256, seed 42, and changed clamp limits.

- **`clone_patch.sbs/transform_to_center` - Parity:** All 114,688 RGBA8 pixels match across defaults at 128x128/256x256, seed 42, and changed transform settings. Default all-one matrix settings produce black; the changed nonsingular matrix/offset produces a nonconstant output and is also exact.

- **`clone_patch.sbs/transform_to_center_grayscale` - Parity:** All 114,688 eight-bit pixels match across defaults at 128x128/256x256, seed 42, and changed transform settings. Default all-one matrix settings produce black; the changed nonsingular matrix/offset produces a nonconstant output and is also exact.

- **`clone_patch.sbs/transform_from_center` - Parity:** All 114,688 RGBA8 pixels match across defaults at 128x128/256x256, seed 42, and changed transform settings. Default all-one matrix settings produce black; the changed nonsingular matrix/offset produces a nonconstant output and is also exact.

- **`color_space_conversion.sbs/linear_srgb_to_acescg` - Parity:** All 114,688 RGBA8 pixels match across defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`color_space_conversion.sbs/acescg_to_srgb` - Parity:** All 114,688 RGBA8 pixels match across defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`color_space_conversion.sbs/linear_srgb_to_srgb` - Parity:** All 114,688 RGBA8 pixels match across defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`color_space_conversion.sbs/srgb_to_linear_srgb` - Parity:** All 114,688 RGBA8 pixels match across defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`color_space_conversion.sbs/linear_srgb_to_srgb_grayscale` - Parity:** All 114,688 eight-bit pixels match across defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`color_space_conversion.sbs/srgb_to_linear_srgb_grayscale` - Parity:** All 114,688 eight-bit pixels match across defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **Constant typed values - Parity:** `constant_values.sbs` graphs `constant_float`, `constant_float2`, `constant_float3`, `constant_float4`, `constant_integer`, `constant_integer2`, `constant_integer3`, `constant_boolean` match all three recorded values each (24 typed output comparisons). Cases include zero, positive/negative scalar and vector values, HDR floats, and both Boolean states. These graphs publish values, so image pixel comparisons do not apply. The four-component integer boundary exception is recorded above.

- **`clamp.sbs/clamp` - Parity:** All 114,688 RGBA8 pixels match for defaults at 128x128/256x256, seed 42, and limits 0.2/0.7 with alpha clamping disabled. Original varying-alpha input maps exercise both the corrected legacy ignore-alpha stages and the final alpha-channel selection.

- **`composite.sbs/composite/filters/filter_transformation_grayscale` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed settings.

- **`composite.sbs/composite/filters/filter_mask` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`composite.sbs/composite/filters/filter_split_rgb_a` - Parity:** All 229,376 output pixels match across defaults at 128x128/256x256, seed 42, and an alternate original input map. All output bridges were compared.

- **`composite.sbs/composite/blends/blend_content_grayscale` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed settings.

- **`composite.sbs/composite/blends/blend_matte_fg_passthrough` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`composite.sbs/composite/blends/blend_matte_bg_passthrough` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`composite.sbs/composite/blends/blend_content_pre_post_composite_grayscale` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed settings.

- **`composite.sbs/composite/background/uniform_color_float3` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed settings.

- **`contrast_luminosity.sbs/contrast_luminosity` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed contrast/luminosity.

- **`contrast_luminosity.sbs/contrast_luminosity_grayscale` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed contrast/luminosity.

- **`exposure.sbs/exposure` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and exposure 1.5.

- **`extend_shape.sbs/extend_shape_grayscale` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed extension mode/endpoints with nearest sampling. Original varying-alpha color or nonconstant grayscale input maps were used.

- **`extend_shape.sbs/extend_shape_color` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed extension mode/endpoints with nearest sampling. Original varying-alpha color or nonconstant grayscale input maps were used.

- **`facing_normal.sbs/facing_normal` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and the additional input case. Inputs are original curved normal maps.

- **`flood_fill.sbs/flood_fill_step` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and the additional input case. Inputs are original bounded RGBA coordinate/box fields; color and alpha were compared.

- **`flood_fill.sbs/flood_fill_to_bbox_size` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed exposed parameters. Inputs are original bounded RGBA coordinate/box fields; color and alpha were compared.

- **`flood_fill.sbs/flood_fill_to_position` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and the additional input case. Inputs are original bounded RGBA coordinate/box fields; color and alpha were compared.

- **`flood_fill_2.sbs/flood_fill_to_bbox_size` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed exposed parameters. Inputs are original bounded RGBA coordinate/box fields; color and alpha were compared.

- **`flood_fill_2.sbs/flood_fill_to_position` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and the additional input case. Inputs are original bounded RGBA coordinate/box fields; color and alpha were compared.

- **`flood_fill_2.sbs/flood_fill_impl/flood_fill_step_A` - Parity:** All 229,376 output pixels match across defaults at 128x128/256x256, seed 42, and the additional input case. Inputs are original bounded RGBA coordinate/box fields; color and alpha were compared. Every output bridge was compared.

- **`flood_fill_2.sbs/flood_fill_impl/flood_fill_step_A_switch` - Parity:** All 229,376 output pixels match across defaults at 128x128/256x256, seed 42, and changed exposed parameters. Inputs are original bounded RGBA coordinate/box fields; color and alpha were compared. Every output bridge was compared.

- **`flood_fill_2.sbs/flood_fill_impl/flood_fill_step_B` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and the additional input case. Inputs are original bounded RGBA coordinate/box fields; color and alpha were compared.

- **`floodfill_to_index.sbs/pass` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`fxaa.sbs/fxaa_grayscale` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and the additional input case.

- **`glow.sbs/clamp` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`curvature_smooth_v2.sbs/compute_curvature_v2` - Parity:** All 16,668 RGBA32F pixels match in six cases after correcting internal size resolution. Actual outputs are three 2x2 images, one 4x4 image, one 16x16 image, and one 128x128 image. Larger cases change the level and both intensity components and use original nonconstant curved normal and previous-result maps.

- **`grid_atlas.sbs/grid_atlas_grayscale` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`grid_atlas.sbs/grid_atlas_color` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`hdr_exposure_preview.sbs/hdr_exposure_preview` - Parity:** All 114,688 output pixels (RGBA32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters. The parent format is f32.

- **`height_extrude.sbs/impl/acceleration_struct_impl` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`histogram_range.sbs/histogram_range` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`histogram_scan_non_uniform.sbs/histogram_scan_non_uniform` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`histogram_select.sbs/histogram_select` - Parity:** All 114,688 output pixels (LUM16) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`histogram_tools.sbs/horizontal_max` - Parity:** All 4 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`id_to_mask.sbs/id_to_mask_color` - Parity:** All 114,688 output pixels (LUM16) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`id_to_mask.sbs/id_to_mask_grayscale` - Parity:** All 114,688 output pixels (LUM16) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`invert.sbs/invert` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`invert.sbs/invert_grayscale` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`linear_srgb_conversion.sbs/convert_to_linear` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`linear_srgb_conversion.sbs/convert_to_linear_grayscale` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`linear_srgb_conversion.sbs/convert_to_srgb` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`linear_srgb_conversion.sbs/convert_to_srgb_grayscale` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`material_switch.sbs/material_switch` - Parity:** All 1,376,256 output pixels (LUM8, RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters. All output bridges were compared.

- **`median_filter.sbs/median_filter_grayscale` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`mesh_adaptive_base_color.sbs/base_color_8_inputs` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`mesh_adaptive_base_color.sbs/base_color_4_inputs` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`mesh_adaptive_base_color.sbs/base_color_2_inputs` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`mesh_data_combiner.sbs/mesh_data_combiner` - Parity:** All 1,261,568 output pixels (LUM16, LUM8, RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters. All output bridges were compared.

- **`multiangle_to_normal.sbs/angle_to_normal` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`multiangle_to_normal.sbs/angle_weight` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`normal_color.sbs/normal_color` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`normal_intensity.sbs/normal_intensity` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`normal_invert.sbs/normal_invert` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`normal_mapper.sbs/normal_mapper` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`normal_vector_rotation.sbs/normal_vector_rotation` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`normal_uncombine.sbs/get_intensity_iteration` - Parity:** All 4 typed outputs match exactly across 4 recorded cases, including an alternate original input or changed exposed parameters.

- **`noise_fractal_sum_base_v2.sbs/noise_disorder` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`pattern_brick_generator.sbs/resources/Tile` - Parity:** All 114,688 output pixels (LUM16) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`pattern_checker_1.sbs/checker_1` - Parity:** All 114,688 output pixels (LUM16) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`pattern_tile_random_2.sbs/functions/debug_find_cell` - Parity:** All 114,688 output pixels (RGBA32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`pbr_albedo_safe_color.sbs/pbr_albedo_safe_color` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`pbr_dielectric_F0.sbs/pbr_dielectric_F0` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`pbr_metal_reflectance.sbs/pbr_metal_reflectance` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`pbr_render2.sbs/sat/sat_iteration` - Parity:** All 114,688 output pixels (LUM32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`pbr_render_ortho.sbs/get_max_coord` - Parity:** All 4 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`rt_ao.sbs/pp_template/pp_template_64g` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`rt_bent_normal.sbs/pp_template/pp_template_64c` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`rt_irradiance.sbs/template/pp_template` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`rt_shadows.sbs/template/pp_template` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`pow.sbs/pow_grayscale` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`pow.sbs/pow` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`quantize.sbs/quantize_grayscale` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`quantize.sbs/quantize` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`quantize_2.sbs/quantize_grayscale` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`quantize_color.sbs/apply_color_palette_sub` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`quantize_color.sbs/get_min_max/GetMaxFromAlpha_v3` - Parity:** All 4 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`quantize_color.sbs/helpers/cdf_pass` - Parity:** All 114,688 output pixels (LUM32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`quantize_color.sbs/bitonic_sort/graph/remap_sorting` - Parity:** All 114,688 output pixels (RGBA32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`quantize_color.sbs/bitonic_sort/graph/remap_sorting_256x1` - Parity:** All 114,688 output pixels (RGBA32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`quantize_color.sbs/bitonic_sort/graph/bitonic_sort_iteration` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`quantize_color.sbs/bitonic_sort/graph/initial_iteration` - Parity:** All 57,344 output pixels (RGBA32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`rgb-a_merge.sbs/rgb-a_merge` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`rgb-a_split.sbs/rgb-a_split` - Parity:** All 229,376 output pixels (LUM8, RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map. All output bridges were compared.

- **`rgba_merge.sbs/rgba_merge` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`rgba_split.sbs/rgba_split` - Parity:** All 458,752 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map. All output bridges were compared.

- **`quantize_color.sbs/create_color_palette_16colors` - Parity:** All 1,024 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`quantize_color.sbs/view_color_palette_sub` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`rt_ao_v2.sbs/mip_max_step` - Parity:** All 25,600 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`rt_bent_normal_v2.sbs/mip_max_step` - Parity:** All 25,600 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`rt_shadows_v2.sbs/mip_max_step` - Parity:** All 25,600 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`shape_splatter.sbs/shape_splatter_to_mask` - Parity:** All 114,688 output pixels (LUM32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`shape_splatter.sbs/min_max` - Parity:** All 2,048 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map. All output bridges were compared.

- **`shape_splatter_v2.sbs/bitonic_sort/graph/remap_sorting` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`shape_splatter_v2.sbs/bitonic_sort/graph/bitonic_sort_iteration` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`shape_splatter_v2.sbs/bitonic_sort/graph/initial_iteration` - Parity:** All 57,344 output pixels (RGBA32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`shape_splatter_v2.sbs/poisson_disc/poisson_disc_iteration` - Parity:** All 800 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`shape_splatter_v2.sbs/scanline_to_z_order` - Parity:** All 114,688 output pixels (RGBA16) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`shape_splatter_v2.sbs/grid_to_scanline` - Parity:** All 114,688 output pixels (RGBA32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`shape_splatter_v2_companion.sbs/shape_splatter_to_mask` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`summed_area_table.sbs/summed_area_table_iteration` - Parity:** All 114,688 output pixels (LUM32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`summed_area_table_2.sbs/summed_area_table_iteration` - Parity:** All 114,688 output pixels (LUM32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`symmetry.sbs/symmetry` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`threshold.sbs/threshold` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`value_get_size.sbs/get_size_color` - Parity:** All 4 typed outputs match exactly across 4 recorded cases, including an alternate original input or changed exposed parameters.

- **`value_get_size.sbs/get_size_grayscale` - Parity:** All 4 typed outputs match exactly across 4 recorded cases, including an alternate original input or changed exposed parameters.

- **`paths_tools.sbs/Impl/get_number_of_valid_paths` - Parity:** All 4 typed outputs match exactly across 4 recorded cases, including an alternate original input or changed exposed parameters.

- **`paths_tools.sbs/Utils/RelativeToInputPP` - Parity:** All 1,024 output pixels (RGBA32F) match across 4 recorded cases: original 16x16 curve/path buffers, seed 42, and changed exposed parameters. The parent format is f32.

- **`spline_tools.sbs/internal_tools/get_spline_point_data` - Parity:** All 4 typed outputs match exactly across 4 recorded cases, including an alternate original input or changed exposed parameters.

- **`spline_tools.sbs/internal_tools/compute_spline_distance_iteration` - Parity:** All 1,024 output pixels (LUM32F) match across 4 recorded cases: original 16x16 curve/path buffers, seed 42, and changed exposed parameters. The parent format is f32.

- **`spline_tools.sbs/internal_tools/append_splines_sub` - Parity:** All 262,144 output pixels (RGBA32F) match across 4 recorded cases: original 16x16 curve/path buffers, seed 42, and changed exposed parameters. The parent format is f32. All output bridges were compared. All four typed count outputs also match; connectable counts retain their default values.

- **`spline_tools.sbs/-----WIP-----/circular_array_wip` - Parity:** All 1,024 output pixels (RGBA32F) match across 4 recorded cases: original 16x16 curve/path buffers, seed 42, and changed exposed parameters. The parent format is f32. All four typed count outputs also match; connectable counts retain their default values.

- **`spline_tools.sbs/-----WIP-----/displace_spline_by_spline_wip` - Parity:** All 1,024 output pixels (RGBA32F) match across 4 recorded cases: original 16x16 curve/path buffers, seed 42, and changed exposed parameters. The parent format is f32. All four typed count outputs also match; connectable counts retain their default values.

- **`templates.sbs/Templates/Standard_Materials/Simple_Material` - Parity:** All 491,520 output pixels (RGBA8) match across 3 recorded cases: defaults at 128x128/256x256 and seed 42. All output bridges were compared.

- **`templates.sbs/Templates/Standard_Materials/Extended_Material` - Parity:** All 688,128 output pixels (RGBA8) match across 3 recorded cases: defaults at 128x128/256x256 and seed 42. All output bridges were compared.

- **`pbr_render2.sbs/height_adjust` - Parity:** All 98,304 output pixels (LUM8) match across 3 recorded cases: defaults at 128x128/256x256 and seed 42. Connectable value pins retain their defaults; nondefault values require a wired reference fixture and remain unverified.

## First-pass validation

- Final release CLI and native desktop builds pass, including the unassigned-variable, grayscale Pixel Processor, integer-conversion, and output-bridge changes. Formatting, dependency layers, and all ten WebAssembly crate checks pass. An independent inventory rescan agrees with all 546 roots, and all 320 package links resolve.
- Workspace Clippy is blocked by an existing `chunks_exact_to_as_chunks` lint in the text primitive. Clippy for the four changed library crates passes with that existing lint suppressed.
- Native output-format save/reload checks preserve all four formats: u8, u16, f16, and f32. No repository tests were added or test suites run. Pixel and save/reload measurements use disposable files outside the repository. Reference captures use version 16.0.7 with the Direct3D 11 backend; exact-match claims apply to the recorded cases.
