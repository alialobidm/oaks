import { spawn } from 'child_process';
import { writeFileSync } from 'fs';

const output = spawn('cargo', ['check', '2>&1'], { 
  cwd: 'e:/普遍优化/oaks',
  shell: true,
  stdio: ['pipe', 'pipe', 'pipe']
});

let stdout = '';
let stderr = '';

output.stdout.on('data', (data) => {
  stdout += data.toString();
});

output.stderr.on('data', (data) => {
  stderr += data.toString();
});

output.on('close', (code) => {
  const fullOutput = stdout + '\n' + stderr;
  const lines = fullOutput.split('\n');
  
  const missingDocWarnings = [];
  let currentFile = null;
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    
    // Match file location pattern like: --> examples\oak-tcl\src\lexer\token_type.rs:40:5
    const fileMatch = line.match(/-->\s*([^\s:]+\.rs):(\d+):(\d+)/);
    if (fileMatch) {
      currentFile = fileMatch[1];
    }
    
    if (line.includes('missing documentation')) {
      missingDocWarnings.push({
        file: currentFile,
        message: line.trim()
      });
    }
  }
  
  // Group by file
  const byFile = {};
  for (const w of missingDocWarnings) {
    if (w.file) {
      if (!byFile[w.file]) {
        byFile[w.file] = [];
      }
      byFile[w.file].push(w.message);
    }
  }
  
  // Sort by file
  const sortedFiles = Object.keys(byFile).sort();
  
  let result = `Total: ${missingDocWarnings.length} missing documentation warnings\n\n`;
  for (const file of sortedFiles) {
    result += `${file}: ${byFile[file].length} warnings\n`;
  }
  
  result += '\n\n--- Detailed by file ---\n\n';
  
  for (const file of sortedFiles) {
    result += `\n=== ${file} (${byFile[file].length} warnings) ===\n`;
    for (const msg of byFile[file]) {
      result += `  ${msg}\n`;
    }
  }
  
  writeFileSync('e:/普遍优化/oaks/missing_docs_grouped.txt', result);
  console.log('Done. Found', missingDocWarnings.length, 'warnings in', sortedFiles.length, 'files');
});
