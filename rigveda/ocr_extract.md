```bash
# 1. Print page count and metadata
uv run --with pypdf python3 rigveda/ocr_extract.py --info

# 2. Extract page images for a given range (e.g. pages 1 to 5)
uv run --with pypdf --with pillow python3 rigveda/ocr_extract.py --extract-images --start 1 --end 5

# 3. Run OCR on specific pages (using Tesseract)
uv run --with pypdf --with pillow --with pytesseract python3 rigveda/ocr_extract.py --ocr --start 2 --end 4 --lang eng

# 4. Extract embedded text layer (if present)
uv run --with pypdf python3 rigveda/ocr_extract.py --text-layer --start 1 --end 10
```