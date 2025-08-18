# Text Namespace

Controls typography: font, size, weight, color, alignment, and text effects.

## Properties

### Font Family

Fonts are configured in `reedstyle.fonts.yaml` using the pattern font-a through font-f:

```yaml
# reedstyle.fonts.yaml
fonts:
  font-a:  # Primary font (matches brand-a pattern)
    family: "'Inter', sans-serif"
  font-b:  # Secondary font
    family: "'Playfair Display', serif"
  font-c:  # Code font
    family: "'JetBrains Mono', monospace"
```

Usage:

```html
<!-- Named fonts (from config) -->
<reed as="div" text="font:font-a">    <!-- Primary font -->
<reed as="div" text="font:font-b">    <!-- Secondary font -->
<reed as="div" text="font:font-c">    <!-- Code font -->

<!-- Semantic mappings -->
<reed as="div" text="font:system">    <!-- Maps to font-a -->
<reed as="div" text="font:heading">   <!-- Maps to font-b -->
<reed as="div" text="font:body">      <!-- Maps to font-a -->
<reed as="div" text="font:code">      <!-- Maps to font-c -->
```

### Font Size

```html
<!-- Dimension Scale -->
<reed as="div" text="size:tiny">      <!-- 0.75x root -->
<reed as="div" text="size:small">     <!-- 0.875x root -->
<reed as="div" text="size:normal">    <!-- 1x root -->
<reed as="div" text="size:large">     <!-- 1.25x root -->
<reed as="div" text="size:huge">      <!-- 1.5x root -->
<reed as="div" text="size:mega">      <!-- 2x root -->
<reed as="div" text="size:ultra">     <!-- 3x root -->
```

### Font Weight

```html
<reed as="div" text="weight:thin">     <!-- 100 -->
<reed as="div" text="weight:light">    <!-- 300 -->
<reed as="div" text="weight:normal">   <!-- 400 -->
<reed as="div" text="weight:medium">   <!-- 500 -->
<reed as="div" text="weight:semibold"> <!-- 600 -->
<reed as="div" text="weight:bold">     <!-- 700 -->
<reed as="div" text="weight:extrabold"> <!-- 800 -->
<reed as="div" text="weight:black">    <!-- 900 -->
```

### Text Color

```html
<!-- Brand colors -->
<reed as="div" text="color:brand-a">
<reed as="div" text="color:brand-b">

<!-- Base colors -->
<reed as="div" text="color:base-900">  <!-- Dark text -->
<reed as="div" text="color:base-700">  <!-- Medium dark -->
<reed as="div" text="color:base-500">  <!-- Medium -->
<reed as="div" text="color:base-300">  <!-- Light -->
<reed as="div" text="color:base-100">  <!-- Very light -->

<!-- Semantic colors -->
<reed as="div" text="color:state-success">
<reed as="div" text="color:state-warning">
<reed as="div" text="color:state-error">
<reed as="div" text="color:state-info">

<!-- Special -->
<reed as="div" text="color:current">   <!-- currentColor -->
<reed as="div" text="color:inherit">   <!-- inherit -->
```

### Text Alignment

```html
<reed as="div" text="align:left">
<reed as="div" text="align:center">
<reed as="div" text="align:right">
<reed as="div" text="align:justify">
<reed as="div" text="align:start">     <!-- Locale-aware -->
<reed as="div" text="align:end">       <!-- Locale-aware -->
```

### Line Height

```html
<reed as="div" text="leading:none">    <!-- 1 -->
<reed as="div" text="leading:tight">   <!-- 1.25 -->
<reed as="div" text="leading:snug">    <!-- 1.375 -->
<reed as="div" text="leading:normal">  <!-- 1.5 -->
<reed as="div" text="leading:relaxed"> <!-- 1.625 -->
<reed as="div" text="leading:loose">   <!-- 2 -->
```

### Letter Spacing

```html
<reed as="div" text="tracking:tighter"> <!-- -0.05em -->
<reed as="div" text="tracking:tight">   <!-- -0.025em -->
<reed as="div" text="tracking:normal">  <!-- 0 -->
<reed as="div" text="tracking:wide">    <!-- 0.025em -->
<reed as="div" text="tracking:wider">   <!-- 0.05em -->
<reed as="div" text="tracking:widest">  <!-- 0.1em -->
```

### Text Decoration

```html
<!-- Decoration line -->
<reed as="div" text="decoration:none">
<reed as="div" text="decoration:underline">
<reed as="div" text="decoration:overline">
<reed as="div" text="decoration:line-through">

<!-- Decoration style -->
<reed as="div" text="decoration:solid">
<reed as="div" text="decoration:double">
<reed as="div" text="decoration:dotted">
<reed as="div" text="decoration:dashed">
<reed as="div" text="decoration:wavy">

<!-- Decoration color -->
<reed as="div" text="decoration:brand-a">  <!-- Color -->

<!-- Combined -->
<reed as="div" text="[decoration:underline, decoration:brand-a, decoration:wavy]">
```

### Text Transform

```html
<reed as="div" text="transform:none">
<reed as="div" text="transform:uppercase">
<reed as="div" text="transform:lowercase">
<reed as="div" text="transform:capitalize">
```

### Text Style

```html
<reed as="div" text="style:normal">
<reed as="div" text="style:italic">
<reed as="div" text="style:oblique">
```

### Text Overflow

```html
<reed as="div" text="overflow:clip">
<reed as="div" text="overflow:ellipsis">
<reed as="div" text="truncate">        <!-- Shorthand for ellipsis -->
```

### Whitespace

```html
<reed as="div" text="whitespace:normal">
<reed as="div" text="whitespace:nowrap">
<reed as="div" text="whitespace:pre">
<reed as="div" text="whitespace:pre-line">
<reed as="div" text="whitespace:pre-wrap">
```

### Word Break

```html
<reed as="div" text="break:normal">
<reed as="div" text="break:words">     <!-- break-word -->
<reed as="div" text="break:all">       <!-- break-all -->
<reed as="div" text="break:keep">      <!-- keep-all -->
```

### List Style

```html
<reed as="ul" text="list:none">
<reed as="ul" text="list:disc">
<reed as="ul" text="list:circle">
<reed as="ul" text="list:square">
<reed as="ol" text="list:decimal">
<reed as="ol" text="list:roman">
<reed as="ol" text="list:alpha">
```

## Common Patterns

### Heading Styles

```html
<!-- Large heading -->
<reed as="h1" 
      text="[size:5xl, weight:bold, leading:tight, tracking:tight]">
  Main Heading
</reed>

<!-- Subheading -->
<reed as="h2" 
      text="[size:3xl, weight:semibold, color:base-700]">
  Subheading
</reed>
```

### Body Text

```html
<reed as="p" 
      text="[size:base, leading:relaxed, color:base-800]">
  Comfortable reading text with good line height.
</reed>
```

### Truncated Text

```html
<reed as="div" 
      text="[truncate, whitespace:nowrap]"
      box="overflow:hidden">
  This long text will be truncated with ellipsis...
</reed>
```

### Code Block

```html
<reed as="pre" 
      text="[font:mono, size:sm, whitespace:pre]"
      face="[bg:base-100, radius:md]"
      box="[padding:4, overflow-x:auto]">
  const code = "example";
</reed>
```

### Link Styles

```html
<reed as="a" 
      text="[color:brand-a, decoration:underline]"
      text-hover="[color:brand-a-strong, decoration:none]">
  Styled link
</reed>
```

### Responsive Typography

```html
<reed as="h1" 
      text="[size:3xl, weight:bold]"
      text-tablet="[size:4xl]"
      text-screen="[size:5xl, tracking:tight]">
  Responsive heading
</reed>
```

### Gradient Text

```html
<reed as="h1" 
      text="[size:6xl, weight:bold]"
      face="[bg:gradient-primary, bg-clip:text, text-fill:transparent]">
  Gradient Text
</reed>
```

## Typography Scale

ReedSTYLE uses the Dimension Scale for consistent typography:

| Size | Multiplier | Use Case |
|------|------------|----------|
| tiny | 0.75x | Small labels, disclaimers |
| small | 0.875x | Secondary text, captions |
| normal | 1x | Body text (base) |
| large | 1.25x | Lead text, emphasis |
| huge | 1.5x | Subheadings |
| mega | 2x | Headings |
| ultra | 3x | Display, hero text |

## Smart Typography Engine

### Typography Filters

ReedSTYLE includes an advanced typography engine with automatic language detection and professional typography rules:

```html
<!-- Enable smart typography -->
<reed as="article" text="[filter:smart]" lang="de">
  "Quotes" become „German quotes" automatically.
  Three dots... become ellipsis…
  Dashes -- become proper em dashes—like this.
</reed>

<!-- Professional filter with all enhancements -->
<reed as="article" text="[filter:professional]">
  Enhanced with hyphenation, optical margins, and more
</reed>

<!-- Minimal filter for basic improvements -->
<reed as="article" text="[filter:minimal]">
  Only essential corrections
</reed>
```

### Language-Specific Rules

#### German (DIN 5008 Compliant)
```html
<reed as="article" lang="de" text="[filter:professional]">
  <!-- Automatic DIN 5008 formatting: -->
  <!-- „German quotes" instead of "quotes" -->
  <!-- Non-breaking spaces: z. B., d. h., u. a. -->
  <!-- Number formatting: 10 000 (with thin space) -->
  <!-- Units: 10 kg, 25 °C (with narrow no-break space) -->
  <!-- Currency: 29,99 € (with non-breaking space) -->
  <!-- Date: 14. März 2024 (with non-breaking space) -->
</reed>
```

#### English Variants
```html
<!-- British English -->
<reed as="article" lang="en-GB" text="[filter:smart]">
  'Single quotes' are primary in British English.
  Nested "double quotes" for secondary.
</reed>

<!-- American English -->
<reed as="article" lang="en-US" text="[filter:smart]">
  "Double quotes" are primary in American English.
  Nested 'single quotes' for secondary.
</reed>
```

#### French Typography
```html
<reed as="article" lang="fr" text="[filter:smart]">
  Les "guillemets" become « guillemets français ».
  Proper spacing before : ; ! ? punctuation.
</reed>
```

### OpenType Features

```html
<!-- Ligatures -->
<reed as="p" text="[ligatures:true]">
  Efficient office typography (fi, ff, ffi ligatures)
</reed>

<!-- Small capitals -->
<reed as="p" text="[small-caps:true]">
  SMALL CAPITALS for emphasis
</reed>

<!-- Numeric styles -->
<reed as="table" text="[numbers:tabular]">
  <!-- Tabular numbers align in columns -->
  <tr><td>1,234.56</td></tr>
  <tr><td>9,876.54</td></tr>
</reed>

<reed as="p" text="[numbers:oldstyle]">
  Text with 1234567890 oldstyle figures
</reed>

<!-- Fractions -->
<reed as="p" text="[fractions:true]">
  Recipe: 1/2 cup, 3/4 teaspoon (automatic fractions)
</reed>

<!-- Kerning -->
<reed as="h1" text="[kerning:true]">
  AVAST - Proper letter spacing
</reed>
```

### Hyphenation & Justification

```html
<!-- Enable hyphenation -->
<reed as="p" text="[hyphenate:true, align:justify]" lang="en">
  Long paragraph with automatic hyphenation for better
  text justification and even spacing between words.
</reed>

<!-- Custom hyphenation settings -->
<reed as="p" text="[hyphenate:custom, min-length:6, zone:8]">
  Custom hyphenation with minimum word length
</reed>

<!-- Optical margin alignment -->
<reed as="article" text="[hanging-punctuation:true]">
  "Quotes" hang into the margin for optical alignment
</reed>
```

### Measure & Rhythm

```html
<!-- Optimal line length (measure) -->
<reed as="article" text="[measure:narrow]">  <!-- 45-55 chars -->
  Narrow measure for sidebars
</reed>

<reed as="article" text="[measure:normal]">  <!-- 65-75 chars -->
  Optimal reading measure for body text
</reed>

<reed as="article" text="[measure:wide]">    <!-- 85-95 chars -->
  Wide measure for large screens
</reed>

<!-- Baseline grid -->
<reed as="article" text="[baseline:true, grid:8]">
  All text aligns to 8px baseline grid
</reed>
```

### Advanced Typography Examples

```html
<!-- Article with full typography enhancement -->
<reed as="article" 
      lang="de"
      text="[filter:professional, hyphenate:true, measure:normal]">
  <reed as="h1" text="[size:mega, weight:bold, kerning:true]">
    Überschrift mit perfekter Typografie
  </reed>
  
  <reed as="p" text="[size:normal, leading:relaxed, align:justify]">
    Dieser Text wird automatisch mit deutschen Anführungszeichen,
    korrekten Abständen nach DIN 5008 und professioneller
    Silbentrennung formatiert. Zahlen wie 10 000 und Einheiten
    wie 25 kg werden korrekt dargestellt.
  </reed>
  
  <reed as="blockquote" text="[size:large, style:italic, hanging-punctuation:true]">
    „Ein Zitat mit hängender Interpunktion für optische Ausrichtung."
  </reed>
</reed>

<!-- Typography for different content types -->
<reed as="code" text="[font:font-c, numbers:tabular, ligatures:false]">
  const value = 123456; // Tabular numbers, no ligatures
</reed>

<reed as="table" text="[numbers:tabular, size:small]">
  <!-- Financial data with aligned numbers -->
  <tr><td>€ 1,234.56</td></tr>
  <tr><td>€ 9,876.54</td></tr>
</reed>

<reed as="recipe" text="[fractions:true, numbers:oldstyle]">
  Add 1/2 cup flour and 3/4 tsp salt
</reed>
```

## Typography Scopes

### Weight Scale (Typography Scope)
- `thin` → `light` → `regular` → `medium` → `bold` → `black`

### Size Scale (Dimension Scope)  
- `tiny` → `small` → `normal` → `large` → `huge` → `mega` → `ultra`

### Line Height Scale
- `tight` (1.25) → `normal` (1.5) → `relaxed` (1.75) → `loose` (2.0)

## Performance

The typography engine uses:
- **Lazy processing** - Only processes visible text
- **Caching** - Stores processed text to avoid reprocessing  
- **Progressive enhancement** - CSS features work without JavaScript
- **Language detection** - Automatic from DOM tree

## Best Practices

1. **Always specify language** with `lang` attribute for proper typography
2. **Use semantic sizes** (sm, lg) over arbitrary values
3. **Enable smart typography** for professional content
4. **Test with real content** in target languages
5. **Respect user preferences** (prefers-reduced-motion, prefers-contrast)
6. **Validate spacing** with native speakers (especially for DIN 5008)
7. **Use appropriate filters** - professional for print-quality, smart for web
8. **Consider performance** - lazy load typography for large text blocks

## Next: [Architecture](201-architecture.md)