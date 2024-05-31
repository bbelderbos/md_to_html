import os
import glob
import re
import sys

import markdown2


def convert_md_to_html(md_files, output_dir):
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    index_content = "<html><head><title>Index of Newbies Bites Part II</title></head><body><h1>Index of Newbie Bites Part II</h1><ul>"

    md_files.sort()

    for md_file in md_files:
        subdir_name = os.path.basename(os.path.dirname(md_file))

        if not subdir_name[0].isdigit():
            continue

        html_file_name = f"{subdir_name}.html"

        with open(md_file, "r", encoding="utf-8") as file:
            md_content = file.read()
            html_content = markdown2.markdown(md_content)

            html_file_path = os.path.join(output_dir, html_file_name)

            with open(html_file_path, "w", encoding="utf-8") as html_file:
                html_file.write(
                    "<html><head><title>{}</title></head><body>".format(subdir_name)
                )
                html_file.write(html_content)
                html_file.write("</body></html>")

            index_content += '<li><a href="{}">{}</a></li>\n'.format(
                html_file_name, subdir_name
            )

    index_content += "</ul></body></html>"

    index_file_path = os.path.join(output_dir, "index.html")
    with open(index_file_path, "w", encoding="utf-8") as index_file:
        index_file.write(index_content)

    print(f"HTML pages and index generated in {output_dir}")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python gen_html.py <path_to_bite_directories>")
        sys.exit(1)

    path = sys.argv[1]

    md_files = glob.glob(f"{path}/*/bite.md")

    output_dir = "html_pages"
    os.makedirs(output_dir, exist_ok=True)

    convert_md_to_html(md_files, output_dir)
