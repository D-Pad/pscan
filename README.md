# Project Scan 
This script is a grep-like script written in Rust. A tool that I often use on 
large repo's when I can't remember which python file I left a stray `print()`
statement in.

**Example use:**
<pre><code>
<span>$ pscan -ri ./text_files mary</span> 
<span style="color:#37FF00;">Matches found in ./text_files/line_break_file.txt:</span>
<span style="color:#00EAFF;">19|</span> <span style="color:#FFE100">Mary</span> had a little lamb, 
<span style="color:#00EAFF;">21|</span> And everywhere that <span style="color:#FFE100">Mary</span> went,
</code></pre>

