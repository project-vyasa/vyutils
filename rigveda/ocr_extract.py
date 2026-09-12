#!/usr/bin/env python3
"""
ocr_extract.py: Reusable PDF text and image extraction tool for Rgveda-Pratisakhya.

Usage:
  # Inspect PDF metadata & page count:
  python3 rigveda/ocr_extract.py --info

  # Extract page images (e.g. pages 1 to 5):
  python3 rigveda/ocr_extract.py --extract-images --start 1 --end 5

  # Run OCR on specified page range using Tesseract:
  python3 rigveda/ocr_extract.py --ocr --start 1 --end 5 --lang eng+san

  # Export text layer (if present):
  python3 rigveda/ocr_extract.py --text-layer --start 1 --end 10
"""

import argparse
import io
import os
import sys
from pathlib import Path

def get_reader(pdf_path: Path):
    try:
        import pypdf
        return pypdf.PdfReader(str(pdf_path))
    except ImportError:
        print("pypdf is required. Run with: uv run --with pypdf ...", file=sys.stderr)
        sys.exit(1)

def print_info(pdf_path: Path):
    reader = get_reader(pdf_path)
    print(f"File: {pdf_path.name}")
    print(f"Total Pages: {len(reader.pages)}")
    print("Metadata:")
    for k, v in (reader.metadata or {}).items():
        print(f"  {k}: {v}")

def extract_images(pdf_path: Path, output_dir: Path, start: int, end: int):
    reader = get_reader(pdf_path)
    output_dir.mkdir(parents=True, exist_ok=True)
    total = len(reader.pages)
    end = min(end, total)

    try:
        from PIL import Image
    except ImportError:
        print("pillow is required. Run with: uv run --with pypdf --with pillow ...", file=sys.stderr)
        sys.exit(1)

    print(f"Extracting images for pages {start} to {end}...")
    for p_num in range(start, end + 1):
        page = reader.pages[p_num - 1]
        for img_idx, img in enumerate(page.images):
            out_name = output_dir / f"page_{p_num:04d}_{img_idx}.png"
            pil_img = Image.open(io.BytesIO(img.data))
            pil_img.save(out_name)
            print(f"  Saved: {out_name} (size: {pil_img.size}, mode: {pil_img.mode})")

def extract_text_layer(pdf_path: Path, output_file: Path | None, start: int, end: int):
    reader = get_reader(pdf_path)
    total = len(reader.pages)
    end = min(end, total)
    out_lines = []

    print(f"Extracting embedded text layer for pages {start} to {end}...")
    for p_num in range(start, end + 1):
        page = reader.pages[p_num - 1]
        text = page.extract_text() or ""
        out_lines.append(f"--- PAGE {p_num} ---\n{text}\n")

    full_text = "\n".join(out_lines)
    if output_file:
        output_file.write_text(full_text, encoding="utf-8")
        print(f"Text written to {output_file}")
    else:
        print(full_text)

def run_ocr(pdf_path: Path, output_file: Path | None, start: int, end: int, lang: str):
    try:
        import pytesseract
        from PIL import Image
    except ImportError:
        print("pytesseract and pillow required. Run with: uv run --with pypdf --with pillow --with pytesseract ...", file=sys.stderr)
        sys.exit(1)

    reader = get_reader(pdf_path)
    total = len(reader.pages)
    end = min(end, total)
    out_lines = []

    print(f"Running OCR (lang={lang}) for pages {start} to {end}...")
    for p_num in range(start, end + 1):
        page = reader.pages[p_num - 1]
        page_text = []
        for img_idx, img in enumerate(page.images):
            pil_img = Image.open(io.BytesIO(img.data))
            try:
                txt = pytesseract.image_to_string(pil_img, lang=lang)
            except Exception as e:
                txt = f"[OCR Error: {e}]"
            page_text.append(txt)

        combined = "\n".join(page_text)
        out_lines.append(f"=== PAGE {p_num} ===\n{combined}\n")
        print(f"  Processed page {p_num}/{end}")

    full_text = "\n".join(out_lines)
    if output_file:
        output_file.write_text(full_text, encoding="utf-8")
        print(f"OCR results saved to {output_file}")
    else:
        print(full_text)

def main():
    parser = argparse.ArgumentParser(description="Rgveda-Pratisakhya OCR & Text Extractor")
    parser.add_argument("--pdf", type=Path, default=Path("rigveda/2015.409802.Rgveda-Pratisakhya.pdf"), help="Input PDF path")
    parser.add_argument("--info", action="store_true", help="Print PDF metadata and exit")
    parser.add_argument("--extract-images", action="store_true", help="Extract raw scanned images")
    parser.add_argument("--text-layer", action="store_true", help="Extract existing text layer")
    parser.add_argument("--ocr", action="store_true", help="Run OCR on scanned pages")
    parser.add_argument("--start", type=int, default=1, help="Start page (1-indexed)")
    parser.add_argument("--end", type=int, default=5, help="End page (inclusive)")
    parser.add_argument("--lang", type=str, default="eng", help="Tesseract language code (e.g. eng, san, hin, deva)")
    parser.add_argument("--out-dir", type=Path, default=Path("rigveda/extracted_images"), help="Image output directory")
    parser.add_argument("--out-file", type=Path, default=None, help="Text/OCR output file")

    args = parser.parse_args()

    if not args.pdf.exists():
        print(f"Error: PDF not found at {args.pdf}", file=sys.stderr)
        sys.exit(1)

    if args.info or len(sys.argv) == 1:
        print_info(args.pdf)
        if len(sys.argv) == 1:
            print("\nRun with --help to see all extraction and OCR options.")
        return

    if args.text_layer:
        extract_text_layer(args.pdf, args.out_file, args.start, args.end)
    elif args.extract_images:
        extract_images(args.pdf, args.out_dir, args.start, args.end)
    elif args.ocr:
        run_ocr(args.pdf, args.out_file, args.start, args.end, args.lang)
    else:
        print("No action specified. Choose --info, --text-layer, --extract-images, or --ocr.")

if __name__ == "__main__":
    main()
