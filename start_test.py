#!/usr/bin/env python3

import os
import pathlib
import shutil
import smtplib
import stat
import subprocess
import sys
import tempfile
import time

ADDR_FROM = "test.from@example.org"
DEFAULT_PORT = 2525


def fail(message):
    print(message, file=sys.stderr)
    sys.exit(1)


def cp_tmp_file(path, executable=False):
    file = tempfile.NamedTemporaryFile(suffix=f"-{path.name}", delete=False)
    with open(path, mode="rb") as f:
        file.write(f.read())
        file.flush()
    flags = stat.S_IRUSR | stat.S_IWUSR | stat.S_IRGRP | stat.S_IROTH
    if executable:
        flags = flags | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH
    os.chmod(file.name, flags)
    file.close()
    return file


def get_filter_path(test_dir, target):
    filter_path = test_dir / "target" / target / "filter-sake"
    filter_path = cp_tmp_file(filter_path, executable=True).name
    return filter_path


def get_opensmtpd_config(port, filter_exe, maildir_path):
    cfg_content = f"""# OpenSMTPD test configuration

# DKIM filter
filter "sake" proc-exec "{filter_exe} --address 'a@example.org:11voiefK5PgCX5F1TTcuoQ==' --address 'b:11voiefK5PgCX5F1TTcuoQ=='"

# Tables
table domains {{ "example.org", "example.com" }}
table vusers {{ "test" = "1000:100:{maildir_path}", "b" = "1000:100:{maildir_path}" }}
table aliases {{ "a" = "test" }}

# Listening
listen on 127.0.0.1 port {port} hostname localhost filter "sake"
listen on ::1 port {port} hostname localhost filter "sake"

# Delivering
action "deliver" maildir userbase <vusers> alias <aliases>
match from any for domain <domains> action "deliver"
"""
    cfg_file = tempfile.NamedTemporaryFile(prefix="smtpd-", suffix=".conf")
    cfg_file.write(cfg_content.encode())
    cfg_file.flush()
    return cfg_file


def send_msg(smtp, to_addr, is_valid):
    msg = f"From: {ADDR_FROM}\r\nTo: {to_addr}\r\n\r\ntest"
    try:
        smtp.sendmail(ADDR_FROM, to_addr, msg)
        if not is_valid:
            print(f"{to_addr}: accepted: error")
            return False
        print(f"{to_addr}: accepted: ok")
    except smtplib.SMTPRecipientsRefused:
        if is_valid:
            print(f"{to_addr}: refused: error")
            return False
        print(f"{to_addr}: refused: ok")
    return True


def start_opensmtpd(cfg_path):
    args = [
        shutil.which("sudo"),
        shutil.which("smtpd"),
        "-d",
        "-f",
        cfg_path.name,
    ]
    p = subprocess.Popen(
        args,
        stdin=subprocess.DEVNULL,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    time.sleep(5)
    return p.pid


def kill_opensmtpd(pid):
    if pid is not None:
        subprocess.Popen([shutil.which("sudo"), shutil.which("kill"), f"{pid}"])


def get_maildir():
    maildir = tempfile.TemporaryDirectory(prefix="Maildir_")
    flags = (
        stat.S_IRUSR
        | stat.S_IWUSR
        | stat.S_IXUSR
        | stat.S_IRGRP
        | stat.S_IWGRP
        | stat.S_IXGRP
        | stat.S_IROTH
        | stat.S_IWOTH
        | stat.S_IXOTH
    )
    os.chmod(maildir.name, flags)
    return maildir


def start_tests(test_dir, smtp_port):
    to_addrs = [
        ("test@example.org", True),
        ("test@nope.example.org", False),
        ("a@example.com", True),
        ("a@example.org", False),
        ("a+invalid@example.org", False),
        ("a+invalid+input@example.org", False),
        ("b@example.org", False),
    ]
    maildir = get_maildir()
    filter_cmd = get_filter_path(test_dir, "debug")
    pid_smtpd = None
    has_errors = False
    try:
        cfg_path = get_opensmtpd_config(smtp_port, filter_cmd, maildir.name)
        pid_smtpd = start_opensmtpd(cfg_path)
        with smtplib.SMTP(host="localhost", port=smtp_port) as smtp_session:
            for addr, is_valid in to_addrs:
                if not send_msg(smtp_session, addr, is_valid):
                    has_errors = True
    except Exception:
        kill_opensmtpd(pid_smtpd)
        raise
    kill_opensmtpd(pid_smtpd)
    if has_errors:
        fail("test failed")


def main():
    test_dir = pathlib.Path(__file__).parent.resolve()
    os.chdir(test_dir.parent)
    start_tests(test_dir, DEFAULT_PORT)


if __name__ == "__main__":
    main()
