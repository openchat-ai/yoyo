import subprocess

result = subprocess.run(
    [r'f:\yoyo\yoyo-rust\target\debug\yoyo.exe', 'test', 'golden'],
    cwd=r'f:\yoyo', capture_output=True, text=True, timeout=120,
    encoding='utf-8', errors='replace'
)

print('=== STDERR ===')
print(result.stderr)
print('=== STDOUT tail ===')
lines = (result.stdout or '').strip().split('\n')
print('\n'.join(lines[-5:]))

std = result.stdout or ''
err = result.stderr or ''
print(f'\nPASS={std.count("PASS")}')
print(f'FAIL in stdout={std.count("FAIL")}')
print(f'FAIL in stderr={err.count("FAIL")}')
