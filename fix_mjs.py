p = 'scripts/_probe/parallel-batch-97-run.mjs'
lines = open(p, encoding='utf-8').readlines()
bt = chr(96)
semi = chr(59)
nl = chr(10)
pipe = chr(124)
# Build: return `| H_${h} ... | `${r.pin}` ... `;
# Template literal, with escaped backtick \\` before ${r.pin} and after jsha
inner = (pipe + " H_${h} | 0x${sel} | ${r.opcode} | ${argsShort} | "
         "\\`" + "${r.pin}" + "\\` "
         + "(${r.len}B) | "
         "\\`" + "${r.jsha}" + "\\` |")
new = "  return " + bt + inner + bt + ";" + nl
lines[196] = new
open(p, 'w', encoding='utf-8', newline='\n').writelines(lines)
print('ok:', repr(lines[196]))
