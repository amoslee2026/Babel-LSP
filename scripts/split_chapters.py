#!/usr/bin/env python3
"""按章节拆分IEEE Std 1800-2023 Markdown文档"""

import re
from pathlib import Path

def clean_content(text: str) -> str:
    """清理冗余信息"""
    # 移除页眉页脚
    patterns_to_remove = [
        r'IEEE Std 1800™-2023\n.*?Language\n',
        r'1800-2023\n.*?Language\n',
        r'Copyright © \d+ IEEE\. All rights reserved\.?',
        r'PWNED Restrictions apply\.?',
        r'IEEE Standard for SystemVerilog—.*?Language',
        r'This is an unapproved draft.*?',
    ]
    for pattern in patterns_to_remove:
        text = re.sub(pattern, '', text, flags=re.MULTILINE)

    # 移除多余的页码标记，保留章节页码
    text = re.sub(r'\n\d+\s*$', '', text)
    text = re.sub(r'\n{3,}', '\n\n', text)
    return text.strip()

def get_chapter_number(title: str) -> tuple:
    """解析章节编号"""
    # 正常章节: "## 1. Overview" -> (1, "Overview")
    match = re.match(r'^## (\d+)\.\s+(.+)$', title)
    if match:
        return ('chapter', int(match.group(1)), match.group(2))

    # 附录: "## Annex A" -> ('annex', 'A', title)
    match = re.match(r'^## Annex ([A-Z])', title)
    if match:
        return ('annex', match.group(1), title.replace('## ', ''))

    return None

def split_by_chapters(md_path: str, output_dir: str):
    """按章节拆分文档"""
    output_path = Path(output_dir)
    output_path.mkdir(parents=True, exist_ok=True)

    with open(md_path, 'r', encoding='utf-8') as f:
        content = f.read()

    lines = content.split('\n')
    chapters = []
    current_chapter = None
    current_content = []

    # 找到真正的第一章开始位置（跳过前言）
    first_chapter_line = None
    for i, line in enumerate(lines):
        if re.match(r'^## 1\. Overview$', line):
            first_chapter_line = i
            break

    if first_chapter_line is None:
        print("Cannot find first chapter")
        return

    # 从第一章开始处理
    for i, line in enumerate(lines[first_chapter_line:], start=first_chapter_line):
        # 检查是否是章节标题（排除误识别的年份）
        chapter_info = None
        if re.match(r'^## \d+\.\s+', line) and not re.match(r'^## (19|20)\d+\.\s+', line):
            chapter_info = get_chapter_number(line)
        elif re.match(r'^## Annex [A-Z]', line):
            chapter_info = get_chapter_number(line)

        if chapter_info:
            # 保存上一章节
            if current_chapter and current_content:
                chapters.append((current_chapter, '\n'.join(current_content)))

            # 开始新章节
            current_chapter = chapter_info
            current_content = [line]
        elif current_chapter:
            current_content.append(line)

    # 保存最后一个章节
    if current_chapter and current_content:
        chapters.append((current_chapter, '\n'.join(current_content)))

    # 写入文件
    for chapter_info, text in chapters:
        type_, num, title = chapter_info

        if type_ == 'chapter':
            # 清理标题格式
            clean_title = re.sub(r'\.{2,}\s*\d+', '', title).strip()
            filename = f"ch{num:02d}_{clean_title.replace(' ', '_').replace('/', '_')}.md"
            header = f"# Chapter {num}: {clean_title}\n\n"
        else:
            clean_title = title.replace('(normative)', '').replace('(informative)', '').strip()
            filename = f"annex_{num}_{clean_title.replace(' ', '_')}.md"
            header = f"# Annex {num}: {title}\n\n"

        clean_text = clean_content(text)

        filepath = output_path / filename
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(header)
            f.write(clean_text)

        print(f"Created: {filename} ({len(clean_text)} chars)")

    # 创建索引文件
    index_path = output_path / "INDEX.md"
    with open(index_path, 'w', encoding='utf-8') as f:
        f.write("# IEEE Std 1800-2023 - SystemVerilog Standard\n\n")
        f.write("## Chapters\n\n")
        for chapter_info, _ in chapters:
            type_, num, title = chapter_info
            if type_ == 'chapter':
                clean_title = re.sub(r'\.{2,}\s*\d+', '', title).strip()
                filename = f"ch{num:02d}_{clean_title.replace(' ', '_').replace('/', '_')}.md"
                f.write(f"- [Chapter {num}: {clean_title}](ch{num:02d}_{clean_title.replace(' ', '_').replace('/', '_')}.md)\n")
            else:
                filename = f"annex_{num}_{title.replace(' ', '_')}.md"
                f.write(f"- [Annex {num}: {title}](annex_{num}_{title.replace(' ', '_')}.md)\n")

    print(f"\nTotal: {len(chapters)} chapters created")
    print(f"Index: {index_path}")

if __name__ == "__main__":
    md_path = "IEEE_Std/IEEE_Std_1800-2023.md"
    output_dir = "IEEE_Std/chapters"
    split_by_chapters(md_path, output_dir)