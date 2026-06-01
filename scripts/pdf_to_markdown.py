#!/usr/bin/env python3
"""将IEEE Std 1800-2023 PDF转换为Markdown格式"""

import pdfplumber
import re
import os
from pathlib import Path
from tqdm import tqdm

def clean_text(text: str) -> str:
    """清理提取的文本"""
    if not text:
        return ""
    # 移除页码标记
    text = re.sub(r'\n\d+\n', '\n', text)
    # 移除"PWNED Restrictions apply."等版权标记
    text = re.sub(r'PWNED Restrictions apply\.', '', text)
    text = re.sub(r'Copyright © 2024 IEEE\. All rights reserved\.', '', text)
    # 清理多余空白
    text = re.sub(r'\n{3,}', '\n\n', text)
    return text.strip()

def format_as_markdown(text: str, page_num: int) -> str:
    """将文本格式化为Markdown"""
    if not text:
        return ""

    lines = text.split('\n')
    md_lines = []

    for line in lines:
        line = line.strip()
        if not line:
            md_lines.append('')
            continue

        # 检测章节标题 (如 "1. Overview", "Annex A")
        if re.match(r'^[A-Z]?\d+\.\s+[A-Z]', line) or re.match(r'^Annex\s+[A-Z]', line):
            md_lines.append(f'## {line}')
            continue

        # 检测小节标题 (如 "1.1 General", "1.2 Overview")
        if re.match(r'^\d+\.\d+\s+', line):
            md_lines.append(f'### {line}')
            continue

        # 检测更小节标题 (如 "1.1.1 Something")
        if re.match(r'^\d+\.\d+\.\d+\s+', line):
            md_lines.append(f'#### {line}')
            continue

        # 检测表格行（包含多个空格分隔的内容）
        if '  ' in line and not line.startswith('#'):
            # 可能是表格，用管道符格式化
            cells = [c.strip() for c in line.split('  ') if c.strip()]
            if len(cells) > 1:
                md_lines.append('| ' + ' | '.join(cells) + ' |')
                continue

        md_lines.append(line)

    return '\n'.join(md_lines)

def extract_page(pdf, page_num: int) -> str:
    """提取单页内容"""
    try:
        page = pdf.pages[page_num - 1]  # pdfplumber使用0索引
        text = page.extract_text()
        return clean_text(text)
    except Exception as e:
        print(f"Error extracting page {page_num}: {e}")
        return ""

def get_toc_structure(pdf):
    """获取目录结构"""
    toc = []
    # 目录通常在第12-15页
    for i in range(11, 16):
        text = extract_page(pdf, i + 1)
        if text:
            # 解析目录项
            for line in text.split('\n'):
                match = re.match(r'^(\d+\.?\s+.+?)\s+(\d+)$', line.strip())
                if match:
                    title, page = match.groups()
                    toc.append((title.strip(), int(page)))
    return toc

def convert_pdf_to_markdown(pdf_path: str, output_dir: str, batch_size: int = 50):
    """将PDF转换为Markdown"""
    output_path = Path(output_dir)
    output_path.mkdir(parents=True, exist_ok=True)

    print(f"Processing: {pdf_path}")
    print(f"Output directory: {output_dir}")

    with pdfplumber.open(pdf_path) as pdf:
        total_pages = len(pdf.pages)
        print(f"Total pages: {total_pages}")

        # 创建主Markdown文件
        main_md = output_path / "IEEE_Std_1800-2023.md"

        # 分批处理
        all_content = []

        for batch_start in tqdm(range(0, total_pages, batch_size), desc="Processing batches"):
            batch_end = min(batch_start + batch_size, total_pages)

            batch_content = []
            for page_num in range(batch_start + 1, batch_end + 1):
                text = extract_page(pdf, page_num)
                if text:
                    md_text = format_as_markdown(text, page_num)
                    batch_content.append(f"\n<!-- Page {page_num} -->\n\n{md_text}\n")

            all_content.extend(batch_content)

            # 定期保存进度
            if batch_end % 200 == 0 or batch_end == total_pages:
                with open(main_md, 'w', encoding='utf-8') as f:
                    f.write("# IEEE Std 1800-2023\n\n")
                    f.write("# IEEE Standard for SystemVerilog—Unified Hardware Design, Specification, and Verification Language\n\n")
                    f.write(f"**Total Pages: {total_pages}**\n\n")
                    f.write("---\n\n")
                    f.write('\n'.join(all_content))
                print(f"Saved progress at page {batch_end}")

        print(f"\nConversion complete! Output saved to: {main_md}")
        print(f"File size: {os.path.getsize(main_md) / 1024 / 1024:.2f} MB")

if __name__ == "__main__":
    pdf_path = "refs/IEEE_Std_1800-2023.pdf"
    output_dir = "IEEE_Std"
    convert_pdf_to_markdown(pdf_path, output_dir, batch_size=100)