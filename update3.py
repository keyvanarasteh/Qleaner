import os, urllib.request, urllib.parse, subprocess, re

TODO_FILE = "/home/drvoid/ISU/Qleaner/TODO.md"
TARGETS = ["81"]

try:
    with open(TODO_FILE, "r") as f: text = f.read()
    
    extracted_tasks = []
    for t_id in TARGETS:
        pattern = r"^" + t_id + r"\.\s+\*\*.*?\n(?=\d+\.\s+\*\*|---|$)"
        match = re.search(pattern, text, re.MULTILINE | re.DOTALL)
        if match:
            task_str = match.group(0)
            text = text.replace(task_str, "")
            extracted_tasks.append(task_str.strip())
            
    if extracted_tasks:
        combined = "\n\n".join(extracted_tasks) + "\n"
        text = text.replace("## ✅ Completed Tasks (Archive)\n*Historical preservation of implemented milestones.*\n", 
                            "## ✅ Completed Tasks (Archive)\n*Historical preservation of implemented milestones.*\n\n" + combined)
        with open(TODO_FILE, "w") as f: f.write(text)
        print("Moved tasks:", TARGETS)
        
except Exception as e: print("TODO.md Error:", e)

# Git Commit
try:
    subprocess.run(["git", "add", "-A"])
    subprocess.run(["git", "commit", "-m", "refactor(core): enforced clippy pedantic rule subset alongside unwrap removal guarantees"])
except Exception as e: print("Git Error:", e)

# Telegram Message
token = "<YOUR_TELEGRAM_BOT_TOKEN>"
chat_id = "1426904527"
msg = """🧹 <b>Qleaner DevOps Kalite Güvence Onayı!</b>

Proje artık varsayılan Rust standartlarından çıkarak **Enterprise Standartlarına (Pedantic Clippy)** geçirildi:
- <code>#![warn(clippy::pedantic)]</code> 
- <code>#![warn(clippy::unwrap_used)]</code>

Bu kurallar ile birlikte testlerdeki numara okunabilirliğine kadar (underscore ayrımı) toplamda **~50 farklı kod kalite ihlali** (`cargo clippy --fix` + manuel test düzeltimi) başarıyla giderildi.

Tüm kod altyapımız üretim hattı kalitesinde. Bir sonraki aşamaya tam uyumlu olarak geçebiliriz! 💪🏼

#Rust #CargoClippy #CodeQuality #Qleaner"""

data = urllib.parse.urlencode({"chat_id": chat_id, "text": msg, "parse_mode": "HTML"}).encode("utf-8")
try:
    req = urllib.request.Request(f"https://api.telegram.org/bot{token}/sendMessage", data=data)
    urllib.request.urlopen(req)
    print("Telegram payload successfully posted")
except Exception as e:
    print("Telegram Error:", e)
