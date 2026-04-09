import os, urllib.request, urllib.parse, subprocess, re

TODO_FILE = "/home/drvoid/ISU/Qleaner/TODO.md"
TARGETS = ["61", "63", "67", "72"]

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

try:
    subprocess.run(["git", "add", "-A"])
    subprocess.run(["git", "commit", "-m", "feat: applied stringent security profiling, CSP strict directives, and zero-panic mutex locking recoveries"])
except Exception as e: print("Git Error:", e)

token = "<YOUR_TELEGRAM_BOT_TOKEN>"
chat_id = "1426904527"
msg = """🛡️ <b>Qleaner SecOps Updates & Backend Hardening Gerçekleşti!</b>

Uygulamanın mimari çekirdeğindeki (Tier 1) kritik güvenlik ve kararlılık problemleri çözüldü:

- <b>Strict CSP:</b> Tauri `security` blokları kullanılarak olası tüm XSS, Data Exfiltration durumları sıfıra indirildi (`default-src self`, `object-src none`).
- <b>Zero-Panic Mutexes:</b> Multi-threading mekanizmasında nadiren çökmelere yol açabilecek `unwrap()` kilitleri yakalanıp (`unwrap_or_else`), izole edildi! Böylece sistem stabilitesi güçlendi.
- <b>Network / Sandbox Audit:</b> Herhangi bir harici http modülü (reqwest vs) derlemeye eklenmedi; Tauri IPC `fs` özellikleri de tam sandboxed tutularak uygulamanın Frontend üzerinden Root path manipülasyonuna uğraması doğal yollarla engellendi.

MVP`den Enterprise Security seviyesine geçişin en büyük adımlarını attık. Sırada diğer CI/CD yapı taşları var. 🚀

#Rust #SystemSecurity #DevOps #Qleaner"""

data = urllib.parse.urlencode({"chat_id": chat_id, "text": msg, "parse_mode": "HTML"}).encode("utf-8")
try:
    req = urllib.request.Request(f"https://api.telegram.org/bot{token}/sendMessage", data=data)
    urllib.request.urlopen(req)
    print("Telegram payload successfully posted")
except Exception as e:
    print("Telegram Error:", e)
