# md2pdf Examples

This directory contains example files demonstrating various features of md2pdf.

## Files

### custom-style.css

A custom CSS example demonstrating:
- Brand color customization (blue theme)
- Custom typography (Georgia for body, Arial for headings)
- Custom table styling with shadow effects
- Dark-themed code blocks
- Professional blockquote styling
- Proper page break control

#### Usage

```bash
md2pdf your-document.md --css examples/custom-style.css -o output.pdf
```

## Creating Your Own Custom CSS

To create your own custom styling:

1. Copy `custom-style.css` as a starting point
2. Modify colors, fonts, and spacing to match your brand
3. Ensure you include page break control rules:
   - `page-break-inside: avoid` for tables, code blocks, blockquotes
   - `page-break-after: avoid` for headings
   - `orphans` and `widows` properties for paragraphs

### Key CSS Properties for Page Breaks

```css
/* Prevent page breaks inside elements */
table, pre, blockquote {
    page-break-inside: avoid;
    break-inside: avoid;
}

/* Keep headings with following content */
h1, h2, h3 {
    page-break-after: avoid;
    break-after: avoid;
}

/* Control orphan/widow lines */
p {
    orphans: 3;  /* Min lines at bottom of page */
    widows: 3;   /* Min lines at top of page */
}
```

### Supported @page Rules

```css
@page {
    size: A4;           /* Paper size */
    margin: 2cm;        /* Page margins */

    /* Optional headers/footers */
    @top-center {
        content: "Document Title";
    }

    @bottom-right {
        content: counter(page);
    }
}
```

## Tips for Professional PDFs

1. **Font Selection**: Use web-safe fonts or system fonts
   - Sans-serif: Arial, Helvetica, Segoe UI
   - Serif: Georgia, Times New Roman
   - Monospace: Consolas, Monaco, Courier New

2. **Color Scheme**: Choose a consistent color palette
   - Primary brand color for headings
   - Neutral colors for body text
   - High contrast for readability

3. **Spacing**: Use consistent margins and padding
   - Line height: 1.5-1.7 for readability
   - Paragraph spacing: 1em
   - Section spacing: 2-3em

4. **Tables**: Style for clarity
   - Header with distinct background
   - Alternating row colors
   - Clear borders
   - Adequate padding

5. **Code Blocks**: Make code readable
   - Monospace font
   - Background contrast
   - Left border accent
   - Proper padding
