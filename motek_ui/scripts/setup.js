import { executeCommand, withSpinner, printHeader } from './utils.js';
import { join } from 'path';

async function setup() {
  printHeader('Setting up Motek UI project');

  await withSpinner('Getting Flutter dependencies', async () => {
    await executeCommand('flutter', ['pub', 'get']);
  });

  await withSpinner('Building Rust library', async () => {
    const rustDir = join(process.cwd(), 'rust');
    await executeCommand('cargo', ['build'], { cwd: rustDir });
  });

  await withSpinner('Generating code with flutter_rust_bridge', async () => {
    await executeCommand('flutter_rust_bridge_codegen', ['generate']);
  });

  console.log('\nProject setup completed successfully! 🎉');
}

setup().catch(err => {
  console.error('\nError while setting up project:', err);
  process.exit(1);
});
