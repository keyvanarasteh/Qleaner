import os, urllib.request, urllib.parse, subprocess, re

TODO_FILE = "/home/drvoid/ISU/Qleaner/TODO.md"
TARGETS = ["83", "84", "87"]

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
    subprocess.run(["git", "commit", "-m", "ci: embedded GitHub Actions for multi-platform Rust releases and finalized backend Mock Tests"])
except Exception as e: print("Git Error:", e)

# Telegram Message
token = "<YOUR_TELEGRAM_BOT_TOKEN>"
chat_id = "1426904527"
msg = """🐙 <b>Qleaner CI/CD ve Testing Tesisleri Aktif!</b>

Bir Enterprise ürünün en büyük ihtiyacı güvenilir Pipeline ve Test ortamlarıdır. Qleaner artık bunlara sahip:

- <b>Mock Dosya Sistemleri (TempDir):</b> `clean_items` ve `scanner` mekanizmalarını test etmek için ana sistemi kirletmeden simüle edilmiş dizinlerle çalışan edge-case test birimleri `cargo test` üzerinden başarıyla koşturuluyor. 
- <b>GitHub Actions Multi-Build:</b> `.github/workflows/build.yml` oluşturuldu. Artık Windows (MSVC), macOS (Darwin) ve Linux (AppImage) binary dosyaları tamamen bulutta otomatik olarak native bir şekilde derlenip Release olarak sunulacak!

Artık Qleaner'i son kullanıcılara ve kurumsal firmalara sunabileceğimiz bir DevOps dağıtım bandımız (Belt) var! 🚀

#DevOps #GitHubActions #RustTesting #Qleaner"""

data = urllib.parse.urlencode({"chat_id": chat_id, "text": msg, "parse_mode": "HTML"}).encode("utf-8")
try:
    req = urllib.request.Request(f"https://api.telegram.org/bot{token}/sendMessage", data=data)
    urllib.request.urlopen(req)
    print("Telegram payload successfully posted")
except Exception as e:
    print("Telegram Error:", e)
