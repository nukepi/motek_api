import { executeCommand, withSpinner, printHeader } from './utils.js';
import { join } from 'path';

async function build() {
  printHeader('Building Motek UI project');

  await withSpinner('Building Rust library', async () => {
    const rustDir = join(process.cwd(), 'rust');
    await executeCommand('cargo', ['build'], { cwd: rustDir });
  });

  await withSpinner('Getting Flutter dependencies', async () => {
    await executeCommand('flutter', ['pub', 'get']);
  });

  console.log('\nProject built successfully! 🚀');
}

build().catch(err => {
  console.error('\nError while building project:', err);
  process.exit(1);
});
