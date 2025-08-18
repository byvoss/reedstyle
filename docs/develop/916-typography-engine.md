# Ticket #916: Typography Engine (DIN 5008)

## Status: 📋 Planned

## Description
Implement smart typography engine with automatic language detection and professional typography rules including DIN 5008 compliance for German.

## Requirements

### Core Features

1. **Automatic Language Detection**
   - Walk DOM tree for `lang` attributes
   - Apply language-specific rules
   - Support inheritance from parent elements

2. **Typography Filters**
   - `minimal` - Basic corrections only
   - `smart` - Smart quotes, dashes, ellipsis
   - `professional` - All enhancements including hyphenation

3. **Language Support**
   - German (DIN 5008 compliant)
   - English (US & GB variants)
   - French (guillemets and spacing)
   - Extensible for more languages

### Translation to Reed Element System

#### Old System (data-r-typo)
```html
<!-- Old attribute system -->
<p data-r-typo="[filter:smart, size:large, weight:bold]">
```

#### New System (reed element)
```html
<!-- New reed element system -->
<reed as="p" text="[filter:smart, size:large, weight:bold]">
```

### German Rules (DIN 5008)

```rust
// src/typography/german.rs
pub struct GermanRules {
    // Quotation marks
    quotes: QuoteRules {
        primary: ('„', '"'),      // German quotes
        secondary: ('‚', '''),     // Single quotes
    },
    
    // Spaces per DIN 5008
    spaces: SpaceRules {
        before_unit: '\u{202F}',     // Narrow no-break space
        in_number: '\u{202F}',        // Thousands separator
        before_currency: '\u{00A0}',  // Before €
        after_abbr: '\u{00A0}',       // z. B.
    },
    
    replacements: vec![
        // Number ranges with en dash
        (r"(\d+)-(\d+)", "$1–$2"),
        
        // Abbreviations with NBSP
        (r"z\.\s+B\.", "z.\u{00A0}B."),
        (r"d\.\s+h\.", "d.\u{00A0}h."),
        
        // Numbers and units
        (r"(\d+)\s+(kg|km|cm|mm)", "$1\u{202F}$2"),
        (r"(\d+)\s+(€|EUR)", "$1\u{00A0}$2"),
    ],
}
```

### CSS Generation

```css
/* Typography filter classes */
reed[text*="filter:smart"] {
  /* Processed by JS engine */
}

/* OpenType features */
reed[text*="ligatures:true"] {
  font-variant-ligatures: common-ligatures;
  font-feature-settings: "liga" 1, "clig" 1;
}

reed[text*="small-caps:true"] {
  font-variant-caps: small-caps;
  font-feature-settings: "smcp" 1;
}

reed[text*="numbers:tabular"] {
  font-variant-numeric: tabular-nums;
  font-feature-settings: "tnum" 1;
}

reed[text*="numbers:oldstyle"] {
  font-variant-numeric: oldstyle-nums;
  font-feature-settings: "onum" 1;
}

/* Hyphenation */
reed[text*="hyphenate:true"] {
  hyphens: auto;
  -webkit-hyphens: auto;
  hyphenate-limit-lines: 2;
  hyphenate-limit-last: always;
}

/* Hanging punctuation */
reed[text*="hanging-punctuation:true"] {
  hanging-punctuation: first last;
}

/* Line height (leading) */
reed[text*="leading:tight"] { line-height: 1.25; }
reed[text*="leading:normal"] { line-height: 1.5; }
reed[text*="leading:relaxed"] { line-height: 1.75; }
reed[text*="leading:loose"] { line-height: 2; }

/* Measure (line length) */
reed[text*="measure:narrow"] { max-width: 45ch; }
reed[text*="measure:normal"] { max-width: 65ch; }
reed[text*="measure:wide"] { max-width: 85ch; }
```

### JavaScript Implementation

```javascript
// dist/reedstyle.js - Typography module
class TypographyEngine {
  constructor() {
    this.rules = new Map([
      ['de', germanRules],
      ['en-GB', britishRules],
      ['en-US', americanRules],
      ['fr', frenchRules]
    ]);
  }
  
  init() {
    // Process all reed elements with text filters
    const elements = document.querySelectorAll('reed[text*="filter:"]');
    
    elements.forEach(element => {
      this.processElement(element);
    });
    
    // Watch for new elements
    this.observeChanges();
  }
  
  processElement(element) {
    const lang = this.detectLanguage(element);
    const textAttr = element.getAttribute('text');
    const filter = this.extractFilter(textAttr);
    
    if (filter && this.rules.has(lang)) {
      this.applyRules(element, this.rules.get(lang), filter);
    }
  }
  
  extractFilter(textAttr) {
    if (!textAttr) return null;
    const match = textAttr.match(/filter:(\w+)/);
    return match ? match[1] : null;
  }
  
  detectLanguage(element) {
    // Walk up DOM for lang attribute
    let current = element;
    while (current) {
      if (current.lang) return current.lang;
      current = current.parentElement;
    }
    return document.documentElement.lang || 'en';
  }
  
  applyRules(element, rules, filter) {
    const walker = document.createTreeWalker(
      element,
      NodeFilter.SHOW_TEXT,
      null
    );
    
    let node;
    while (node = walker.nextNode()) {
      let text = node.textContent;
      
      // Apply based on filter level
      if (filter === 'smart' || filter === 'professional') {
        text = this.applySmartQuotes(text, rules);
        text = this.applyDashes(text, rules);
        text = this.applySpacing(text, rules);
      }
      
      if (filter === 'professional') {
        text = this.applyAdvancedRules(text, rules);
      }
      
      if (text !== node.textContent) {
        node.textContent = text;
      }
    }
  }
}

// Auto-init when ReedStyle loads
if (window.ReedStyle) {
  window.ReedStyle.typography = new TypographyEngine();
  window.ReedStyle.typography.init();
}
```

### Configuration

```yaml
# reedstyle.typography.yaml (optional)
typography:
  default_filter: smart
  
  features:
    ligatures: true
    kerning: true
    hyphenation: auto
    hanging_punctuation: false
    
  languages:
    de:
      quotes: ["„", """]
      single_quotes: ["‚", "'"]
      
    en-US:
      quotes: [""", """]
      single_quotes: ["'", "'"]
      
    en-GB:  
      quotes: ["'", "'"]
      single_quotes: [""", """]
      
    fr:
      quotes: ["« ", " »"]
      single_quotes: [""", """]
```

## Acceptance Criteria

- [ ] Language detection from DOM tree
- [ ] DIN 5008 compliance for German
- [ ] Smart quotes for all supported languages
- [ ] Proper dash replacements (em/en dashes)
- [ ] Non-breaking spaces per language rules
- [ ] OpenType feature support
- [ ] Hyphenation control
- [ ] Performance optimization (lazy loading)
- [ ] Cache processed text
- [ ] Works without JavaScript (CSS features only)

## Testing

```html
<!-- Test German DIN 5008 -->
<reed as="article" lang="de" text="[filter:professional]">
  "Quotes" werden zu „echten Anführungszeichen".
  Abkürzungen wie z. B. diese bekommen geschützte Leerzeichen.
  Zahlen wie 10 000 und Einheiten wie 10 kg werden korrekt formatiert.
  Preise wie 29,99 € folgen DIN 5008.
</reed>

<!-- Test English variants -->
<reed as="p" lang="en-GB" text="[filter:smart]">
  'British quotes' use single quotes primarily.
</reed>

<reed as="p" lang="en-US" text="[filter:smart]">
  "American quotes" use double quotes primarily.
</reed>

<!-- Test French -->
<reed as="p" lang="fr" text="[filter:smart]">
  Les "guillemets" deviennent « guillemets français ».
</reed>

<!-- Test OpenType features -->
<reed as="h1" text="[ligatures:true, small-caps:true]">
  Efficient Typography
</reed>

<!-- Test tabular numbers -->
<reed as="table" text="[numbers:tabular]">
  <tr><td>1,234.56</td></tr>
  <tr><td>9,876.54</td></tr>
</reed>
```

## Dependencies

- Ticket #907 (Namespace CSS Generation) - Text namespace
- Ticket #913 (JavaScript Optional API) - Core JS structure

## Blocks

None

## Performance Considerations

1. **Lazy Processing** - Only process visible text
2. **Caching** - Cache processed text to avoid reprocessing
3. **Intersection Observer** - Process as elements become visible
4. **Text Node Walking** - Efficient DOM traversal
5. **Minimal Reflows** - Batch text updates

## Notes

- Typography engine is progressive enhancement
- CSS features work without JavaScript
- JavaScript adds smart quotes and spacing
- Language detection is automatic but can be overridden
- DIN 5008 compliance is critical for German market