#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

function getBinaryName() {
  return process.platform === 'win32' ? 'gwtr.exe' : 'gwtr';
}

function checkBinary() {
  const platform = process.platform;
  const arch = process.arch;
  const platformArch = `${platform}-${arch}`;
  
  // Check if the platform-specific package exists
  const packageName = `@gwtr/${platformArch}`;
  
  try {
    require.resolve(`${packageName}/package.json`);
    console.log(`✓ gwtr binary found for ${platformArch}`);
    return true;
  } catch (e) {
    // Binary not found
  }
  
  // Check for local binary (development)
  const localBinary = path.join(__dirname, '..', 'binaries', platformArch, getBinaryName());
  if (fs.existsSync(localBinary)) {
    console.log(`✓ gwtr binary found for ${platformArch} (local)`);
    return true;
  }
  
  return false;
}

// Only show warning if binary is not found
if (!checkBinary()) {
  const platform = process.platform;
  const arch = process.arch;
  
  console.warn(`
⚠️  gwtr binary not found for ${platform}-${arch}

This platform might not be supported yet. Supported platforms:
- darwin-x64 (macOS Intel)
- darwin-arm64 (macOS Apple Silicon)
- linux-x64 (Linux x64)
- linux-arm64 (Linux ARM64)
- win32-x64 (Windows x64)

If you believe this platform should be supported, please open an issue at:
https://github.com/katsuhirohonda/gwtr/issues
`);
}